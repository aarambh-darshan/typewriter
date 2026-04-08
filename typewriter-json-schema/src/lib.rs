//! # typewriter-json-schema
//!
//! JSON Schema (Draft 2020-12) emitter for the typewriter type sync SDK.
//! Generates `.schema.json` definitions from Rust structs and enums.
//!
//! ## Type Mappings
//!
//! | Rust Type | JSON Schema Type |
//! |-----------|------------------|
//! | `String` | `{ "type": "string" }` |
//! | `bool` | `{ "type": "boolean" }` |
//! | `u8`-`u32`, `i8`-`i32` | `{ "type": "integer" }` |
//! | `u64`, `i64`, `u128`, `i128` | `{ "type": "integer" }` (64-bit) |
//! | `f32`, `f64` | `{ "type": "number" }` |
//! | `Uuid` | `{ "type": "string", "format": "uuid" }` |
//! | `DateTime<Utc>` | `{ "type": "string", "format": "date-time" }` |
//! | `NaiveDate` | `{ "type": "string", "format": "date" }` |
//! | `Option<T>` | Not in `required` array |
//! | `Vec<T>` | `{ "type": "array", "items": {...} }` |
//! | `HashMap<K, V>` | `{ "type": "object", "additionalProperties": {...} }` |
//!
//! ## Configuration
//!
//! In `typewriter.toml`:
//!
//! ```toml
//! [json_schema]
//! output_dir = "./generated/json-schema"
//! file_style = "snake_case"   # snake_case (default), kebab-case, or PascalCase
//! ```
//!
//! ## Example
//!
//! **Rust Input:**
//! ```rust,ignore
//! #[derive(TypeWriter)]
//! #[sync_to(json_schema)]
//! pub struct User {
//!     pub id: String,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! **Generated JSON Schema:**
//! ```json
//! {
//!   "$schema": "https://json-schema.org/draft/2020-12/schema",
//!   "type": "object",
//!   "properties": {
//!     "id": { "type": "string", "format": "uuid" },
//!     "email": { "type": "string" },
//!     "age": { "type": "integer" }
//!   },
//!   "required": ["id", "email"],
//!   "additionalProperties": false
//! }
//! ```

mod emitter;
mod mapper;

pub use mapper::JsonSchemaMapper;
