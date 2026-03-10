//! # typebridge
//!
//! Cross-Language Type Synchronization SDK for Rust.
//!
//! Define your types once in Rust. Get perfectly matching types in TypeScript,
//! Python, Go, Swift, and Kotlin — automatically, forever.
//!
//! # Usage
//!
//! ```rust,ignore
//! use typebridge::TypeWriter;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, TypeWriter)]
//! #[sync_to(typescript, python)]
//! pub struct UserProfile {
//!     pub id: Uuid,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! // On cargo build, generates:
//! // ✅ ./generated/typescript/user-profile.ts
//! // ✅ ./generated/python/user_profile.py
//! ```

/// Re-export the `TypeWriter` derive macro.
pub use typewriter_macros::TypeWriter;

/// Re-export core types for advanced usage.
pub use typewriter_core::*;
