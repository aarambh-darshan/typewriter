//! # typewriter-typescript
//!
//! TypeScript / JavaScript emitter for the typewriter type sync SDK.
//! Generates `.ts` typed interfaces and discriminated union types.

mod emitter;
mod mapper;

pub use mapper::TypeScriptMapper;
