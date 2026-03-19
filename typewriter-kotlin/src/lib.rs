//! # typewriter-kotlin
//!
//! Kotlin emitter for the typewriter type sync SDK.
//! Generates `.kt` files with `@Serializable` data classes and sealed classes.

mod emitter;
mod mapper;

pub use mapper::KotlinMapper;
