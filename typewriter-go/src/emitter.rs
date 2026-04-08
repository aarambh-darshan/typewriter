#![allow(
    clippy::useless_format,
    clippy::single_char_add_str,
    clippy::if_same_then_else
)]

use crate::mapper::GoMapper;
use typewriter_core::ir::{EnumDef, EnumRepr, StructDef, VariantKind};
use typewriter_core::mapper::TypeMapper;

pub fn render_struct(mapper: &GoMapper, def: &StructDef) -> String {
    let mut out = String::new();

    if let Some(doc) = &def.doc {
        out.push_str(&format!("// {}\n", doc));
    }

    out.push_str(&format!("type {}", def.name));

    if !def.generics.is_empty() {
        out.push_str("[");
        for (i, param) in def.generics.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(param);
            out.push_str(" any");
        }
        out.push_str("]");
    }

    out.push_str(" struct {\n");

    for field in &def.fields {
        if field.skip {
            continue;
        }

        if let Some(doc) = &field.doc {
            out.push_str(&format!("\t// {}\n", doc));
        }

        if field.flatten {
            out.push_str("\t// @flatten\n");
        }
        let go_type = field
            .type_override
            .clone()
            .unwrap_or_else(|| mapper.map_type(&field.ty));

        let json_name = field.rename.as_deref().unwrap_or(&field.name);
        // Struct field names in Go must be capitalized to be exported and parsed by encoding/json
        let go_name = capitalize_first_letter(&field.name);

        let mut json_tag = String::from(json_name);
        if field.optional {
            json_tag.push_str(",omitempty");
        }

        out.push_str(&format!(
            "\t{} {} `json:\"{}\"`\n",
            go_name, go_type, json_tag
        ));
    }

    out.push_str("}\n\n");
    out
}

pub fn render_enum(mapper: &GoMapper, def: &EnumDef) -> String {
    let mut out = String::new();

    if let Some(doc) = &def.doc {
        out.push_str(&format!("// {}\n", doc));
    }

    // Check if it's a simple enum (only unit variants) or data carrying
    let is_simple = def
        .variants
        .iter()
        .all(|v| matches!(v.kind, VariantKind::Unit));

    if is_simple {
        out.push_str(&format!("type {} string\n\n", def.name));
        out.push_str("const (\n");
        for variant in &def.variants {
            if let Some(doc) = &variant.doc {
                out.push_str(&format!("\t// {}\n", doc));
            }
            let value = variant.rename.as_deref().unwrap_or(&variant.name);
            let go_name = format!("{}{}", def.name, capitalize_first_letter(&variant.name));
            out.push_str(&format!("\t{} {} = \"{}\"\n", go_name, def.name, value));
        }
        out.push_str(")\n\n");
    } else {
        match &def.representation {
            EnumRepr::Internal { tag } => render_enum_internal(mapper, def, &mut out, tag),
            EnumRepr::Adjacent { tag, content } => {
                render_enum_adjacent(mapper, def, &mut out, tag, content)
            }
            EnumRepr::External => render_enum_external(mapper, def, &mut out),
            EnumRepr::Untagged => render_enum_untagged(mapper, def, &mut out),
        }
    }

    out
}

/// Internal: tag field inlined in each variant struct.
fn render_enum_internal(mapper: &GoMapper, def: &EnumDef, out: &mut String, tag: &str) {
    out.push_str(&format!(
        "type {} interface {{\n\tis{}()\n}}\n\n",
        def.name, def.name
    ));

    for variant in &def.variants {
        let variant_name = format!("{}{}", def.name, capitalize_first_letter(&variant.name));
        let serial_name = variant.rename.as_deref().unwrap_or(&variant.name);

        if let Some(doc) = &variant.doc {
            out.push_str(&format!("// {}\n", doc));
        }

        out.push_str(&format!("type {} struct {{\n", variant_name));

        // Tag field
        out.push_str(&format!(
            "\t{} string `json:\"{}\"`\n",
            capitalize_first_letter(tag),
            tag
        ));

        render_variant_struct_fields(mapper, &variant.kind, out);

        out.push_str("}\n\n");
        out.push_str(&format!(
            "func (x *{}) is{}() {{}}\n\n",
            variant_name, def.name
        ));

        // Constructor helper
        out.push_str(&format!(
            "func New{}() *{} {{\n\treturn &{}{{{}:  \"{}\"}}\n}}\n\n",
            variant_name,
            variant_name,
            variant_name,
            capitalize_first_letter(tag),
            serial_name
        ));
    }
}

/// External: `{ "VariantName": { ...data } }` — each variant struct is wrapped.
fn render_enum_external(mapper: &GoMapper, def: &EnumDef, out: &mut String) {
    out.push_str(&format!(
        "type {} interface {{\n\tis{}()\n}}\n\n",
        def.name, def.name
    ));

    for variant in &def.variants {
        let variant_name = format!("{}{}", def.name, capitalize_first_letter(&variant.name));

        if let Some(doc) = &variant.doc {
            out.push_str(&format!("// {}\n", doc));
        }

        out.push_str(&format!("type {} struct {{\n", variant_name));
        render_variant_struct_fields(mapper, &variant.kind, out);
        out.push_str("}\n\n");
        out.push_str(&format!(
            "func (x *{}) is{}() {{}}\n\n",
            variant_name, def.name
        ));
    }
}

/// Adjacent: `{ "t": "variant", "c": { ...data } }` — tag + content fields.
fn render_enum_adjacent(
    mapper: &GoMapper,
    def: &EnumDef,
    out: &mut String,
    tag: &str,
    content: &str,
) {
    out.push_str(&format!(
        "type {} interface {{\n\tis{}()\n}}\n\n",
        def.name, def.name
    ));

    for variant in &def.variants {
        let variant_name = format!("{}{}", def.name, capitalize_first_letter(&variant.name));
        let serial_name = variant.rename.as_deref().unwrap_or(&variant.name);
        let content_type = format!("{}Content", variant_name);

        if let Some(doc) = &variant.doc {
            out.push_str(&format!("// {}\n", doc));
        }

        // Content struct (for non-unit variants)
        let has_content = !matches!(&variant.kind, VariantKind::Unit);
        if has_content {
            out.push_str(&format!("type {} struct {{\n", content_type));
            render_variant_struct_fields(mapper, &variant.kind, out);
            out.push_str("}\n\n");
        }

        // Wrapper struct with tag + content
        out.push_str(&format!("type {} struct {{\n", variant_name));
        out.push_str(&format!(
            "\t{} string `json:\"{}\"`\n",
            capitalize_first_letter(tag),
            tag
        ));
        if has_content {
            out.push_str(&format!(
                "\t{} {} `json:\"{}\"`\n",
                capitalize_first_letter(content),
                content_type,
                content
            ));
        }
        out.push_str("}\n\n");

        out.push_str(&format!(
            "func (x *{}) is{}() {{}}\n\n",
            variant_name, def.name
        ));

        // Constructor
        if has_content {
            out.push_str(&format!(
                "func New{}() *{} {{\n\treturn &{}{{{}:  \"{}\"}}\n}}\n\n",
                variant_name,
                variant_name,
                variant_name,
                capitalize_first_letter(tag),
                serial_name
            ));
        } else {
            out.push_str(&format!(
                "func New{}() *{} {{\n\treturn &{}{{{}:  \"{}\"}}\n}}\n\n",
                variant_name,
                variant_name,
                variant_name,
                capitalize_first_letter(tag),
                serial_name
            ));
        }
    }
}

/// Untagged: same interface pattern, no tag field.
fn render_enum_untagged(mapper: &GoMapper, def: &EnumDef, out: &mut String) {
    out.push_str(&format!(
        "type {} interface {{\n\tis{}()\n}}\n\n",
        def.name, def.name
    ));

    for variant in &def.variants {
        let variant_name = format!("{}{}", def.name, capitalize_first_letter(&variant.name));

        if let Some(doc) = &variant.doc {
            out.push_str(&format!("// {}\n", doc));
        }

        out.push_str(&format!("type {} struct {{\n", variant_name));
        render_variant_struct_fields(mapper, &variant.kind, out);
        out.push_str("}\n\n");
        out.push_str(&format!(
            "func (x *{}) is{}() {{}}\n\n",
            variant_name, def.name
        ));
    }
}

/// Helper: render variant's struct/tuple fields into a Go struct body.
fn render_variant_struct_fields(mapper: &GoMapper, kind: &VariantKind, out: &mut String) {
    match kind {
        VariantKind::Unit => {}
        VariantKind::Tuple(types) => {
            for (i, ty) in types.iter().enumerate() {
                let go_type = mapper.map_type(ty);
                out.push_str(&format!("\tItem{} {} `json:\"item_{}\"`\n", i, go_type, i));
            }
        }
        VariantKind::Struct(fields) => {
            for field in fields {
                if field.skip {
                    continue;
                }
                if let Some(doc) = &field.doc {
                    out.push_str(&format!("\t// {}\n", doc));
                }
                if field.flatten {
                    out.push_str("\t// @flatten\n");
                }
                let go_type = field
                    .type_override
                    .clone()
                    .unwrap_or_else(|| mapper.map_type(&field.ty));
                let json_name = field.rename.as_deref().unwrap_or(&field.name);
                let go_name = capitalize_first_letter(&field.name);

                let mut json_tag = String::from(json_name);
                if field.optional {
                    json_tag.push_str(",omitempty");
                }

                out.push_str(&format!(
                    "\t{} {} `json:\"{}\"`\n",
                    go_name, go_type, json_tag
                ));
            }
        }
    }
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
