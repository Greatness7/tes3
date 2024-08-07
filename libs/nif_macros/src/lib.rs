use std::sync::{LazyLock, Mutex};

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DataStruct, DeriveInput, Fields, Ident, LitByteStr, Type};

mod util;
use util::*;

type LazyMap<K, V> = LazyLock<Mutex<HashMap<K, V>>>;

static RELATIONS: LazyMap<String, String> = LazyMap::new(Default::default);

/// Internal derive macro for use with `NiObject` structs in `nif.rs`.
#[doc(hidden)]
#[proc_macro_derive(Meta)]
pub fn derive_meta(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    // the ident of this annotated struct
    let self_id = &input.ident;

    // the ident of the base field struct
    let base_id = get_base_ident(&input.data);

    // the struct ident as a byte literal
    let self_id_bytes = get_literal_byte_str(self_id);

    // iter over the structs named fields
    let fields = get_struct_fields_rev(&input.data);

    // trait impls for inheritence system
    let inheritence_impls = impl_inheritence(self_id, base_id);

    let output = quote! {
        impl #self_id {
            #[doc(hidden)]
            pub const fn type_name(&self) -> &'static [u8] {
                #self_id_bytes
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
        #inheritence_impls
    };

    if let Some(ident) = base_id {
        RELATIONS.lock().unwrap().insert(self_id.to_string(), ident.to_string());
    }

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
        _ => panic!("derive(NiType): invalid input"),
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
                        _ => Reader::error(format!("Invalid Type: {}", type_name))?,
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
    // idents is an array of all the structs tagged with "Meta"
    // we need them in string form as well, so create a mapping
    let strings_to_idents: HashMap<String, &Ident> = idents.iter().map(|&ident| (ident.to_string(), ident)).collect();

    // build a map that pairs structs with their "base" structs
    let mut structs_to_bases = HashMap::<&Ident, Vec<&Ident>>::default();

    for (mut ident_string, ident) in &strings_to_idents {
        // we include the struct itself in its own bases vector
        structs_to_bases.entry(ident).or_default().push(ident);

        // RELATIONS maps all structs to their immediate "base"
        // use it to enable traversal of the struct heirarchies
        while let Some(r) = RELATIONS.lock().unwrap().get(ident_string) {
            // get the base struct's string and ident via our mapping
            let (base_string, base_ident) = strings_to_idents.get_key_value(r).unwrap();
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

fn impl_inheritence(self_id: &Ident, base_id: Option<&Ident>) -> impl ToTokens {
    let Some(base_id) = base_id else {
        return quote!();
    };
    quote! {
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
    }
}

/// Convert an Ident into a `LitByteStr`.
fn get_literal_byte_str(id: &Ident) -> LitByteStr {
    LitByteStr::new(id.to_string().as_bytes(), id.span())
}

/// Return the type ident of the first struct field if it is named "base".
fn get_base_ident(data: &Data) -> Option<&Ident> {
    if let Data::Struct(s) = data {
        if let Fields::Named(f) = &s.fields {
            let field = f.named.first()?;
            let ident = field.ident.as_ref()?;
            if let Type::Path(ty) = &field.ty {
                if ident == "base" {
                    return ty.path.get_ident();
                }
            }
        }
    }
    None
}
