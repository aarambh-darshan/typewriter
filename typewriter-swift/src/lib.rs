//! # typewriter-swift
//!
//! Swift emitter for the typewriter type sync SDK.
//! Generates `.swift` files with `Codable` structs and enums.
//!
//! ## Type Mappings
//!
//! | Rust Type | Swift Type |
//! |-----------|------------|
//! | `String` | `String` |
//! | `bool` | `Bool` |
//! | `u8`, `u16`, `u32`, `u64` | `UInt8`, `UInt16`, `UInt32`, `UInt64` |
//! | `i8`, `i16`, `i32`, `i64` | `Int8`, `Int16`, `Int32`, `Int64` |
//! | `f32`, `f64` | `Float`, `Double` |
//! | `Uuid` | `UUID` |
//! | `DateTime<Utc>` | `Date` |
//! | `Option<T>` | `T?` |
//! | `Vec<T>` | `[T]` |
//! | `HashMap<K, V>` | `[K: V]` |
//!
//! ## Configuration
//!
//! In `typewriter.toml`:
//!
//! ```toml
//! [swift]
//! output_dir = "./generated/swift"
//! file_style = "PascalCase"   # PascalCase (default), snake_case, or kebab-case
//! ```
//!
//! ## Example
//!
//! **Rust Input:**
//! ```rust,ignore
//! #[derive(TypeWriter)]
//! #[sync_to(swift)]
//! pub struct User {
//!     pub id: String,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! **Generated Swift:**
//! ```swift
//! struct User: Codable {
//!     let id: String
//!     let email: String
//!     let age: UInt32?
//! }
//! ```

mod emitter;
mod mapper;

pub use mapper::SwiftMapper;
