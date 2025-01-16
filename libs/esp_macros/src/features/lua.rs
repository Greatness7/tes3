use proc_macro::TokenStream;
use quote::quote;

pub fn struct_impl(ident: &syn::Ident, data: &syn::DataStruct) -> TokenStream {
    let fields: Vec<_> = get_struct_fields(&data).collect();
    if fields.is_empty() {
        impl_lua_bindings_for_bitflags(ident)
    } else {
        impl_lua_bindings_for_struct_with_named_fields(ident, data)
    }
}

pub fn enum_impl(ident: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
    let variants = &data.variants;
    let has_discriminants = variants.iter().all(|v| v.discriminant.is_some());
    if has_discriminants {
        impl_lua_bindings_for_enum_with_discriminants(ident, data).into()
    } else {
        impl_lua_bindings_for_enum_with_data_variants(ident, data).into()
    }
}

pub fn impl_lua_bindings_for_enum_with_discriminants(ident: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
    let variants: Vec<_> = data.variants.iter().map(|v| &v.ident).collect();
    let variants_str = variants.iter().map(|id| into_literal_str(id));

    quote! {
        const _: () = {
            use crate::features::lua::*;

            impl LuaPrimitive for #ident {
                fn clone_into_lua(&self, lua: &Lua) -> LuaResult<LuaValue> {
                    match self {
                        #(
                            Self::#variants => #variants_str,
                        )*
                    }
                    .into_lua(lua)
                }
            }
        };
    }
    .into()
}

pub fn impl_lua_bindings_for_enum_with_data_variants(ident: &syn::Ident, data: &syn::DataEnum) -> TokenStream {
    let variants = crate::parse_variant_idents(&data.variants);

    quote! {
        const _: () = {
            use crate::features::lua::*;

            impl Getter for #ident {
                impl_getter!();
            }

            impl Getter for Vec<#ident> {
                impl_getter!();
            }

            impl UserData for Ref<&'static Vec<#ident>> {
                fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
                    fields.add_field("type", Self::type_name());
                }
                fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
                    impl_meta_method!(methods, "__tostring");
                    impl_meta_method!(methods, "__len");
                    impl_meta_method!(methods, "__index");
                    impl_meta_method!(methods, "__ipairs");
                }
            }

            impl IntoLua for Ref<&'static #ident> {
                fn into_lua(self, lua: &Lua) -> LuaResult<LuaValue> {
                    match self.get() {
                        #(
                            #ident::#variants(_) => {
                                self.map::<&_>(|this, _| {
                                    let #ident::#variants(inner) = this else { unsafe { std::hint::unreachable_unchecked() } };
                                    inner
                                })
                                .into_lua(lua)
                            }
                        )*
                    }
                }
            }
        };
    }
    .into()
}

pub fn impl_lua_bindings_for_struct_with_named_fields(ident: &syn::Ident, data: &syn::DataStruct) -> TokenStream {
    let field_idents: Vec<_> = data.fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_names = field_idents.iter().map(|id| into_literal_str(id));

    let ident_str = into_literal_str(ident);

    quote! {
        const _: () = {
            use crate::features::lua::*;

            impl Getter for #ident {
                impl_getter!();
            }

            impl<const N: usize> Getter for [#ident; N] {
                impl_getter!();
            }

            impl Getter for Vec<#ident>
            where
                #ident: 'static,
                Ref<&'static #ident>: UserData,
            {
                impl_getter!();
            }

            impl UserData for Ref<&'static #ident> {
                fn add_fields<F: UserDataFields<Self>>(fields: &mut F) {
                    fields.add_field("type", #ident_str);
                    #(
                        let #field_idents = |this: &Self| this.map(|inner, _| &inner.#field_idents);
                    )*
                    #(
                        getter(fields, #field_names, #field_idents);
                    )*
                }

                fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
                    impl_meta_method!(methods, "__tostring");
                }
            }


        };
    }
    .into()
}

pub fn impl_lua_bindings_for_bitflags(ident: &syn::Ident) -> TokenStream {
    quote! {
        const _: () = {
            use crate::features::lua::*;
            impl Getter for #ident {
                fn getter<S>(
                    fields: &mut impl UserDataFields<S>,
                    name: impl ToString,
                    get: impl Get<S, Self>,
                ) {
                    fields.add_field_method_get(name, move |lua, this| Ok(get(this).bits()))
                }
            }
        };
    }
    .into()
}

pub fn get_struct_fields(data: &syn::DataStruct) -> impl Iterator<Item = &syn::Ident> {
    if let syn::Fields::Named(f) = &data.fields {
        Some(f)
    } else {
        None
    }
    .into_iter()
    .flat_map(|f| f.named.iter().filter_map(|f| f.ident.as_ref()))
}

pub fn into_literal_str(id: &syn::Ident) -> syn::LitStr {
    syn::LitStr::new(&id.to_string(), id.span())
}
