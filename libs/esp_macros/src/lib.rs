use cfg_if::cfg_if;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};

#[doc(hidden)]
#[proc_macro_attribute]
pub fn esp_meta(_args: TokenStream, input: TokenStream) -> TokenStream {
    cfg_if! {
        if #[cfg(feature = "serde")] {
            serde_impls::impl_serialize_deserialize(input)
        } else {
            input
        }
    }
}

#[doc(hidden)]
#[proc_macro_derive(TES3Object, attributes(tag))]
pub fn derive_tes3object(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let variants = match &input.data {
        syn::Data::Enum(e) => &e.variants,
        _ => panic!("derive(TES3Object): item must be an enum"),
    };

    let idents = parse_variant_idents(variants);
    let tags = parse_variant_tags(variants);

    let impl_variants = tes3object_variant_impls(&idents, &tags);
    let impl_object = tes3object_inherent_impls(&idents);
    let impl_macros = tes3object_macros(&idents);

    let output = quote! {
        const _: () = {
            #impl_variants
            #impl_object
            #impl_macros
        };
    };

    output.into()
}

fn tes3object_variant_impls(idents: &[syn::Ident], tags: &[syn::LitStr]) -> impl ToTokens {
    let tags_bytes = tags //
        .iter()
        .map(|tag| syn::LitByteStr::new(tag.value().as_bytes(), tag.span()));

    let idents_str = idents //
        .iter()
        .map(|ident| syn::LitStr::new(&ident.to_string(), ident.span()));

    quote! {
        #(
            #[doc(hidden)]
            impl #idents {
                pub const TAG: &'static [u8; 4] = #tags_bytes;
                pub const TAG_STR: &'static str = #tags;
                pub const TYPE_NAME: &'static str = #idents_str;
            }

            impl TryFrom<TES3Object> for #idents {
                type Error = ();
                fn try_from(value: TES3Object) -> Result<Self, Self::Error> {
                    match value {
                        TES3Object::#idents(inner) => Ok(inner),
                        _ => Err(())
                    }
                }
            }

            impl<'a> TryFrom<&'a TES3Object> for &'a #idents {
                type Error = ();
                fn try_from(value: &'a TES3Object) -> Result<Self, Self::Error> {
                    match value {
                        TES3Object::#idents(inner) => Ok(inner),
                        _ => Err(())
                    }
                }
            }

            impl<'a> TryFrom<&'a mut TES3Object> for &'a mut #idents {
                type Error = ();
                fn try_from(value: &'a mut TES3Object) -> Result<Self, Self::Error> {
                    match value {
                        TES3Object::#idents(inner) => Ok(inner),
                        _ => Err(())
                    }
                }
            }
        )*
    }
}

fn tes3object_inherent_impls(idents: &[syn::Ident]) -> impl ToTokens {
    quote! {
        use bytes_io::*;

        impl Load for TES3Object {
            fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                let tag = stream.load()?;
                stream.skip(8)?; // skip size/padding

                match &tag {
                    #(
                        #idents::TAG => Ok(Self::#idents(stream.load()?)),
                    )*
                    _ => Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Unexpected Tag: {}", tag.to_str_lossy()),
                    ))
                }
            }
        }

        impl Save for TES3Object {
            fn save(&self, stream: &mut Writer) -> io::Result<()> {
                let start_pos = stream.cursor.position();

                // buffer for tag/size/padding
                stream.save(&[0u32; 3])?;

                // save object & get tag
                let tag = match self {
                    #(
                        TES3Object::#idents(obj) => { stream.save(obj)?; obj.tag() }
                    )*
                };

                // calculate object size
                let final_pos = stream.cursor.position();
                let size = (final_pos - start_pos - 16) as u32;

                // update the tag & size
                stream.cursor.set_position(start_pos);
                stream.save(tag)?;
                stream.save(&size)?;
                stream.cursor.set_position(final_pos);

                Ok(())
            }
        }
    }
}

fn tes3object_macros(idents: &[syn::Ident]) -> impl ToTokens {
    quote! {
        #[macro_export]
        macro_rules! delegate {
            (impl $name:path { $($body:item)* }) => {
                #(
                    impl $name for #idents {
                        $($body)*
                    }
                )*
            };
            (match $object:ident { $name:ident => $body:expr $(,)? }) => {
                match $object {
                    #(
                        TES3Object::#idents($name) => $body,
                    )*
                }
            };
        }
    }
}

fn parse_variant_tags<'a, I>(variants: I) -> Vec<syn::LitStr>
where
    I: IntoIterator<Item = &'a syn::Variant>,
{
    variants.into_iter().map(|v| v.attrs[0].parse_args().unwrap()).collect()
}

fn parse_variant_idents<'a, I>(variants: I) -> Vec<syn::Ident>
where
    I: IntoIterator<Item = &'a syn::Variant>,
{
    variants.into_iter().map(|v| v.ident.clone()).collect()
}

#[cfg(feature = "serde")]
mod serde_impls {
    use super::*;

    /// See: <https://serde.rs/enum-representations.html>
    ///
    /// We use "internally tagged" representations when possible.
    ///
    #[rustfmt::skip]
    fn get_serde_tag_attr(data: &syn::Data) -> impl ToTokens {
        // Only interested in enums.
        let syn::Data::Enum(e) = data else {
            return quote!();
        };

        // Internally tagged representation doesn't work on enums that have
        // variants which do not have any internal structure (no fields).
        // For those cases use an adjacently tagged representation.
        for variant in &e.variants {
            if let syn::Fields::Unnamed(fields) = &variant.fields {
                for field in &fields.unnamed {
                    if matches!(
                        quote!(#field).to_string().as_ref(),
                        "i8"  | "i16" | "i32" | "i64" | "i128" | "isize" |
                        "u8"  | "u16" | "u32" | "u64" | "u128" | "usize" |
                        "f32" | "f64" |
                        "String"
                    ) {
                        return quote! {
                            #[serde(tag = "type", content = "data")]
                        };
                    }
                }
            }
        }

        quote! {
            #[serde(tag = "type")]
        }
    }

    pub fn impl_serialize_deserialize(input: TokenStream) -> TokenStream {
        let input = syn::parse_macro_input!(input as syn::DeriveInput);
        let attrs = get_serde_tag_attr(&input.data);

        let output = quote! {
            #[derive(serde::Serialize, serde::Deserialize)]
            #attrs
            #input
        };

        output.into()
    }
}
