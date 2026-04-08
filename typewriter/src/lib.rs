//! # typebridge
//!
//! Cross-Language Type Synchronization SDK for Rust.
//!
//! Define your types once in Rust. Get perfectly matching types in TypeScript,
//! Python, Go, Swift, Kotlin, GraphQL, and JSON Schema — automatically, forever.
//!
//! ## Overview
//!
//! typewriter eliminates the tedious work of keeping types in sync across languages.
//! Annotate your Rust structs with `#[derive(TypeWriter)]` and `#[sync_to(...)]`,
//! and the types are automatically generated in all target languages on every build.
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use typebridge::TypeWriter;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, TypeWriter)]
//! #[sync_to(typescript, python, go)]
//! pub struct UserProfile {
//!     pub id: Uuid,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! On `cargo build`, this generates:
//! - `./generated/typescript/user-profile.ts` — TypeScript interface
//! - `./generated/typescript/user-profile.schema.ts` — Zod schema
//! - `./generated/python/user_profile.py` — Python Pydantic model
//! - `./generated/go/user_profile.go` — Go struct
//!
//! ## Supported Languages
//!
//! | Language | Feature Flag | Output |
//! |----------|--------------|--------|
//! | TypeScript | `typescript` (default) | `.ts` interfaces + Zod schemas |
//! | Python | `python` (default) | `.py` Pydantic models |
//! | Go | `go` (default) | `.go` structs |
//! | Swift | `swift` (default) | `.swift` Codable structs |
//! | Kotlin | `kotlin` (default) | `.kt` data classes |
//! | GraphQL | `graphql` (default) | `.graphql` SDL |
//! | JSON Schema | `json_schema` (default) | `.schema.json` |
//!
//! ## Feature Flags
//!
//! Disable languages you don't need to reduce compile times:
//!
//! ```toml
//! [dependencies]
//! typebridge = { version = "0.5.0", default-features = false, features = ["typescript", "python"] }
//! ```
//!
//! ## CLI
//!
//! For project-wide generation, drift checking, and watch mode:
//!
//! ```bash
//! cargo install typebridge-cli
//!
//! typewriter generate --all     # Generate all types
//! typewriter check --ci         # Check for drift
//! typewriter watch              # Watch for changes
//! ```
//!
//! See [`typebridge_cli`](https://crates.io/crates/typebridge-cli) for more.

/// Re-export the `TypeWriter` derive macro.
pub use typewriter_macros::TypeWriter;

/// Re-export core types for advanced usage.
pub use typewriter_core::*;
