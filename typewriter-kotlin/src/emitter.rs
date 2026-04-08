#![allow(clippy::useless_format)]

use crate::mapper::KotlinMapper;
use typewriter_core::ir::{EnumDef, EnumRepr, StructDef, VariantKind};
use typewriter_core::mapper::TypeMapper;

pub fn render_struct(mapper: &KotlinMapper, def: &StructDef) -> String {
    let mut out = String::new();

    if let Some(doc) = &def.doc {
        out.push_str(&format!("/**\n * {}\n */\n", doc));
    }

    out.push_str("@Serializable\n");
    out.push_str(&format!("data class {}", def.name));

    if !def.generics.is_empty() {
        out.push('<');
        for (i, param) in def.generics.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(param);
        }
        out.push('>');
    }

    out.push_str("(\n");

    let active_fields: Vec<_> = def.fields.iter().filter(|f| !f.skip).collect();

    for (i, field) in active_fields.iter().enumerate() {
        if let Some(doc) = &field.doc {
            out.push_str(&format!("    /** {} */\n", doc));
        }

        // Add @SerialName if the field has a rename or if camelCase differs from original
        let kotlin_name = to_camel_case(&field.name);
        let json_name = field.rename.as_deref().unwrap_or(&field.name);
        if kotlin_name != json_name {
            out.push_str(&format!("    @SerialName(\"{}\")\n", json_name));
        }

        if field.flatten {
            out.push_str("    // @flatten\n");
        }
        let kotlin_type = field
            .type_override
            .clone()
            .unwrap_or_else(|| mapper.map_type(&field.ty));

        if field.optional {
            let suffix = if kotlin_type.ends_with('?') { "" } else { "?" };
            out.push_str(&format!(
                "    val {}: {}{} = null",
                kotlin_name, kotlin_type, suffix
            ));
        } else {
            out.push_str(&format!("    val {}: {}", kotlin_name, kotlin_type));
        }

        // Trailing comma for all fields
        out.push(',');
        if i < active_fields.len() - 1 {
            out.push('\n');
        }
    }

    out.push_str("\n)\n\n");
    out
}

pub fn render_enum(mapper: &KotlinMapper, def: &EnumDef) -> String {
    let mut out = String::new();

    if let Some(doc) = &def.doc {
        out.push_str(&format!("/**\n * {}\n */\n", doc));
    }

    let is_simple = def
        .variants
        .iter()
        .all(|v| matches!(v.kind, VariantKind::Unit));

    if is_simple {
        out.push_str("@Serializable\n");
        out.push_str(&format!("enum class {} {{\n", def.name));
        for (i, variant) in def.variants.iter().enumerate() {
            if let Some(doc) = &variant.doc {
                out.push_str(&format!("    /** {} */\n", doc));
            }
            let value = variant.rename.as_deref().unwrap_or(&variant.name);
            out.push_str(&format!("    @SerialName(\"{}\")\n", value));
            out.push_str(&format!("    {}", variant.name));
            if i < def.variants.len() - 1 {
                out.push(',');
            }
            out.push('\n');
        }
        out.push_str("}\n\n");
    } else {
        match &def.representation {
            EnumRepr::Internal { tag } => render_enum_internal(mapper, def, &mut out, tag),
            EnumRepr::External => render_enum_external(mapper, def, &mut out),
            EnumRepr::Adjacent { tag, content } => {
                render_enum_adjacent(mapper, def, &mut out, tag, content)
            }
            EnumRepr::Untagged => render_enum_untagged(mapper, def, &mut out),
        }
    }

    out
}

/// Internal: `@JsonClassDiscriminator("type")` on sealed class.
fn render_enum_internal(mapper: &KotlinMapper, def: &EnumDef, out: &mut String, tag: &str) {
    out.push_str("@Serializable\n");
    if tag != "type" {
        out.push_str(&format!("@JsonClassDiscriminator(\"{}\")\n", tag));
    }
    out.push_str(&format!("sealed class {} {{\n", def.name));

    for variant in &def.variants {
        render_kotlin_variant(mapper, def, out, variant);
    }

    out.push_str("}\n\n");
}

/// External: default polymorphic behavior with `@SerialName`.
fn render_enum_external(mapper: &KotlinMapper, def: &EnumDef, out: &mut String) {
    out.push_str("@Serializable\n");
    out.push_str(&format!("sealed class {} {{\n", def.name));

    for variant in &def.variants {
        render_kotlin_variant(mapper, def, out, variant);
    }

    out.push_str("}\n\n");
}

/// Adjacent: tag + content wrapper. Uses `@Serializable` with custom structure.
fn render_enum_adjacent(
    mapper: &KotlinMapper,
    def: &EnumDef,
    out: &mut String,
    tag: &str,
    content: &str,
) {
    out.push_str("@Serializable\n");
    out.push_str(&format!("sealed class {} {{\n", def.name));

    // Add tag/content info as a comment for documentation
    out.push_str(&format!(
        "    // Adjacent tagged: \"{}\" = tag, \"{}\" = content\n\n",
        tag, content
    ));

    for variant in &def.variants {
        render_kotlin_variant(mapper, def, out, variant);
    }

    out.push_str("}\n\n");
}

/// Untagged: no discriminator.
fn render_enum_untagged(mapper: &KotlinMapper, def: &EnumDef, out: &mut String) {
    out.push_str("@Serializable\n");
    out.push_str(&format!("sealed class {} {{\n", def.name));

    for variant in &def.variants {
        render_kotlin_variant(mapper, def, out, variant);
    }

    out.push_str("}\n\n");
}

/// Helper: render a single Kotlin variant as a subtype of the sealed class.
fn render_kotlin_variant(
    mapper: &KotlinMapper,
    def: &EnumDef,
    out: &mut String,
    variant: &typewriter_core::ir::VariantDef,
) {
    if let Some(doc) = &variant.doc {
        out.push_str(&format!("    /** {} */\n", doc));
    }

    let serial_name = variant.rename.as_deref().unwrap_or(&variant.name);
    out.push_str(&format!("    @SerialName(\"{}\")\n", serial_name));

    match &variant.kind {
        VariantKind::Unit => {
            out.push_str("    @Serializable\n");
            out.push_str(&format!("    object {} : {}()\n\n", variant.name, def.name));
        }
        VariantKind::Tuple(types) => {
            out.push_str("    @Serializable\n");
            out.push_str(&format!("    data class {}(\n", variant.name));
            for (i, ty) in types.iter().enumerate() {
                let kotlin_type = mapper.map_type(ty);
                out.push_str(&format!(
                    "        @SerialName(\"item_{}\") val item{}: {},\n",
                    i, i, kotlin_type
                ));
            }
            out.push_str(&format!("    ) : {}()\n\n", def.name));
        }
        VariantKind::Struct(fields) => {
            out.push_str("    @Serializable\n");
            out.push_str(&format!("    data class {}(\n", variant.name));
            for field in fields {
                if field.skip {
                    continue;
                }
                if let Some(doc) = &field.doc {
                    out.push_str(&format!("        /** {} */\n", doc));
                }
                let kotlin_name = to_camel_case(&field.name);
                let json_name = field.rename.as_deref().unwrap_or(&field.name);
                if kotlin_name != json_name {
                    out.push_str(&format!("        @SerialName(\"{}\")\n", json_name));
                }
                if field.flatten {
                    out.push_str("        // @flatten\n");
                }
                let kotlin_type = field
                    .type_override
                    .clone()
                    .unwrap_or_else(|| mapper.map_type(&field.ty));
                if field.optional {
                    let suffix = if kotlin_type.ends_with('?') { "" } else { "?" };
                    out.push_str(&format!(
                        "        val {}: {}{} = null,\n",
                        kotlin_name, kotlin_type, suffix
                    ));
                } else {
                    out.push_str(&format!("        val {}: {},\n", kotlin_name, kotlin_type));
                }
            }
            out.push_str(&format!("    ) : {}()\n\n", def.name));
        }
    }
}

/// Convert snake_case to camelCase.
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;
    for (i, c) in s.chars().enumerate() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_uppercase().next().unwrap());
            capitalize_next = false;
        } else if i == 0 {
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}
