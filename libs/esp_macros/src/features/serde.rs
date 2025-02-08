/// Implement Serialize/Deserialize for input.
///
pub fn impl_serialize_deserialize(input: &mut syn::DeriveInput) {
    input.attrs.push(syn::parse_quote! {
        #[derive(serde::Serialize, serde::Deserialize)]
    });
    match &mut input.data {
        syn::Data::Struct(data_struct) => insert_struct_attrs(&mut input.attrs, data_struct),
        syn::Data::Enum(data_enum) => insert_enum_attrs(&mut input.attrs, data_enum),
        syn::Data::Union(_) => unimplemented!(),
    }
}

fn insert_struct_attrs(attrs: &mut Vec<syn::Attribute>, data: &mut syn::DataStruct) {
    // Add serde(transparent) tag.
    if is_repr_transparent(attrs) {
        attrs.push(syn::parse_quote! {
            #[serde(transparent)]
        });
    }
    // Insert per-field attributes.
    for field in &mut data.fields {
        insert_struct_field_attrs(field);
    }
}

/// <https://serde.rs/enum-representations.html>
///
fn insert_enum_attrs(attrs: &mut Vec<syn::Attribute>, data: &syn::DataEnum) {
    // Use default representation for "C-like" enums.
    if data.variants.iter().all(|variant| variant.discriminant.is_some()) {
        return;
    }
    // Use "adjacently tagged" representation for enums with numeric primitive variants.
    for variant in &data.variants {
        if let syn::Fields::Unnamed(fields) = &variant.fields {
            for field in &fields.unnamed {
                if field.is_numeric_primitive() {
                    attrs.push(syn::parse_quote! {
                        #[serde(tag = "type", content = "data")]
                    });
                    return;
                }
            }
        }
    }
    // Use "internally tagged" representation for everything else.
    attrs.push(syn::parse_quote! {
        #[serde(tag = "type")]
    });
}

fn insert_struct_field_attrs(field: &mut syn::Field) {
    // Extract the type's final path segment.
    // e.g. the 'Box' from 'std::boxed::Box'.
    let segment = match &field.ty {
        syn::Type::Path(ty) => ty.path.segments.last().unwrap(),
        _ => return,
    };

    let ident = segment.ident.to_string();

    // Skip serializing `None` for smaller jsons.
    if ident == "Option" {
        field.attrs.push(syn::parse_quote! {
            #[serde(skip_serializing_if = "Option::is_none")]
        });
    }

    // Otherwise we only care about Box/Vec types.
    if ident != "Box" && ident != "Vec" {
        return;
    }

    // And only if consists of numeric primitives.
    if !segment.arguments.is_numeric_primitive() {
        return;
    }

    // Serialize/deserialize these as base64 bytes.
    field.attrs.push(syn::parse_quote! {
        #[serde(with = "crate::features::serde::base64_bytes")]
    });
}

fn is_repr_transparent(attributes: &[syn::Attribute]) -> bool {
    for attr in attributes {
        if let Some(outer_ident) = attr.path().get_ident() {
            if let Ok(inner_ident) = attr.parse_args::<syn::Ident>() {
                if outer_ident == "repr" && inner_ident == "transparent" {
                    return true;
                }
            }
        }
    }
    false
}

/// Utility trait for checking if a syntax element is a numeric primitive.
trait IsNumericPrimitive {
    fn is_numeric_primitive(&self) -> bool;
}
const _: () = {
    use syn::visit::Visit;

    struct Visitor(bool);

    impl Visit<'_> for Visitor {
        #[rustfmt::skip]
        fn visit_ident(&mut self, ident: &syn::Ident) {
            self.0 &= matches!(
                ident.to_string().as_str(),
                "i8"  | "i16" | "i32" | "i64" | "i128" | "isize" |
                "u8"  | "u16" | "u32" | "u64" | "u128" | "usize" |
                "f32" | "f64"
            );
        }
    }

    impl IsNumericPrimitive for syn::PathArguments {
        fn is_numeric_primitive(&self) -> bool {
            let mut visitor = Visitor(true);
            visitor.visit_path_arguments(self);
            visitor.0
        }
    }

    impl IsNumericPrimitive for syn::Field {
        fn is_numeric_primitive(&self) -> bool {
            let mut visitor = Visitor(true);
            visitor.visit_field(self);
            visitor.0
        }
    }
};
