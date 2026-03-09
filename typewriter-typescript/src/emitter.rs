//! TypeScript code rendering helpers.

use typewriter_core::ir::*;
use typewriter_core::mapper::TypeMapper;

use crate::mapper::TypeScriptMapper;

/// Render a Rust struct as a TypeScript `export interface`.
pub fn render_interface(mapper: &TypeScriptMapper, def: &StructDef) -> String {
    let mut output = String::new();

    // Doc comment
    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    output.push_str(&format!("export interface {} {{\n", def.name));

    for field in &def.fields {
        if field.skip {
            continue;
        }

        // Field doc comment
        if let Some(doc) = &field.doc {
            output.push_str(&format!("  /** {} */\n", doc.trim()));
        }

        let field_name = field.rename.as_deref().unwrap_or(&field.name);
        let type_str = mapper.map_type(&field.ty);

        let readonly_prefix = if mapper.readonly { "readonly " } else { "" };

        if field.optional {
            output.push_str(&format!(
                "  {}{}?: {};\n",
                readonly_prefix, field_name, type_str
            ));
        } else {
            output.push_str(&format!(
                "  {}{}: {};\n",
                readonly_prefix, field_name, type_str
            ));
        }
    }

    output.push_str("}\n");
    output
}

/// Render a Rust enum as TypeScript types.
///
/// - Simple (all-unit) enums → string literal union: `type Role = "Admin" | "User" | "Guest"`
/// - Data-carrying enums → discriminated union types
pub fn render_enum(mapper: &TypeScriptMapper, def: &EnumDef) -> String {
    let all_unit = def
        .variants
        .iter()
        .all(|v| matches!(v.kind, VariantKind::Unit));

    if all_unit {
        render_string_enum(def)
    } else {
        render_discriminated_union(mapper, def)
    }
}

/// Render a simple enum as a string literal union.
///
/// ```typescript
/// export type Role = "Admin" | "User" | "Guest";
/// ```
fn render_string_enum(def: &EnumDef) -> String {
    let mut output = String::new();

    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    let variants: Vec<String> = def
        .variants
        .iter()
        .map(|v| {
            let name = v.rename.as_deref().unwrap_or(&v.name);
            format!("\"{}\"", name)
        })
        .collect();

    output.push_str(&format!(
        "export type {} =\n  | {};\n",
        def.name,
        variants.join("\n  | ")
    ));

    output
}

/// Render a data-carrying enum as a discriminated union.
///
/// For internally tagged enums (`#[serde(tag = "type")]`):
/// ```typescript
/// export type PaymentStatus =
///   | { type: "pending" }
///   | { type: "completed"; transaction_id: string }
///   | { type: "failed"; reason: string; code: number };
/// ```
fn render_discriminated_union(mapper: &TypeScriptMapper, def: &EnumDef) -> String {
    let mut output = String::new();

    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    let tag_field = match &def.representation {
        EnumRepr::Internal { tag } => tag.as_str(),
        EnumRepr::Adjacent { tag, .. } => tag.as_str(),
        _ => "type",
    };

    let variants: Vec<String> = def
        .variants
        .iter()
        .map(|v| {
            let variant_name = v.rename.as_deref().unwrap_or(&v.name);

            match &v.kind {
                VariantKind::Unit => {
                    format!("{{ {}: \"{}\" }}", tag_field, variant_name)
                }
                VariantKind::Struct(fields) => {
                    let field_strs: Vec<String> = fields
                        .iter()
                        .filter(|f| !f.skip)
                        .map(|f| {
                            let fname = f.rename.as_deref().unwrap_or(&f.name);
                            let ftype = mapper.map_type(&f.ty);
                            if f.optional {
                                format!("{}?: {}", fname, ftype)
                            } else {
                                format!("{}: {}", fname, ftype)
                            }
                        })
                        .collect();

                    match &def.representation {
                        EnumRepr::Adjacent { tag, content } => {
                            format!(
                                "{{ {}: \"{}\"; {}: {{ {} }} }}",
                                tag,
                                variant_name,
                                content,
                                field_strs.join("; ")
                            )
                        }
                        _ => {
                            format!(
                                "{{ {}: \"{}\"; {} }}",
                                tag_field,
                                variant_name,
                                field_strs.join("; ")
                            )
                        }
                    }
                }
                VariantKind::Tuple(types) => {
                    let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();

                    match &def.representation {
                        EnumRepr::Adjacent { tag, content } => {
                            if types.len() == 1 {
                                format!(
                                    "{{ {}: \"{}\"; {}: {} }}",
                                    tag, variant_name, content, type_strs[0]
                                )
                            } else {
                                format!(
                                    "{{ {}: \"{}\"; {}: [{}] }}",
                                    tag,
                                    variant_name,
                                    content,
                                    type_strs.join(", ")
                                )
                            }
                        }
                        _ => {
                            if types.len() == 1 {
                                format!(
                                    "{{ {}: \"{}\"; value: {} }}",
                                    tag_field, variant_name, type_strs[0]
                                )
                            } else {
                                format!(
                                    "{{ {}: \"{}\"; value: [{}] }}",
                                    tag_field,
                                    variant_name,
                                    type_strs.join(", ")
                                )
                            }
                        }
                    }
                }
            }
        })
        .collect();

    output.push_str(&format!(
        "export type {} =\n  | {};\n",
        def.name,
        variants.join("\n  | ")
    ));

    output
}
