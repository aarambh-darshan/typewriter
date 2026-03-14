//! # typewriter-go
//!
//! Go (Golang) emitter for the typewriter type sync SDK.
//! Generates `.go` files with Go structs and type definitions.

mod emitter;
mod mapper;

pub use mapper::GoMapper;
