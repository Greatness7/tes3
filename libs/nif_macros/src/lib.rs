use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, LitByteStr, Type};

use dashmap::DashMap;
use hashbrown::{hash_map::DefaultHashBuilder as S, HashMap};

// use [`std::sync::LazyLock`] when stable
use once_cell::sync::Lazy as LazyLock;

type LazyMap<K, V> = LazyLock<DashMap<K, V, S>>;

static RELATIONS: LazyMap<String, String> = LazyMap::new(DashMap::default);

/// Internal derive macro for use with `NiObject` structs in `nif.rs`.
#[doc(hidden)]
#[proc_macro_derive(Meta)]
pub fn derive_meta(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    // the ident of this annotated struct
    let self_id = &input.ident;

    // the ident of the base field struct
    let base_id = get_base_ident(&input.data).clone();

    // the struct ident as a byte literal
    let self_id_bytes = get_literal_byte_str(self_id);

    // iter over the structs named fields
    let fields = get_struct_fields_rev(&input.data);

    let output = quote! {
        impl #self_id {
            #[doc(hidden)]
            pub const fn type_name(&self) -> &'static [u8] {
                #self_id_bytes
            }
        }
        impl ::std::ops::Deref for #self_id {
            type Target = #base_id;
            fn deref(&self) -> &Self::Target {
                &self.base
            }
        }
        impl ::std::ops::DerefMut for #self_id {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.base
            }
        }
        impl Visitor for #self_id {
            #[inline(always)]
            fn visitor<'a, F>(&self, f: &mut F)
            where
                F: FnMut(NiKey)
            {
                #(
                    (&self.#fields).visitor(f);
                )*
            }
        }
    };

    // ...
    RELATIONS.insert(self_id.to_string(), base_id.to_string());

    output.into()
}

#[rustfmt::skip]
fn get_struct_fields_rev(data: &Data) -> impl Iterator<Item = &Ident> {
    let fields = match data {
        Data::Struct(DataStruct { fields: Fields::Named(f), .. }) => {
            Some(f.named.iter().filter_map(|f| f.ident.as_ref()))
        },
        _ => None
    };
    fields.into_iter().flatten().rev()
}

/// Internal derive macro for use with the `NiType` enum in `nif.rs`.
#[doc(hidden)]
#[proc_macro_derive(NiType)]
pub fn derive_nitype(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let variants = match &input.data {
        Data::Enum(e) => &e.variants,
        _ => panic!("derive(NiTypeDerive): invalid input"),
    };

    // the idents of all variants
    let idents: Vec<_> = variants.iter().map(|v| &v.ident).collect();

    // the idents as byte literals
    let idents_bytes = idents.iter().map(|id| get_literal_byte_str(id));

    let impl_try_from = impl_try_from_nitype(&idents);

    let output = quote! {
        const _: () = {
            use crate::prelude::*;
            use io::{Read, Write};

            impl Load for NiType {
                fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
                    let type_name: ::bstr::BString = stream.load()?;
                    match type_name.as_slice() {
                        #(
                            #idents_bytes => Ok(Self::#idents(stream.load()?)),
                        )*
                        // TODO: more detailed error information!
                        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "invalid data")),
                    }
                }
            }
            impl Save for NiType {
                fn save(&self, stream: &mut Writer) -> io::Result<()> {
                    match self {
                        #(
                            Self::#idents(inner) => {
                                let type_name = inner.type_name();
                                let len = type_name.len() as u32;
                                stream.save(&len)?;
                                stream.write_all(&type_name)?;
                                stream.save(inner)?;
                            }
                        )*
                    }
                    Ok(())
                }
            }
            impl Visitor for NiType {
                #[inline(always)]
                fn visitor<'a, F>(&self, f: &mut F)
                where
                    F: FnMut(NiKey)
                {
                    match self {
                        #(
                            Self::#idents(inner) => inner.visitor(f),
                        )*
                    }
                }
            }
        };
        #impl_try_from
    };

    output.into()
}

fn impl_try_from_nitype(idents: &[&Ident]) -> impl ToTokens {
    // idents is an array of all the structs tagged with NiMeta
    // we need them in string form as well, so create a mapping
    let strings_to_idents: HashMap<String, &Ident> = idents.iter().map(|ident| (ident.to_string(), *ident)).collect();

    // build a map that pairs structs with their "base" structs
    let mut structs_to_bases = HashMap::<&Ident, Vec<&Ident>>::with_capacity(idents.len());

    for (mut ident_string, ident) in strings_to_idents.iter() {
        // we include the struct itself in its own bases vector
        structs_to_bases.entry(ident).or_default().push(ident);

        // RELATIONS maps all structs to their immediate "base"
        // use it to enable traversal of the struct heirarchies
        while let Some(r) = RELATIONS.get(ident_string) {
            // get the base struct's string and ident via our mapping
            let (base_string, base_ident) = strings_to_idents.get_key_value(r.value()).unwrap();
            // add the base ident into our struct's sub-struct vector
            structs_to_bases.entry(base_ident).or_default().push(ident);
            ident_string = base_string;
        }
    }

    let mut output = quote! {
        impl NiType {
            pub const fn type_name(&self) -> &'static [u8] {
                match self {
                    #(
                        NiType::#idents(inner) => inner.type_name(),
                    )*
                }
            }
        }
        #(
            impl TryFrom<NiType> for #idents {
                type Error = ();
                fn try_from(value: NiType) -> Result<Self, Self::Error> {
                    if let NiType::#idents(o) = value {
                        Ok(o)
                    } else {
                        Err(())
                    }
                }
            }
        )*
    };

    for (ident, child_idents) in structs_to_bases {
        output = quote! {
            #output
            impl<'a> TryFrom<&'a NiType> for &'a #ident {
                type Error = ();
                fn try_from(value: &'a NiType) -> Result<Self, Self::Error> {
                    match value {
                        #(
                            NiType::#child_idents(inner) => Ok(inner),
                        )*
                        _ => Err(())
                    }
                }
            }
            impl<'a> TryFrom<&'a mut NiType> for &'a mut #ident {
                type Error = ();
                fn try_from(value: &'a mut NiType) -> Result<Self, Self::Error> {
                    match value {
                        #(
                            NiType::#child_idents(inner) => Ok(inner),
                        )*
                    _ => Err(())
                    }
                }
            }
            #(
                impl AsRef<#ident> for #child_idents {
                    fn as_ref(&self) -> &#ident {
                        &self
                    }
                }
            )*
        };
    }

    output
}

/// Convert an Ident into a `LitByteStr`.
fn get_literal_byte_str(id: &Ident) -> LitByteStr {
    LitByteStr::new(id.to_string().as_bytes(), id.span())
}

/// Return the type ident of the first struct field if it is named "base".
fn get_base_ident(data: &Data) -> &Ident {
    if let Data::Struct(s) = data {
        if let Fields::Named(f) = &s.fields {
            if let Some(first) = f.named.first() {
                if first.ident.as_ref().unwrap() == "base" {
                    if let Type::Path(ty) = &first.ty {
                        return ty.path.get_ident().unwrap();
                    }
                }
            }
        }
    }
    panic!("Could not find `base` field.")
}
