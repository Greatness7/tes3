use proc_macro::TokenStream;
use quote::quote;

#[allow(clippy::missing_const_for_fn)] // false positive
#[doc(hidden)]
#[proc_macro_attribute]
pub fn esp_meta(_args: TokenStream, input: TokenStream) -> TokenStream {
    input
}

#[allow(clippy::cognitive_complexity, clippy::too_many_lines)] // TODO
#[doc(hidden)]
#[proc_macro_derive(TES3Object, attributes(tag))]
pub fn derive_tes3object(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let variants = match &input.data {
        syn::Data::Enum(e) => &e.variants,
        _ => panic!("annotated item must be an enum"),
    };

    let idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    let tags: Vec<syn::LitByteStr> = variants.iter().map(|v| v.attrs[0].parse_args().unwrap()).collect();

    let tag_strs = tags
        .iter()
        .map(|tag| syn::LitStr::new(std::str::from_utf8(&tag.value()).unwrap(), tag.span()));

    let ident_strs = idents.iter().map(|id| syn::LitStr::new(&id.to_string(), id.span()));

    let output = quote! {
        const _: () = {
            use bytes_io::*;

            impl TES3Object {
                pub const fn tag(&self) -> &'static [u8; 4] {
                    match self {
                        #(
                            TES3Object::#idents(_) => #idents::TAG,
                        )*
                    }
                }
                pub const fn tag_str(&self) -> &'static str {
                    match self {
                        #(
                            TES3Object::#idents(_) => #idents::TAG_STR,
                        )*
                    }
                }
                pub fn flags(&self) -> &ObjectFlags {
                    match self {
                        #(
                            TES3Object::#idents(object) => &object.flags,
                        )*
                    }
                }
                pub fn flags_mut(&mut self) -> &mut ObjectFlags {
                    match self {
                        #(
                            TES3Object::#idents(object) => &mut object.flags,
                        )*
                    }
                }
            }

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

            #(
                impl #idents {
                    pub const TAG: &'static [u8; 4] = #tags;
                    pub const TAG_STR: &'static str = #tag_strs;
                    pub const fn tag(&self) -> &'static [u8; 4] { Self::TAG }
                    pub const fn tag_str(&self) -> &'static str { Self::TAG_STR }
                    pub const fn type_name(&self) -> &'static str { #ident_strs }
                }
                impl From<#idents> for TES3Object {
                    fn from(value: #idents) -> Self {
                        Self::#idents(value)
                    }
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
        };
    };

    output.into()
}
