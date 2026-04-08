//! GraphQL SDL code rendering helpers.

use crate::mapper::GraphQLMapper;
use typewriter_core::ir::{EnumDef, EnumRepr, FieldDef, StructDef, VariantKind};
use typewriter_core::mapper::TypeMapper;

/// Render a Rust struct as a GraphQL `type` definition.
///
/// ```graphql
/// """
/// User profile information
/// """
/// type UserProfile {
///   id: ID!
///   email: String!
///   age: Int
/// }
/// ```
pub fn render_type(mapper: &GraphQLMapper, def: &StructDef) -> String {
    let mut out = String::new();

    // Doc comment
    if let Some(doc) = &def.doc {
        out.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", doc.trim()));
    }

    // Type declaration (GraphQL doesn't have generics, so we ignore them)
    out.push_str(&format!("type {} {{\n", def.name));

    for field in &def.fields {
        if field.skip {
            continue;
        }

        // Field doc comment
        if let Some(doc) = &field.doc {
            out.push_str(&format!("  \"\"\" {} \"\"\"\n", doc.trim()));
        }

        let field_name = field.rename.as_deref().unwrap_or(&field.name);
        let gql_type = render_field_type(mapper, field);

        out.push_str(&format!("  {}: {}\n", field_name, gql_type));
    }

    out.push_str("}\n");
    out
}

/// Render a Rust enum as GraphQL types.
///
/// - Simple (all-unit) enums → `enum { VARIANT1 VARIANT2 }`
/// - Data-carrying enums → `union = ...` + individual types per variant
pub fn render_enum(mapper: &GraphQLMapper, def: &EnumDef) -> String {
    let all_unit = def
        .variants
        .iter()
        .all(|v| matches!(v.kind, VariantKind::Unit));

    if all_unit {
        render_simple_enum(def)
    } else {
        render_data_enum(mapper, def)
    }
}

/// Render a simple (all-unit) enum as a GraphQL `enum`.
///
/// ```graphql
/// enum Role {
///   Admin
///   User
///   Guest
/// }
/// ```
fn render_simple_enum(def: &EnumDef) -> String {
    let mut out = String::new();

    if let Some(doc) = &def.doc {
        out.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", doc.trim()));
    }

    out.push_str(&format!("enum {} {{\n", def.name));

    for variant in &def.variants {
        if let Some(doc) = &variant.doc {
            out.push_str(&format!("  \"\"\" {} \"\"\"\n", doc.trim()));
        }
        let name = variant.rename.as_deref().unwrap_or(&variant.name);
        out.push_str(&format!("  {}\n", name));
    }

    out.push_str("}\n");
    out
}

/// Render a data-carrying enum.
///
/// Each variant with data becomes its own `type`, and a `union` ties them together.
///
/// ```graphql
/// type ShapeCircle {
///   radius: Float!
/// }
/// type ShapeRectangle {
///   width: Float!
///   height: Float!
/// }
/// union Shape = ShapeCircle | ShapeRectangle
/// ```
fn render_data_enum(mapper: &GraphQLMapper, def: &EnumDef) -> String {
    let mut out = String::new();

    if let Some(doc) = &def.doc {
        out.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", doc.trim()));
    }

    let mut variant_type_names: Vec<String> = Vec::new();

    for variant in &def.variants {
        let variant_type_name = format!("{}{}", def.name, &variant.name);

        match &variant.kind {
            VariantKind::Unit => {
                // Unit variants in a data-carrying enum become empty types
                if let Some(doc) = &variant.doc {
                    out.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", doc.trim()));
                }
                out.push_str(&format!("type {} {{\n", variant_type_name));

                // Render the discriminator field based on representation
                render_discriminator_fields(&def.representation, variant, &mut out);

                out.push_str("}\n\n");
            }
            VariantKind::Struct(fields) => {
                if let Some(doc) = &variant.doc {
                    out.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", doc.trim()));
                }
                out.push_str(&format!("type {} {{\n", variant_type_name));

                // Add discriminator field for internal/adjacent representations
                render_discriminator_fields(&def.representation, variant, &mut out);

                // Render data fields
                match &def.representation {
                    EnumRepr::Adjacent { content, .. } => {
                        // Adjacent wraps data in `content` as an embedded type —
                        // but in GraphQL we flatten the fields for usability
                        for field in fields {
                            if field.skip {
                                continue;
                            }
                            if let Some(doc) = &field.doc {
                                out.push_str(&format!("  \"\"\" {} \"\"\"\n", doc.trim()));
                            }
                            let fname = field.rename.as_deref().unwrap_or(&field.name);
                            let ftype = render_field_type(mapper, field);
                            out.push_str(&format!(
                                "  {}{}: {}\n",
                                content,
                                capitalize(fname),
                                ftype
                            ));
                        }
                    }
                    _ => {
                        for field in fields {
                            if field.skip {
                                continue;
                            }
                            if let Some(doc) = &field.doc {
                                out.push_str(&format!("  \"\"\" {} \"\"\"\n", doc.trim()));
                            }
                            let fname = field.rename.as_deref().unwrap_or(&field.name);
                            let ftype = render_field_type(mapper, field);
                            out.push_str(&format!("  {}: {}\n", fname, ftype));
                        }
                    }
                }

                out.push_str("}\n\n");
            }
            VariantKind::Tuple(types) => {
                if let Some(doc) = &variant.doc {
                    out.push_str(&format!("\"\"\"\n{}\n\"\"\"\n", doc.trim()));
                }
                out.push_str(&format!("type {} {{\n", variant_type_name));

                // Add discriminator field
                render_discriminator_fields(&def.representation, variant, &mut out);

                // Render tuple items as numbered fields
                for (i, ty) in types.iter().enumerate() {
                    let gql_type = mapper.map_type(ty);
                    out.push_str(&format!("  item{}: {}!\n", i, gql_type));
                }

                out.push_str("}\n\n");
            }
        }

        variant_type_names.push(variant_type_name);
    }

    // Union declaration
    out.push_str(&format!(
        "union {} = {}\n",
        def.name,
        variant_type_names.join(" | ")
    ));

    out
}

/// Render discriminator/tag fields based on enum representation.
fn render_discriminator_fields(
    repr: &EnumRepr,
    variant: &typewriter_core::ir::VariantDef,
    out: &mut String,
) {
    let serial_name = variant.rename.as_deref().unwrap_or(&variant.name);

    match repr {
        EnumRepr::Internal { tag } => {
            // `{ "type": "variant_name", ...data }`
            // The tag field is always a String! with a fixed value
            out.push_str(&format!(
                "  \"\"\" Discriminator: always \\\"{}\\\" \"\"\"\n  {}: String!\n",
                serial_name, tag
            ));
        }
        EnumRepr::Adjacent { tag, .. } => {
            // `{ "t": "variant_name", "c": { ...data } }`
            out.push_str(&format!(
                "  \"\"\" Discriminator: always \\\"{}\\\" \"\"\"\n  {}: String!\n",
                serial_name, tag
            ));
        }
        EnumRepr::External | EnumRepr::Untagged => {
            // No explicit discriminator field needed
        }
    }
}

/// Render a field's type with nullability.
///
/// In GraphQL:
/// - Required (non-optional) fields get `!` suffix → `String!`
/// - Optional fields omit the `!` → `String`
fn render_field_type(mapper: &GraphQLMapper, field: &FieldDef) -> String {
    if let Some(override_ty) = &field.type_override {
        if field.optional {
            return override_ty.clone();
        } else {
            return format!("{}!", override_ty);
        }
    }

    if field.optional {
        // Optional: return the inner type without `!`
        match &field.ty {
            typewriter_core::ir::TypeKind::Option(inner) => mapper.map_type(inner),
            other => mapper.map_type(other),
        }
    } else {
        // Required: append `!` for non-null
        format!("{}!", mapper.map_type(&field.ty))
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
