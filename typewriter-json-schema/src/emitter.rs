//! JSON Schema code rendering helpers.
//!
//! Builds JSON Schema Draft 2020-12 objects from typewriter IR types.

use crate::mapper::JsonSchemaMapper;
use serde_json::{Map, Value, json};
use typewriter_core::ir::{EnumDef, EnumRepr, FieldDef, StructDef, VariantKind};
use typewriter_core::mapper::TypeMapper;

/// Render a Rust struct as a JSON Schema `object` definition.
///
/// ```json
/// {
///   "$schema": "https://json-schema.org/draft/2020-12/schema",
///   "$id": "UserProfile",
///   "title": "UserProfile",
///   "type": "object",
///   "properties": { ... },
///   "required": [ ... ]
/// }
/// ```
pub fn render_object_schema(mapper: &JsonSchemaMapper, def: &StructDef) -> String {
    let mut schema = Map::new();

    let mut properties = Map::new();
    let mut required = Vec::new();

    for field in &def.fields {
        if field.skip {
            continue;
        }

        let field_name = field.rename.as_deref().unwrap_or(&field.name);
        let field_schema = render_field_schema(mapper, field);

        properties.insert(field_name.to_string(), field_schema);

        if !field.optional {
            required.push(Value::String(field_name.to_string()));
        }
    }

    schema.insert("type".to_string(), json!("object"));

    // Doc comment → description
    if let Some(doc) = &def.doc {
        schema.insert("description".to_string(), json!(doc.trim()));
    }

    schema.insert("properties".to_string(), Value::Object(properties));

    if !required.is_empty() {
        schema.insert("required".to_string(), Value::Array(required));
    }

    schema.insert("additionalProperties".to_string(), json!(false));

    serde_json::to_string_pretty(&Value::Object(schema)).unwrap()
}

/// Render a Rust enum as a JSON Schema definition.
///
/// - Simple (all-unit) enums → `{ "type": "string", "enum": ["A", "B"] }`
/// - Data-carrying enums → `{ "oneOf": [...] }` with variant sub-schemas
pub fn render_enum_schema(mapper: &JsonSchemaMapper, def: &EnumDef) -> String {
    let all_unit = def
        .variants
        .iter()
        .all(|v| matches!(v.kind, VariantKind::Unit));

    let schema = if all_unit {
        render_simple_enum_schema(def)
    } else {
        render_data_enum_schema(mapper, def)
    };

    serde_json::to_string_pretty(&schema).unwrap()
}

/// Render a simple (all-unit) enum as a JSON Schema string enum.
fn render_simple_enum_schema(def: &EnumDef) -> Value {
    let mut schema = Map::new();

    schema.insert("type".to_string(), json!("string"));

    if let Some(doc) = &def.doc {
        schema.insert("description".to_string(), json!(doc.trim()));
    }

    let values: Vec<Value> = def
        .variants
        .iter()
        .map(|v| {
            let name = v.rename.as_deref().unwrap_or(&v.name);
            json!(name)
        })
        .collect();

    schema.insert("enum".to_string(), Value::Array(values));

    Value::Object(schema)
}

/// Render a data-carrying enum using `oneOf` composition.
fn render_data_enum_schema(mapper: &JsonSchemaMapper, def: &EnumDef) -> Value {
    let mut schema = Map::new();

    if let Some(doc) = &def.doc {
        schema.insert("description".to_string(), json!(doc.trim()));
    }

    let mut one_of = Vec::new();

    for variant in &def.variants {
        let serial_name = variant.rename.as_deref().unwrap_or(&variant.name);
        let variant_schema =
            render_variant_schema(mapper, &def.representation, serial_name, variant);
        one_of.push(variant_schema);
    }

    schema.insert("oneOf".to_string(), Value::Array(one_of));

    Value::Object(schema)
}

/// Render a single variant's sub-schema based on enum representation.
fn render_variant_schema(
    mapper: &JsonSchemaMapper,
    repr: &EnumRepr,
    serial_name: &str,
    variant: &typewriter_core::ir::VariantDef,
) -> Value {
    match repr {
        EnumRepr::External => render_external_variant(mapper, serial_name, variant),
        EnumRepr::Internal { tag } => render_internal_variant(mapper, tag, serial_name, variant),
        EnumRepr::Adjacent { tag, content } => {
            render_adjacent_variant(mapper, tag, content, serial_name, variant)
        }
        EnumRepr::Untagged => render_untagged_variant(mapper, variant),
    }
}

/// External representation: `{ "VariantName": { ...data } }` or `{ "VariantName": value }`
fn render_external_variant(
    mapper: &JsonSchemaMapper,
    serial_name: &str,
    variant: &typewriter_core::ir::VariantDef,
) -> Value {
    let mut obj = Map::new();
    obj.insert("type".to_string(), json!("object"));

    let mut properties = Map::new();

    match &variant.kind {
        VariantKind::Unit => {
            // `"VariantName"` as a string constant
            let mut variant_schema = Map::new();
            variant_schema.insert("type".to_string(), json!("string"));
            variant_schema.insert("const".to_string(), json!(serial_name));
            return Value::Object(variant_schema);
        }
        VariantKind::Struct(fields) => {
            let inner = build_struct_schema(mapper, fields);
            properties.insert(serial_name.to_string(), inner);
        }
        VariantKind::Tuple(types) => {
            let inner = build_tuple_schema(mapper, types);
            properties.insert(serial_name.to_string(), inner);
        }
    }

    obj.insert("properties".to_string(), Value::Object(properties));
    obj.insert("required".to_string(), json!([serial_name]));
    obj.insert("additionalProperties".to_string(), json!(false));

    if let Some(doc) = &variant.doc {
        obj.insert("description".to_string(), json!(doc.trim()));
    }

    Value::Object(obj)
}

/// Internal representation: `{ "tag": "VariantName", ...data }`
fn render_internal_variant(
    mapper: &JsonSchemaMapper,
    tag: &str,
    serial_name: &str,
    variant: &typewriter_core::ir::VariantDef,
) -> Value {
    let mut obj = Map::new();
    obj.insert("type".to_string(), json!("object"));

    let mut properties = Map::new();
    let mut required = vec![json!(tag)];

    // Tag field
    let mut tag_schema = Map::new();
    tag_schema.insert("type".to_string(), json!("string"));
    tag_schema.insert("const".to_string(), json!(serial_name));
    properties.insert(tag.to_string(), Value::Object(tag_schema));

    // Data fields
    match &variant.kind {
        VariantKind::Unit => {}
        VariantKind::Struct(fields) => {
            for field in fields {
                if field.skip {
                    continue;
                }
                let fname = field.rename.as_deref().unwrap_or(&field.name);
                let fschema = render_field_schema(mapper, field);
                properties.insert(fname.to_string(), fschema);
                if !field.optional {
                    required.push(json!(fname));
                }
            }
        }
        VariantKind::Tuple(types) => {
            for (i, ty) in types.iter().enumerate() {
                let field_name = format!("item{}", i);
                let ty_json: Value =
                    serde_json::from_str(&mapper.map_type(ty)).unwrap_or(json!({}));
                properties.insert(field_name.clone(), ty_json);
                required.push(json!(field_name));
            }
        }
    }

    if let Some(doc) = &variant.doc {
        obj.insert("description".to_string(), json!(doc.trim()));
    }

    obj.insert("properties".to_string(), Value::Object(properties));
    obj.insert("required".to_string(), Value::Array(required));
    obj.insert("additionalProperties".to_string(), json!(false));

    Value::Object(obj)
}

/// Adjacent representation: `{ "tag": "VariantName", "content": ...data }`
fn render_adjacent_variant(
    mapper: &JsonSchemaMapper,
    tag: &str,
    content: &str,
    serial_name: &str,
    variant: &typewriter_core::ir::VariantDef,
) -> Value {
    let mut obj = Map::new();
    obj.insert("type".to_string(), json!("object"));

    let mut properties = Map::new();
    let mut required = vec![json!(tag)];

    // Tag field
    let mut tag_schema = Map::new();
    tag_schema.insert("type".to_string(), json!("string"));
    tag_schema.insert("const".to_string(), json!(serial_name));
    properties.insert(tag.to_string(), Value::Object(tag_schema));

    // Content field
    match &variant.kind {
        VariantKind::Unit => {}
        VariantKind::Struct(fields) => {
            let inner = build_struct_schema(mapper, fields);
            properties.insert(content.to_string(), inner);
            required.push(json!(content));
        }
        VariantKind::Tuple(types) => {
            let inner = build_tuple_schema(mapper, types);
            properties.insert(content.to_string(), inner);
            required.push(json!(content));
        }
    }

    if let Some(doc) = &variant.doc {
        obj.insert("description".to_string(), json!(doc.trim()));
    }

    obj.insert("properties".to_string(), Value::Object(properties));
    obj.insert("required".to_string(), Value::Array(required));
    obj.insert("additionalProperties".to_string(), json!(false));

    Value::Object(obj)
}

/// Untagged representation: just the data, no discriminator.
fn render_untagged_variant(
    mapper: &JsonSchemaMapper,
    variant: &typewriter_core::ir::VariantDef,
) -> Value {
    match &variant.kind {
        VariantKind::Unit => {
            // Untagged unit variant — null
            json!({ "type": "null" })
        }
        VariantKind::Struct(fields) => build_struct_schema(mapper, fields),
        VariantKind::Tuple(types) => build_tuple_schema(mapper, types),
    }
}

/// Build an inline object schema from struct-variant fields.
fn build_struct_schema(mapper: &JsonSchemaMapper, fields: &[FieldDef]) -> Value {
    let mut obj = Map::new();
    obj.insert("type".to_string(), json!("object"));

    let mut properties = Map::new();
    let mut required = Vec::new();

    for field in fields {
        if field.skip {
            continue;
        }
        let fname = field.rename.as_deref().unwrap_or(&field.name);
        let fschema = render_field_schema(mapper, field);
        properties.insert(fname.to_string(), fschema);
        if !field.optional {
            required.push(json!(fname));
        }
    }

    obj.insert("properties".to_string(), Value::Object(properties));
    if !required.is_empty() {
        obj.insert("required".to_string(), Value::Array(required));
    }

    Value::Object(obj)
}

/// Build an inline schema for tuple variant data.
fn build_tuple_schema(mapper: &JsonSchemaMapper, types: &[typewriter_core::ir::TypeKind]) -> Value {
    if types.len() == 1 {
        // Single-element tuple — unwrap
        let ty_json: Value = serde_json::from_str(&mapper.map_type(&types[0])).unwrap_or(json!({}));
        return ty_json;
    }

    // Multi-element tuple — use an array with prefixItems
    let mut obj = Map::new();
    obj.insert("type".to_string(), json!("array"));

    let items: Vec<Value> = types
        .iter()
        .map(|ty| serde_json::from_str(&mapper.map_type(ty)).unwrap_or(json!({})))
        .collect();

    obj.insert("prefixItems".to_string(), Value::Array(items));
    obj.insert("minItems".to_string(), json!(types.len()));
    obj.insert("maxItems".to_string(), json!(types.len()));

    Value::Object(obj)
}

/// Render a single field's JSON Schema representation.
fn render_field_schema(mapper: &JsonSchemaMapper, field: &FieldDef) -> Value {
    if let Some(override_ty) = &field.type_override {
        let mut obj = Map::new();
        // Treat override as a raw type name
        obj.insert("type".to_string(), json!(override_ty));
        if let Some(doc) = &field.doc {
            obj.insert("description".to_string(), json!(doc.trim()));
        }
        return Value::Object(obj);
    }

    let ty = if field.optional {
        // Unwrap Option<T> to get T
        match &field.ty {
            typewriter_core::ir::TypeKind::Option(inner) => inner.as_ref(),
            other => other,
        }
    } else {
        &field.ty
    };

    let type_str = mapper.map_type(ty);
    let mut schema: Value = serde_json::from_str(&type_str).unwrap_or(json!({}));

    // Add description from doc comment
    if let Some(doc) = &field.doc
        && let Value::Object(ref mut map) = schema
    {
        map.insert("description".to_string(), json!(doc.trim()));
    }

    schema
}
