//! # typewriter-graphql
//!
//! GraphQL Schema Definition Language (SDL) emitter for the typewriter type sync SDK.
//! Generates `.graphql` type definitions, enums, and unions from Rust structs and enums.
//!
//! ## Type Mappings
//!
//! | Rust Type | GraphQL Type |
//! |-----------|--------------|
//! | `String` | `String` |
//! | `bool` | `Boolean` |
//! | `u8`, `u16`, `u32`, `i8`, `i16`, `i32` | `Int` |
//! | `f32`, `f64` | `Float` |
//! | `Uuid` | `ID` |
//! | `DateTime<Utc>` | `DateTime` (custom scalar) |
//! | `Option<T>` | Nullable (no `!`) |
//! | `Vec<T>` | `[T!]` |
//! | `HashMap<K, V>` | `JSON` (custom scalar) |
//!
//! ## Configuration
//!
//! In `typewriter.toml`:
//!
//! ```toml
//! [graphql]
//! output_dir = "./generated/graphql"
//! file_style = "snake_case"   # snake_case (default), kebab-case, or PascalCase
//! ```
//!
//! ## Example
//!
//! **Rust Input:**
//! ```rust,ignore
//! #[derive(TypeWriter)]
//! #[sync_to(graphql)]
//! pub struct User {
//!     pub id: String,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! **Generated GraphQL SDL:**
//! ```graphql
//! scalar DateTime
//! scalar JSON
//!
//! type User {
//!   id: ID!
//!   email: String!
//!   age: Int
//! }
//! ```

mod emitter;
mod mapper;

pub use mapper::GraphQLMapper;
