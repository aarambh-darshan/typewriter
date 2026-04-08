//! # typewriter-python
//!
//! Python Pydantic v2 / dataclass emitter for the typewriter type sync SDK.
//! Generates `.py` files with Pydantic `BaseModel` classes.
//!
//! ## Type Mappings
//!
//! | Rust Type | Python Type |
//! |-----------|-------------|
//! | `String` | `str` |
//! | `bool` | `bool` |
//! | `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64` | `int` |
//! | `f32`, `f64` | `float` |
//! | `Uuid` | `UUID` |
//! | `DateTime<Utc>` | `datetime` |
//! | `Option<T>` | `Optional[T] = None` |
//! | `Vec<T>` | `list[T]` |
//! | `HashMap<K, V>` | `dict[K, V]` |
//! | Custom struct/enum | Class reference |
//!
//! ## Configuration
//!
//! In `typewriter.toml`:
//!
//! ```toml
//! [python]
//! output_dir = "./generated/python"
//! file_style = "snake_case"   # snake_case (default), kebab-case, or PascalCase
//! pydantic_v2 = true          # Use Pydantic v2 (default: true)
//! use_dataclass = false       # Use @dataclass instead of BaseModel
//! ```
//!
//! ## Example
//!
//! **Rust Input:**
//! ```rust,ignore
//! #[derive(TypeWriter)]
//! #[sync_to(python)]
//! pub struct User {
//!     pub id: String,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! **Generated Python:**
//! ```python
//! from pydantic import BaseModel
//! from typing import Optional
//!
//!
//! class User(BaseModel):
//!     id: str
//!     email: str
//!     age: Optional[int] = None
//! ```

mod emitter;
mod mapper;

pub use mapper::PythonMapper;
