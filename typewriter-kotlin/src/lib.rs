//! # typewriter-kotlin
//!
//! Kotlin emitter for the typewriter type sync SDK.
//! Generates `.kt` files with `@Serializable` data classes and sealed classes.
//!
//! ## Type Mappings
//!
//! | Rust Type | Kotlin Type |
//! |-----------|-------------|
//! | `String` | `String` |
//! | `bool` | `Boolean` |
//! | `u8`, `u16`, `u32`, `u64` | `UByte`, `UShort`, `UInt`, `ULong` |
//! | `i8`, `i16`, `i32`, `i64` | `Byte`, `Short`, `Int`, `Long` |
//! | `f32`, `f64` | `Float`, `Double` |
//! | `Uuid` | `String` |
//! | `DateTime<Utc>` | `kotlinx.datetime.Instant` |
//! | `Option<T>` | `T? = null` |
//! | `Vec<T>` | `List<T>` |
//! | `HashMap<K, V>` | `Map<K, V>` |
//!
//! ## Configuration
//!
//! In `typewriter.toml`:
//!
//! ```toml
//! [kotlin]
//! output_dir = "./generated/kotlin"
//! file_style = "PascalCase"   # PascalCase (default), snake_case, or kebab-case
//! package_name = "types"      # Kotlin package name (default: "types")
//! ```
//!
//! ## Example
//!
//! **Rust Input:**
//! ```rust,ignore
//! #[derive(TypeWriter)]
//! #[sync_to(kotlin)]
//! pub struct User {
//!     pub id: String,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! **Generated Kotlin:**
//! ```kotlin
//! package types
//!
//! import kotlinx.serialization.SerialName
//! import kotlinx.serialization.Serializable
//!
//! @Serializable
//! data class User(
//!     @SerialName("id") val id: String,
//!     @SerialName("email") val email: String,
//!     @SerialName("age") val age: Int? = null
//! )
//! ```

mod emitter;
mod mapper;

pub use mapper::KotlinMapper;
