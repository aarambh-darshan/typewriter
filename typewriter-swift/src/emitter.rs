use crate::mapper::SwiftMapper;
use typewriter_core::ir::{EnumDef, EnumRepr, FieldDef, StructDef, VariantKind};
use typewriter_core::mapper::TypeMapper;

pub fn render_struct(mapper: &SwiftMapper, def: &StructDef) -> String {
    let mut out = String::new();

    if let Some(doc) = &def.doc {
        out.push_str(&format!("/// {}\n", doc));
    }

    out.push_str(&format!("struct {}", def.name));

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

    out.push_str(": Codable {\n");

    let active_fields: Vec<&FieldDef> = def.fields.iter().filter(|f| !f.skip).collect();

    for field in &active_fields {
        if let Some(doc) = &field.doc {
            out.push_str(&format!("    /// {}\n", doc));
        }

        if field.flatten {
            out.push_str("    // @flatten\n");
        }
        let swift_type = field.type_override.clone().unwrap_or_else(|| mapper.map_type(&field.ty));
        let field_name = to_camel_case(&field.name);

        if field.optional {
            out.push_str(&format!("    let {}: {}?\n", field_name, swift_type));
        } else {
            out.push_str(&format!("    let {}: {}\n", field_name, swift_type));
        }
    }

    // Generate CodingKeys if any field has a rename or needs snake_case → camelCase mapping
    let needs_coding_keys = active_fields.iter().any(|f| {
        f.rename.is_some() || to_camel_case(&f.name) != f.name
    });

    if needs_coding_keys {
        out.push_str("\n    enum CodingKeys: String, CodingKey {\n");
        for field in &active_fields {
            let swift_name = to_camel_case(&field.name);
            let json_name = field.rename.as_deref().unwrap_or(&field.name);

            if swift_name == json_name {
                out.push_str(&format!("        case {}\n", swift_name));
            } else {
                out.push_str(&format!("        case {} = \"{}\"\n", swift_name, json_name));
            }
        }
        out.push_str("    }\n");
    }

    out.push_str("}\n\n");
    out
}

pub fn render_enum(mapper: &SwiftMapper, def: &EnumDef) -> String {
    let mut out = String::new();

    if let Some(doc) = &def.doc {
        out.push_str(&format!("/// {}\n", doc));
    }

    let is_simple = def.variants.iter().all(|v| matches!(v.kind, VariantKind::Unit));

    if is_simple {
        out.push_str(&format!("enum {}: String, Codable {{\n", def.name));
        for variant in &def.variants {
            if let Some(doc) = &variant.doc {
                out.push_str(&format!("    /// {}\n", doc));
            }
            let value = variant.rename.as_deref().unwrap_or(&variant.name);
            let swift_name = to_lower_camel_case(&variant.name);
            if swift_name == value {
                out.push_str(&format!("    case {}\n", swift_name));
            } else {
                out.push_str(&format!("    case {} = \"{}\"\n", swift_name, value));
            }
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

/// Internal: `{ "type": "variant", ...data }` — tag is a coding key in each case.
fn render_enum_internal(mapper: &SwiftMapper, def: &EnumDef, out: &mut String, tag: &str) {
    out.push_str(&format!("enum {}: Codable {{\n", def.name));

    // Case declarations
    for variant in &def.variants {
        if let Some(doc) = &variant.doc {
            out.push_str(&format!("    /// {}\n", doc));
        }
        let case_name = to_lower_camel_case(&variant.name);
        render_swift_case(mapper, out, &case_name, &variant.kind);
    }

    // CodingKeys with tag
    out.push_str(&format!("\n    enum CodingKeys: String, CodingKey {{\n"));
    out.push_str(&format!("        case {} = \"{}\"\n", to_camel_case(tag), tag));
    out.push_str("    }\n");

    // Variant-specific CodingKeys
    for variant in &def.variants {
        if let VariantKind::Struct(fields) = &variant.kind {
            let active: Vec<&FieldDef> = fields.iter().filter(|f| !f.skip).collect();
            if !active.is_empty() {
                out.push_str(&format!(
                    "\n    enum {}Keys: String, CodingKey {{\n",
                    variant.name
                ));
                for field in &active {
                    let swift_name = to_camel_case(&field.name);
                    let json_name = field.rename.as_deref().unwrap_or(&field.name);
                    if swift_name == json_name {
                        out.push_str(&format!("        case {}\n", swift_name));
                    } else {
                        out.push_str(&format!(
                            "        case {} = \"{}\"\n",
                            swift_name, json_name
                        ));
                    }
                }
                out.push_str("    }\n");
            }
        }
    }

    out.push_str("}\n\n");
}

/// External: `{ "VariantName": { ...data } }` — variant name is the key.
fn render_enum_external(mapper: &SwiftMapper, def: &EnumDef, out: &mut String) {
    out.push_str(&format!("enum {}: Codable {{\n", def.name));

    for variant in &def.variants {
        if let Some(doc) = &variant.doc {
            out.push_str(&format!("    /// {}\n", doc));
        }
        let case_name = to_lower_camel_case(&variant.name);
        render_swift_case(mapper, out, &case_name, &variant.kind);
    }

    // CodingKeys — one per variant name
    out.push_str("\n    enum CodingKeys: String, CodingKey {\n");
    for variant in &def.variants {
        let case_name = to_lower_camel_case(&variant.name);
        let serial_name = variant.rename.as_deref().unwrap_or(&variant.name);
        if case_name == serial_name {
            out.push_str(&format!("        case {}\n", case_name));
        } else {
            out.push_str(&format!(
                "        case {} = \"{}\"\n",
                case_name, serial_name
            ));
        }
    }
    out.push_str("    }\n");

    out.push_str("}\n\n");
}

/// Adjacent: `{ "t": "variant", "c": { ...data } }` — tag + content keys.
fn render_enum_adjacent(
    mapper: &SwiftMapper,
    def: &EnumDef,
    out: &mut String,
    tag: &str,
    content: &str,
) {
    out.push_str(&format!("enum {}: Codable {{\n", def.name));

    for variant in &def.variants {
        if let Some(doc) = &variant.doc {
            out.push_str(&format!("    /// {}\n", doc));
        }
        let case_name = to_lower_camel_case(&variant.name);
        render_swift_case(mapper, out, &case_name, &variant.kind);
    }

    // CodingKeys for tag + content
    out.push_str(&format!("\n    enum CodingKeys: String, CodingKey {{\n"));
    out.push_str(&format!(
        "        case {} = \"{}\"\n",
        to_camel_case(tag),
        tag
    ));
    out.push_str(&format!(
        "        case {} = \"{}\"\n",
        to_camel_case(content),
        content
    ));
    out.push_str("    }\n");

    out.push_str("}\n\n");
}

/// Untagged: no discriminator — try each case until one succeeds.
fn render_enum_untagged(mapper: &SwiftMapper, def: &EnumDef, out: &mut String) {
    out.push_str(&format!("enum {}: Codable {{\n", def.name));

    for variant in &def.variants {
        if let Some(doc) = &variant.doc {
            out.push_str(&format!("    /// {}\n", doc));
        }
        let case_name = to_lower_camel_case(&variant.name);
        render_swift_case(mapper, out, &case_name, &variant.kind);
    }

    out.push_str("}\n\n");
}

/// Helper: render a Swift `case` declaration.
fn render_swift_case(mapper: &SwiftMapper, out: &mut String, case_name: &str, kind: &VariantKind) {
    match kind {
        VariantKind::Unit => {
            out.push_str(&format!("    case {}\n", case_name));
        }
        VariantKind::Tuple(types) => {
            let type_strs: Vec<String> = types.iter().map(|ty| mapper.map_type(ty)).collect();
            out.push_str(&format!(
                "    case {}({})\n",
                case_name,
                type_strs.join(", ")
            ));
        }
        VariantKind::Struct(fields) => {
            let field_strs: Vec<String> = fields
                .iter()
                .filter(|f| !f.skip)
                .map(|f| {
                    if f.flatten {
                        out.push_str("    // @flatten\n"); // Just as a marker before fields in case, though Swift cases are on one line in our generation.
                    }
                    let swift_type = f.type_override.clone().unwrap_or_else(|| mapper.map_type(&f.ty));
                    let name = to_camel_case(&f.name);
                    if f.optional {
                        format!("{}: {}?", name, swift_type)
                    } else {
                        format!("{}: {}", name, swift_type)
                    }
                })
                .collect();
            out.push_str(&format!(
                "    case {}({})\n",
                case_name,
                field_strs.join(", ")
            ));
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

/// Convert PascalCase to lowerCamelCase for enum case names.
fn to_lower_camel_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => {
            let mut result: String = f.to_lowercase().collect();
            result.push_str(chars.as_str());
            result
        }
    }
}
