//! Parser: converts Rust AST (`syn`) into typewriter IR (`TypeDef`).

use syn::{Attribute, Data, DeriveInput, Expr, ExprLit, Fields, Lit, Meta, Type};
use typewriter_core::ir::*;

/// Parse a `syn::DeriveInput` into a `TypeDef`.
pub fn parse_type_def(input: &DeriveInput) -> syn::Result<TypeDef> {
    let name = input.ident.to_string();
    let doc = extract_doc_comment(&input.attrs);
    let generics: Vec<String> = input
        .generics
        .type_params()
        .map(|p| p.ident.to_string())
        .collect();

    match &input.data {
        Data::Struct(data) => {
            let fields = parse_fields(&data.fields)?;
            Ok(TypeDef::Struct(StructDef {
                name,
                fields,
                doc,
                generics,
            }))
        }
        Data::Enum(data) => {
            let repr = parse_enum_repr(&input.attrs);
            let variants = data
                .variants
                .iter()
                .map(parse_variant)
                .collect::<syn::Result<Vec<_>>>()?;

            Ok(TypeDef::Enum(EnumDef {
                name,
                variants,
                representation: repr,
                doc,
            }))
        }
        Data::Union(_) => Err(syn::Error::new_spanned(
            &input.ident,
            "typewriter: unions are not supported. Use structs or enums.",
        )),
    }
}

/// Parse the `#[sync_to(typescript, python, ...)]` attribute.
pub fn parse_sync_to_attr(input: &DeriveInput) -> syn::Result<Vec<Language>> {
    let mut targets = Vec::new();

    for attr in &input.attrs {
        if !attr.path().is_ident("sync_to") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if let Some(ident) = meta.path.get_ident() {
                let lang_str = ident.to_string();
                if let Some(language) = Language::from_str(&lang_str) {
                    targets.push(language);
                } else {
                    return Err(meta.error(format!(
                        "typewriter: unknown language '{}'. Supported: typescript, python, go, swift, kotlin",
                        lang_str
                    )));
                }
            }
            Ok(())
        })?;
    }

    Ok(targets)
}

/// Detect if item attributes include `#[derive(... TypeWriter ...)]`.
pub fn has_typewriter_derive(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident("derive") {
            continue;
        }

        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta
                .path
                .segments
                .last()
                .map(|s| s.ident == "TypeWriter")
                .unwrap_or(false)
            {
                found = true;
            }
            Ok(())
        });

        if found {
            return true;
        }
    }

    false
}

/// Parse struct/variant fields into `Vec<FieldDef>`.
fn parse_fields(fields: &Fields) -> syn::Result<Vec<FieldDef>> {
    match fields {
        Fields::Named(named) => named
            .named
            .iter()
            .map(|f| {
                let name = f.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
                let ty = parse_type(&f.ty);
                let optional =
                    matches!(&ty, TypeKind::Option(_)) || has_tw_attr(&f.attrs, "optional");
                let rename = get_rename(&f.attrs);
                let skip = has_serde_skip(&f.attrs) || has_tw_attr(&f.attrs, "skip");
                let flatten = has_serde_flatten(&f.attrs);
                let doc = extract_doc_comment(&f.attrs);
                let type_override = get_tw_type_override(&f.attrs);

                Ok(FieldDef {
                    name,
                    ty,
                    optional,
                    rename,
                    doc,
                    skip,
                    flatten,
                    type_override,
                })
            })
            .collect(),
        Fields::Unnamed(_) | Fields::Unit => Ok(vec![]),
    }
}

/// Parse a `syn::Type` into a `TypeKind`.
fn parse_type(ty: &Type) -> TypeKind {
    match ty {
        Type::Path(type_path) => {
            let path = &type_path.path;

            if let Some(segment) = path.segments.last() {
                let ident = segment.ident.to_string();

                match ident.as_str() {
                    "Option" => {
                        if let Some(inner) = extract_single_generic_arg(segment) {
                            return TypeKind::Option(Box::new(parse_type(&inner)));
                        }
                    }
                    "Vec" => {
                        if let Some(inner) = extract_single_generic_arg(segment) {
                            return TypeKind::Vec(Box::new(parse_type(&inner)));
                        }
                    }
                    "HashMap" | "BTreeMap" => {
                        if let Some((k, v)) = extract_double_generic_arg(segment) {
                            return TypeKind::HashMap(
                                Box::new(parse_type(&k)),
                                Box::new(parse_type(&v)),
                            );
                        }
                    }
                    "Box" | "Arc" | "Rc" => {
                        if let Some(inner) = extract_single_generic_arg(segment) {
                            return parse_type(&inner);
                        }
                    }
                    _ => {}
                }

                if let Some(prim) = map_primitive_name(&ident) {
                    return TypeKind::Primitive(prim);
                }

                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    let type_args: Vec<TypeKind> = args
                        .args
                        .iter()
                        .filter_map(|arg| {
                            if let syn::GenericArgument::Type(ty) = arg {
                                Some(parse_type(ty))
                            } else {
                                None
                            }
                        })
                        .collect();

                    if !type_args.is_empty() {
                        return TypeKind::Generic(ident, type_args);
                    }
                }

                TypeKind::Named(ident)
            } else {
                TypeKind::Unit
            }
        }
        Type::Tuple(tuple) => {
            if tuple.elems.is_empty() {
                TypeKind::Unit
            } else {
                let elements: Vec<TypeKind> = tuple.elems.iter().map(parse_type).collect();
                TypeKind::Tuple(elements)
            }
        }
        Type::Reference(reference) => parse_type(&reference.elem),
        _ => TypeKind::Unit,
    }
}

fn map_primitive_name(name: &str) -> Option<PrimitiveType> {
    match name {
        "String" | "str" => Some(PrimitiveType::String),
        "bool" => Some(PrimitiveType::Bool),
        "u8" => Some(PrimitiveType::U8),
        "u16" => Some(PrimitiveType::U16),
        "u32" => Some(PrimitiveType::U32),
        "u64" => Some(PrimitiveType::U64),
        "u128" => Some(PrimitiveType::U128),
        "i8" => Some(PrimitiveType::I8),
        "i16" => Some(PrimitiveType::I16),
        "i32" => Some(PrimitiveType::I32),
        "i64" => Some(PrimitiveType::I64),
        "i128" => Some(PrimitiveType::I128),
        "f32" => Some(PrimitiveType::F32),
        "f64" => Some(PrimitiveType::F64),
        "Uuid" => Some(PrimitiveType::Uuid),
        "DateTime" => Some(PrimitiveType::DateTime),
        "NaiveDate" => Some(PrimitiveType::NaiveDate),
        "Value" => Some(PrimitiveType::JsonValue),
        _ => None,
    }
}

fn extract_single_generic_arg(segment: &syn::PathSegment) -> Option<Type> {
    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
            return Some(ty.clone());
        }
    }
    None
}

fn extract_double_generic_arg(segment: &syn::PathSegment) -> Option<(Type, Type)> {
    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        let mut iter = args.args.iter();
        if let (Some(syn::GenericArgument::Type(k)), Some(syn::GenericArgument::Type(v))) =
            (iter.next(), iter.next())
        {
            return Some((k.clone(), v.clone()));
        }
    }
    None
}

fn parse_variant(variant: &syn::Variant) -> syn::Result<VariantDef> {
    let name = variant.ident.to_string();
    let rename = get_rename(&variant.attrs);
    let doc = extract_doc_comment(&variant.attrs);

    let kind = match &variant.fields {
        Fields::Unit => VariantKind::Unit,
        Fields::Named(named) => {
            let fields = named
                .named
                .iter()
                .map(|f| {
                    let fname = f.ident.as_ref().map(|i| i.to_string()).unwrap_or_default();
                    let ty = parse_type(&f.ty);
                    let optional = matches!(&ty, TypeKind::Option(_));
                    let field_rename = get_rename(&f.attrs);
                    let skip = has_serde_skip(&f.attrs) || has_tw_attr(&f.attrs, "skip");
                    let fdoc = extract_doc_comment(&f.attrs);
                    let type_override = get_tw_type_override(&f.attrs);

                    FieldDef {
                        name: fname,
                        ty,
                        optional,
                        rename: field_rename,
                        doc: fdoc,
                        skip,
                        flatten: false,
                        type_override,
                    }
                })
                .collect();
            VariantKind::Struct(fields)
        }
        Fields::Unnamed(unnamed) => {
            let types: Vec<TypeKind> = unnamed.unnamed.iter().map(|f| parse_type(&f.ty)).collect();
            VariantKind::Tuple(types)
        }
    };

    Ok(VariantDef {
        name,
        rename,
        kind,
        doc,
    })
}

fn parse_enum_repr(attrs: &[Attribute]) -> EnumRepr {
    let mut tag = None;
    let mut content = None;
    let mut untagged = false;

    for attr in attrs {
        if !attr.path().is_ident("serde") {
            continue;
        }

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("tag") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                tag = Some(s.value());
            } else if meta.path.is_ident("content") {
                let value = meta.value()?;
                let s: syn::LitStr = value.parse()?;
                content = Some(s.value());
            } else if meta.path.is_ident("untagged") {
                untagged = true;
            } else if meta.path.is_ident("rename_all") {
                let value = meta.value()?;
                let _s: syn::LitStr = value.parse()?;
            }
            Ok(())
        });
    }

    if untagged {
        return EnumRepr::Untagged;
    }

    match (tag, content) {
        (Some(t), Some(c)) => EnumRepr::Adjacent { tag: t, content: c },
        (Some(t), None) => EnumRepr::Internal { tag: t },
        _ => EnumRepr::External,
    }
}

fn get_rename(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("tw") {
            let mut rename_val = None;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    rename_val = Some(s.value());
                }
                Ok(())
            });
            if rename_val.is_some() {
                return rename_val;
            }
        }
    }

    for attr in attrs {
        if attr.path().is_ident("serde") {
            let mut rename_val = None;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("rename") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    rename_val = Some(s.value());
                }
                Ok(())
            });
            if rename_val.is_some() {
                return rename_val;
            }
        }
    }

    None
}

fn has_serde_skip(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") || meta.path.is_ident("skip_serializing") {
                    found = true;
                }
                Ok(())
            });
            if found {
                return true;
            }
        }
    }
    false
}

fn has_serde_flatten(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if attr.path().is_ident("serde") {
            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("flatten") {
                    found = true;
                }
                Ok(())
            });
            if found {
                return true;
            }
        }
    }
    false
}

fn has_tw_attr(attrs: &[Attribute], attr_name: &str) -> bool {
    for attr in attrs {
        if attr.path().is_ident("tw") {
            let mut found = false;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident(attr_name) {
                    found = true;
                }
                Ok(())
            });
            if found {
                return true;
            }
        }
    }
    false
}

fn get_tw_type_override(attrs: &[Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.path().is_ident("tw") {
            let mut type_val = None;
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("type") {
                    let value = meta.value()?;
                    let s: syn::LitStr = value.parse()?;
                    type_val = Some(s.value());
                }
                Ok(())
            });
            if type_val.is_some() {
                return type_val;
            }
        }
    }
    None
}

fn extract_doc_comment(attrs: &[Attribute]) -> Option<String> {
    let docs: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let Meta::NameValue(nv) = &attr.meta {
                    if let Expr::Lit(ExprLit {
                        lit: Lit::Str(s), ..
                    }) = &nv.value
                    {
                        return Some(s.value().trim().to_string());
                    }
                }
            }
            None
        })
        .collect();

    if docs.is_empty() {
        None
    } else {
        Some(docs.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_typewriter_derive() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[derive(Debug, TypeWriter)]
            #[sync_to(typescript)]
            struct User { id: String }
        };

        assert!(has_typewriter_derive(&input.attrs));
    }

    #[test]
    fn parses_sync_targets() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[derive(TypeWriter)]
            #[sync_to(typescript, python)]
            struct User { id: String }
        };

        let targets = parse_sync_to_attr(&input).unwrap();
        assert_eq!(targets, vec![Language::TypeScript, Language::Python]);
    }
}
