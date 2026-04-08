//! JSON Schema type mapper implementation.

use serde_json::json;
use typewriter_core::ir::*;
use typewriter_core::mapper::TypeMapper;
use typewriter_core::naming::{FileStyle, to_file_style};

use crate::emitter;

/// JSON Schema language mapper.
///
/// Generates JSON Schema Draft 2020-12 definitions from Rust structs and enums.
///
/// ## Type Mappings
///
/// | Rust | JSON Schema |
/// |------|-------------|
/// | `struct` | `{ "type": "object", "properties": { ... } }` |
/// | Simple enum | `{ "type": "string", "enum": [...] }` |
/// | Data-carrying enum | `{ "oneOf": [...] }` |
/// | `String` | `{ "type": "string" }` |
/// | `bool` | `{ "type": "boolean" }` |
/// | `u8`–`u64`, `i8`–`i64` | `{ "type": "integer" }` |
/// | `u128`, `i128` | `{ "type": "string" }` |
/// | `f32`, `f64` | `{ "type": "number" }` |
/// | `Option<T>` | schema of T (field not in `required`) |
/// | `Vec<T>` | `{ "type": "array", "items": T }` |
/// | `HashMap<K,V>` | `{ "type": "object", "additionalProperties": V }` |
/// | `Uuid` | `{ "type": "string", "format": "uuid" }` |
/// | `DateTime` | `{ "type": "string", "format": "date-time" }` |
/// | `NaiveDate` | `{ "type": "string", "format": "date" }` |
/// | `JsonValue` | `{}` (any value) |
pub struct JsonSchemaMapper {
    /// File naming style (default: `snake_case`)
    pub file_style: FileStyle,
}

impl JsonSchemaMapper {
    pub fn new() -> Self {
        Self {
            file_style: FileStyle::SnakeCase,
        }
    }

    pub fn with_file_style(mut self, style: FileStyle) -> Self {
        self.file_style = style;
        self
    }
}

impl Default for JsonSchemaMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeMapper for JsonSchemaMapper {
    /// Map primitives to JSON Schema type snippets (returned as JSON strings).
    fn map_primitive(&self, ty: &PrimitiveType) -> String {
        match ty {
            PrimitiveType::String => json!({"type": "string"}).to_string(),
            PrimitiveType::Bool => json!({"type": "boolean"}).to_string(),
            PrimitiveType::U8
            | PrimitiveType::U16
            | PrimitiveType::U32
            | PrimitiveType::U64
            | PrimitiveType::I8
            | PrimitiveType::I16
            | PrimitiveType::I32
            | PrimitiveType::I64 => json!({"type": "integer"}).to_string(),
            PrimitiveType::U128 | PrimitiveType::I128 => {
                json!({"type": "string", "description": "128-bit integer serialized as string"})
                    .to_string()
            }
            PrimitiveType::F32 | PrimitiveType::F64 => json!({"type": "number"}).to_string(),
            PrimitiveType::Uuid => json!({"type": "string", "format": "uuid"}).to_string(),
            PrimitiveType::DateTime => json!({"type": "string", "format": "date-time"}).to_string(),
            PrimitiveType::NaiveDate => json!({"type": "string", "format": "date"}).to_string(),
            PrimitiveType::JsonValue => json!({}).to_string(),
        }
    }

    fn map_option(&self, inner: &TypeKind) -> String {
        // In JSON Schema, optional is handled via `required` array omission.
        // Return the inner type's schema.
        self.map_type(inner)
    }

    fn map_vec(&self, inner: &TypeKind) -> String {
        let inner_str = self.map_type(inner);
        let inner_val: serde_json::Value = serde_json::from_str(&inner_str).unwrap_or(json!({}));
        json!({"type": "array", "items": inner_val}).to_string()
    }

    fn map_hashmap(&self, _key: &TypeKind, value: &TypeKind) -> String {
        let val_str = self.map_type(value);
        let val: serde_json::Value = serde_json::from_str(&val_str).unwrap_or(json!({}));
        json!({"type": "object", "additionalProperties": val}).to_string()
    }

    fn map_tuple(&self, elements: &[TypeKind]) -> String {
        let items: Vec<serde_json::Value> = elements
            .iter()
            .map(|e| {
                let s = self.map_type(e);
                serde_json::from_str(&s).unwrap_or(json!({}))
            })
            .collect();

        json!({
            "type": "array",
            "prefixItems": items,
            "minItems": elements.len(),
            "maxItems": elements.len()
        })
        .to_string()
    }

    fn map_named(&self, name: &str) -> String {
        json!({"$ref": format!("{}.schema.json", to_file_style(name, self.file_style))}).to_string()
    }

    fn emit_struct(&self, def: &StructDef) -> String {
        emitter::render_object_schema(self, def)
    }

    fn emit_enum(&self, def: &EnumDef) -> String {
        emitter::render_enum_schema(self, def)
    }

    fn file_header(&self, type_name: &str) -> String {
        // JSON Schema files start with { — we wrap the emitted body.
        // The header provides the $schema, $id, and title.
        let header = json!({
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "$id": type_name,
            "title": type_name,
        });
        // We return the opening JSON lines; the body is merged by emit_type_def override.
        // For simplicity we embed the metadata differently — see emit_type_def.
        let _ = header; // used in emit_type_def override
        String::new()
    }

    fn file_extension(&self) -> &str {
        "schema.json"
    }

    fn file_naming(&self, type_name: &str) -> String {
        to_file_style(type_name, self.file_style)
    }

    fn map_generic(&self, name: &str, _params: &[TypeKind]) -> String {
        // JSON Schema has no generics — reference the base type
        json!({"$ref": format!("{}.schema.json", to_file_style(name, self.file_style))}).to_string()
    }

    /// Override emit_type_def to produce a complete JSON Schema document
    /// with `$schema`, `$id`, and `title` metadata.
    fn emit_type_def(&self, def: &TypeDef) -> String {
        let type_name = def.name();

        // Get the raw body (object schema or enum schema)
        let body_str = match def {
            TypeDef::Struct(s) => self.emit_struct(s),
            TypeDef::Enum(e) => self.emit_enum(e),
        };

        // Parse the body and inject metadata
        let mut body: serde_json::Value = serde_json::from_str(&body_str).unwrap_or(json!({}));

        if let serde_json::Value::Object(ref mut map) = body {
            // Insert metadata at the top — serde_json::Map is ordered by insertion,
            // but we want $schema first. Build a new ordered map.
            let mut ordered = serde_json::Map::new();
            ordered.insert(
                "$schema".to_string(),
                json!("https://json-schema.org/draft/2020-12/schema"),
            );
            ordered.insert("$id".to_string(), json!(type_name));
            ordered.insert("title".to_string(), json!(type_name));

            // Merge existing keys (skip duplicates)
            for (k, v) in map.iter() {
                if k != "$schema" && k != "$id" && k != "title" {
                    ordered.insert(k.clone(), v.clone());
                }
            }

            body = serde_json::Value::Object(ordered);
        }

        let mut output = serde_json::to_string_pretty(&body).unwrap();
        output.push('\n');
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mapper() -> JsonSchemaMapper {
        JsonSchemaMapper::new()
    }

    #[test]
    fn test_primitive_string() {
        let m = mapper();
        let result = m.map_primitive(&PrimitiveType::String);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "string");
    }

    #[test]
    fn test_primitive_bool() {
        let m = mapper();
        let result = m.map_primitive(&PrimitiveType::Bool);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "boolean");
    }

    #[test]
    fn test_primitive_integer() {
        let m = mapper();
        let result = m.map_primitive(&PrimitiveType::U32);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "integer");
    }

    #[test]
    fn test_primitive_number() {
        let m = mapper();
        let result = m.map_primitive(&PrimitiveType::F64);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "number");
    }

    #[test]
    fn test_primitive_uuid() {
        let m = mapper();
        let result = m.map_primitive(&PrimitiveType::Uuid);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "string");
        assert_eq!(v["format"], "uuid");
    }

    #[test]
    fn test_primitive_datetime() {
        let m = mapper();
        let result = m.map_primitive(&PrimitiveType::DateTime);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "string");
        assert_eq!(v["format"], "date-time");
    }

    #[test]
    fn test_vec_mapping() {
        let m = mapper();
        let result = m.map_vec(&TypeKind::Primitive(PrimitiveType::String));
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "array");
        assert_eq!(v["items"]["type"], "string");
    }

    #[test]
    fn test_hashmap_mapping() {
        let m = mapper();
        let result = m.map_hashmap(
            &TypeKind::Primitive(PrimitiveType::String),
            &TypeKind::Primitive(PrimitiveType::U32),
        );
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "object");
        assert_eq!(v["additionalProperties"]["type"], "integer");
    }

    #[test]
    fn test_file_naming() {
        let m = mapper();
        assert_eq!(m.file_naming("UserProfile"), "user_profile");
        assert_eq!(m.file_naming("User"), "user");
    }

    #[test]
    fn test_output_filename() {
        let m = mapper();
        assert_eq!(m.output_filename("UserProfile"), "user_profile.schema.json");
    }

    #[test]
    fn test_file_naming_kebab() {
        let m = JsonSchemaMapper::new().with_file_style(FileStyle::KebabCase);
        assert_eq!(m.file_naming("UserProfile"), "user-profile");
        assert_eq!(m.output_filename("UserProfile"), "user-profile.schema.json");
    }

    #[test]
    fn test_option_mapping() {
        let m = mapper();
        let result = m.map_option(&TypeKind::Primitive(PrimitiveType::U32));
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "integer");
    }

    #[test]
    fn test_tuple_mapping() {
        let m = mapper();
        let result = m.map_tuple(&[
            TypeKind::Primitive(PrimitiveType::String),
            TypeKind::Primitive(PrimitiveType::U32),
        ]);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["type"], "array");
        assert_eq!(v["prefixItems"][0]["type"], "string");
        assert_eq!(v["prefixItems"][1]["type"], "integer");
        assert_eq!(v["minItems"], 2);
        assert_eq!(v["maxItems"], 2);
    }

    #[test]
    fn test_named_type_ref() {
        let m = mapper();
        let result = m.map_named("UserProfile");
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["$ref"], "user_profile.schema.json");
    }

    #[test]
    fn test_generic_mapped_as_ref() {
        let m = mapper();
        let result = m.map_generic("Pagination", &[TypeKind::Named("User".to_string())]);
        let v: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(v["$ref"], "pagination.schema.json");
    }
}
