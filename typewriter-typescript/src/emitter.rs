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

    // Interface declaration with optional generic params
    if def.generics.is_empty() {
        output.push_str(&format!("export interface {} {{\n", def.name));
    } else {
        output.push_str(&format!(
            "export interface {}<{}> {{\n",
            def.name,
            def.generics.join(", ")
        ));
    }

    for field in &def.fields {
        if field.skip {
            continue;
        }

        // Field doc comment
        if let Some(doc) = &field.doc {
            output.push_str(&format!("  /** {} */\n", doc.trim()));
        }

        let field_name = field.rename.as_deref().unwrap_or(&field.name);
        let type_str = field
            .type_override
            .clone()
            .unwrap_or_else(|| mapper.map_type(&field.ty));

        let readonly_prefix = if mapper.readonly { "readonly " } else { "" };

        if field.flatten {
            output.push_str("  // @flatten\n");
        }

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
/// - Data-carrying enums → discriminated union types based on `EnumRepr`
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
/// Dispatches based on `EnumRepr`:
/// - **External**: `{ "VariantName": { ...data } }`
/// - **Internal**: `{ type: "variant", ...data }`
/// - **Adjacent**: `{ t: "variant", c: { ...data } }`
/// - **Untagged**: `{ ...data }` — no discriminator
fn render_discriminated_union(mapper: &TypeScriptMapper, def: &EnumDef) -> String {
    let mut output = String::new();

    if let Some(doc) = &def.doc {
        output.push_str(&format!("/**\n * {}\n */\n", doc.trim()));
    }

    let variants: Vec<String> = def
        .variants
        .iter()
        .map(|v| render_variant_type(mapper, def, v))
        .collect();

    output.push_str(&format!(
        "export type {} =\n  | {};\n",
        def.name,
        variants.join("\n  | ")
    ));

    output
}

/// Render a single variant based on the enum representation.
fn render_variant_type(mapper: &TypeScriptMapper, def: &EnumDef, v: &VariantDef) -> String {
    let variant_name = v.rename.as_deref().unwrap_or(&v.name);

    match &def.representation {
        EnumRepr::External => render_variant_external(mapper, v, variant_name),
        EnumRepr::Internal { tag } => render_variant_internal(mapper, v, variant_name, tag),
        EnumRepr::Adjacent { tag, content } => {
            render_variant_adjacent(mapper, v, variant_name, tag, content)
        }
        EnumRepr::Untagged => render_variant_untagged(mapper, v),
    }
}

/// External: `{ "VariantName": { ...data } }` or `{ "VariantName": value }`
fn render_variant_external(
    mapper: &TypeScriptMapper,
    v: &VariantDef,
    variant_name: &str,
) -> String {
    match &v.kind {
        VariantKind::Unit => {
            format!("{{ \"{}\": {{}} }}", variant_name)
        }
        VariantKind::Struct(fields) => {
            let field_strs = render_field_strs(mapper, fields);
            format!(
                "{{ \"{}\": {{ {} }} }}",
                variant_name,
                field_strs.join("; ")
            )
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();
            if types.len() == 1 {
                format!("{{ \"{}\": {} }}", variant_name, type_strs[0])
            } else {
                format!("{{ \"{}\": [{}] }}", variant_name, type_strs.join(", "))
            }
        }
    }
}

/// Internal: `{ tag: "variant", ...data }` — data fields are inlined alongside tag
fn render_variant_internal(
    mapper: &TypeScriptMapper,
    v: &VariantDef,
    variant_name: &str,
    tag: &str,
) -> String {
    match &v.kind {
        VariantKind::Unit => {
            format!("{{ {}: \"{}\" }}", tag, variant_name)
        }
        VariantKind::Struct(fields) => {
            let field_strs = render_field_strs(mapper, fields);
            format!(
                "{{ {}: \"{}\"; {} }}",
                tag,
                variant_name,
                field_strs.join("; ")
            )
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();
            if types.len() == 1 {
                format!(
                    "{{ {}: \"{}\"; value: {} }}",
                    tag, variant_name, type_strs[0]
                )
            } else {
                format!(
                    "{{ {}: \"{}\"; value: [{}] }}",
                    tag,
                    variant_name,
                    type_strs.join(", ")
                )
            }
        }
    }
}

/// Adjacent: `{ tag: "variant", content: { ...data } }`
fn render_variant_adjacent(
    mapper: &TypeScriptMapper,
    v: &VariantDef,
    variant_name: &str,
    tag: &str,
    content: &str,
) -> String {
    match &v.kind {
        VariantKind::Unit => {
            format!("{{ {}: \"{}\" }}", tag, variant_name)
        }
        VariantKind::Struct(fields) => {
            let field_strs = render_field_strs(mapper, fields);
            format!(
                "{{ {}: \"{}\"; {}: {{ {} }} }}",
                tag,
                variant_name,
                content,
                field_strs.join("; ")
            )
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();
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
    }
}

/// Untagged: no discriminator — just the data shape
fn render_variant_untagged(mapper: &TypeScriptMapper, v: &VariantDef) -> String {
    match &v.kind {
        VariantKind::Unit => "{}".to_string(),
        VariantKind::Struct(fields) => {
            let field_strs = render_field_strs(mapper, fields);
            format!("{{ {} }}", field_strs.join("; "))
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|t| mapper.map_type(t)).collect();
            if types.len() == 1 {
                type_strs[0].clone()
            } else {
                format!("[{}]", type_strs.join(", "))
            }
        }
    }
}

/// Helper to render struct fields as `name: type` strings.
fn render_field_strs(mapper: &TypeScriptMapper, fields: &[FieldDef]) -> Vec<String> {
    fields
        .iter()
        .filter(|f| !f.skip)
        .map(|f| {
            let fname = f.rename.as_deref().unwrap_or(&f.name);
            let ftype = f
                .type_override
                .clone()
                .unwrap_or_else(|| mapper.map_type(&f.ty));

            let mut field_decl = String::new();
            if f.flatten {
                field_decl.push_str("  // @flatten\n  ");
            }
            if f.optional {
                field_decl.push_str(&format!("{}?: {}", fname, ftype));
            } else {
                field_decl.push_str(&format!("{}: {}", fname, ftype));
            }
            field_decl
        })
        .collect()
}
