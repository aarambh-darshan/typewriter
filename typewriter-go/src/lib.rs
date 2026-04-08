//! # typewriter-go
//!
//! Go (Golang) emitter for the typewriter type sync SDK.
//! Generates `.go` files with Go structs and type definitions.
//!
//! ## Type Mappings
//!
//! | Rust Type | Go Type |
//! |-----------|---------|
//! | `String` | `string` |
//! | `bool` | `bool` |
//! | `u8`, `u16`, `u32` | `uint8`, `uint16`, `uint32` |
//! | `u64` | `uint64` |
//! | `i8`, `i16`, `i32`, `i64` | `int8`, `int16`, `int32`, `int64` |
//! | `f32`, `f64` | `float32`, `float64` |
//! | `Uuid` | `string` |
//! | `DateTime<Utc>` | `time.Time` |
//! | `Option<T>` | `*T` with `omitempty` |
//! | `Vec<T>` | `[]T` |
//! | `HashMap<K, V>` | `map[K]V` |
//!
//! ## Configuration
//!
//! In `typewriter.toml`:
//!
//! ```toml
//! [go]
//! output_dir = "./generated/go"
//! file_style = "snake_case"   # snake_case (default), kebab-case, or PascalCase
//! package_name = "types"       # Go package name (default: "types")
//! ```
//!
//! ## Example
//!
//! **Rust Input:**
//! ```rust,ignore
//! #[derive(TypeWriter)]
//! #[sync_to(go)]
//! pub struct User {
//!     pub id: String,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! **Generated Go:**
//! ```go
//! package types
//!
//! type User struct {
//!     Id    string  `json:"id"`
//!     Email string  `json:"email"`
//!     Age   *uint32 `json:"age,omitempty"`
//! }
//! ```

mod emitter;
mod mapper;

pub use mapper::GoMapper;
