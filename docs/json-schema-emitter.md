# 📘 JSON Schema Emitter

The JSON Schema emitter translates Rust data structures into [JSON Schema Draft 2020-12](https://json-schema.org/draft/2020-12) definitions, generating `.schema.json` files.

## File Organization

All generated `.schema.json` files are output into a single directory configured via `typewriter.toml` using `output_dir`. Typebridge generates one `.schema.json` file per Rust type.

Each file is a valid JSON Schema document with `$schema`, `$id`, and `title` metadata fields.

## Structs

Rust `struct`s become JSON Schema `object` types with `properties` and `required` arrays.

```rust
#[derive(TypeWriter)]
#[sync_to(json_schema)]
pub struct UserProfile {
    /// Unique identifier
    pub id: Uuid,
    pub email: String,
    pub age: Option<u32>,
}
```

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "UserProfile",
  "title": "UserProfile",
  "type": "object",
  "description": "UserProfile",
  "properties": {
    "id": {
      "type": "string",
      "format": "uuid",
      "description": "Unique identifier"
    },
    "email": {
      "type": "string"
    },
    "age": {
      "type": "integer"
    }
  },
  "required": ["id", "email"],
  "additionalProperties": false
}
```

**Key behaviors:**
- Non-optional fields appear in the `required` array
- `Option<T>` fields are omitted from `required` (schema of inner type `T` is used)
- `Uuid` maps to `{ "type": "string", "format": "uuid" }`
- `DateTime` maps to `{ "type": "string", "format": "date-time" }`
- Rust doc comments (`///`) become `description` fields
- `additionalProperties: false` enforces strict object shapes

## Simple Enums

Enums with only unit variants are generated as JSON Schema `string` enums.

```rust
#[derive(TypeWriter)]
#[sync_to(json_schema)]
pub enum Priority {
    High,
    Medium,
    Low,
}
```

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "Priority",
  "title": "Priority",
  "type": "string",
  "enum": ["High", "Medium", "Low"]
}
```

## Data-Carrying Enums

Data-carrying enums use `oneOf` composition. The exact schema depends on the serde representation.

### Internal Tag

```rust
#[derive(TypeWriter)]
#[serde(tag = "type")]
#[sync_to(json_schema)]
pub enum Event {
    Click { x: i32, y: i32 },
    Hover,
}
```

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "Event",
  "title": "Event",
  "oneOf": [
    {
      "type": "object",
      "properties": {
        "type": { "type": "string", "const": "Click" },
        "x": { "type": "integer" },
        "y": { "type": "integer" }
      },
      "required": ["type", "x", "y"],
      "additionalProperties": false
    },
    {
      "type": "object",
      "properties": {
        "type": { "type": "string", "const": "Hover" }
      },
      "required": ["type"],
      "additionalProperties": false
    }
  ]
}
```

## Enum Representations

Typebridge supports all four serde enum representations for JSON Schema:

| Representation | JSON Schema Strategy |
|---|---|
| `External` | `oneOf` with single-property objects per variant |
| `Internal { tag }` | `oneOf` with `const` discriminator on tag field |
| `Adjacent { tag, content }` | `oneOf` with `const` discriminator + content wrapper |
| `Untagged` | `oneOf` without discriminator fields |

## Type Mapping

| Rust Type | JSON Schema |
|---|---|
| `String` | `{ "type": "string" }` |
| `bool` | `{ "type": "boolean" }` |
| `u8`–`u64`, `i8`–`i64` | `{ "type": "integer" }` |
| `u128`, `i128` | `{ "type": "string" }` |
| `f32`, `f64` | `{ "type": "number" }` |
| `Option<T>` | schema of `T` (not in `required`) |
| `Vec<T>` | `{ "type": "array", "items": T }` |
| `HashMap<K, V>` | `{ "type": "object", "additionalProperties": V }` |
| `(A, B)` (tuples) | `{ "type": "array", "prefixItems": [...] }` |
| `Uuid` | `{ "type": "string", "format": "uuid" }` |
| `DateTime<Utc>` | `{ "type": "string", "format": "date-time" }` |
| `NaiveDate` | `{ "type": "string", "format": "date" }` |
| `serde_json::Value` | `{}` (any value) |

## Configuration

Configure the JSON Schema emitter in your `typewriter.toml`:

```toml
[json_schema]
# Where generated .schema.json files are written
# Default: "./generated/json-schema"
output_dir = "../schemas"

# File naming convention for output files
# Default: "snake_case"
# Options: "snake_case", "kebab-case", "PascalCase"
file_style = "snake_case"
```
