use crate::mapper::GoMapper;
use typewriter_core::ir::{EnumDef, StructDef, VariantKind};
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

        let go_type = mapper.map_type(&field.ty);
        
        let json_name = field.rename.as_deref().unwrap_or(&field.name);
        // Struct field names in Go must be capitalized to be exported and parsed by encoding/json
        let go_name = capitalize_first_letter(&field.name);
        
        let mut json_tag = String::from(json_name);
        if field.optional {
            json_tag.push_str(",omitempty");
        }
        
        out.push_str(&format!("\t{} {} `json:\"{}\"`\n", go_name, go_type, json_tag));
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
    let is_simple = def.variants.iter().all(|v| matches!(v.kind, VariantKind::Unit));

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
        // Data-carrying enums are translated to an interface type in Go
        // with independent wrapper structs that implement that interface.
        // Also generated is a custom UnmarshalJSON.

        out.push_str(&format!("type {} interface {{\n\tis{}()\n}}\n\n", def.name, def.name));

        for variant in &def.variants {
            let variant_name = format!("{}{}", def.name, capitalize_first_letter(&variant.name));
            
            if let Some(doc) = &variant.doc {
                out.push_str(&format!("// {}\n", doc));
            }
            
            out.push_str(&format!("type {} struct {{\n", variant_name));
            
            match &variant.kind {
                VariantKind::Unit => {
                    // Empty struct
                }
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
                        let go_type = mapper.map_type(&field.ty);
                        let json_name = field.rename.as_deref().unwrap_or(&field.name);
                        let go_name = capitalize_first_letter(&field.name);
                        
                        let mut json_tag = String::from(json_name);
                        if field.optional {
                            json_tag.push_str(",omitempty");
                        }
                        
                        out.push_str(&format!("\t{} {} `json:\"{}\"`\n", go_name, go_type, json_tag));
                    }
                }
            }
            
            out.push_str("}\n\n");
            
            // Implement the marker interface
            out.push_str(&format!("func (x *{}) is{}() {{}}\n\n", variant_name, def.name));
        }
        
        // Note: Full enum unmarshaling in Go requires custom logic on the parent struct 
        // or a wrapper type, as Go does not allow methods on interfaces.
    }

    out
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
