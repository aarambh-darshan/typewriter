# JSON Schema

Generates JSON Schema Draft 2020-12 definitions.

## Type Mappings

| Rust Type | JSON Schema Type |
|-----------|------------------|
| `String` | `{ "type": "string" }` |
| `bool` | `{ "type": "boolean" }` |
| `u8`-`u32`, `i8`-`i32` | `{ "type": "integer" }` |
| `u64`, `i64`, `u128`, `i128` | `{ "type": "integer" }` (64-bit) |
| `f32`, `f64` | `{ "type": "number" }` |
| `Uuid` | `{ "type": "string", "format": "uuid" }` |
| `DateTime<Utc>` | `{ "type": "string", "format": "date-time" }` |
| `NaiveDate` | `{ "type": "string", "format": "date" }` |
| `Option<T>` | Not in `required` array |
| `Vec<T>` | `{ "type": "array", "items": {...} }` |
| `HashMap<K, V>` | `{ "type": "object", "additionalProperties": {...} }` |

## Example

**Rust:**
```rust
#[derive(TypeWriter)]
#[sync_to(json_schema)]
pub struct User {
    pub id: String,
    pub email: String,
    pub age: Option<u32>,
}
```

**Generated:**
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "type": "object",
  "properties": {
    "id": { "type": "string", "format": "uuid" },
    "email": { "type": "string" },
    "age": { "type": "integer" }
  },
  "required": ["id", "email"],
  "additionalProperties": false
}
```

## File Naming

Files use snake_case by default: `UserProfile` → `user_profile.schema.json`

```toml
[json_schema]
file_style = "kebab-case"  # user-profile.schema.json
```
