//! # typewriter-json-schema
//!
//! JSON Schema (Draft 2020-12) emitter for the typewriter type sync SDK.
//! Generates `.schema.json` definitions from Rust structs and enums.

mod emitter;
mod mapper;

pub use mapper::JsonSchemaMapper;
