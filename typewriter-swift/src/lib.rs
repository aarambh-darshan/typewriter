//! # typewriter-swift
//!
//! Swift emitter for the typewriter type sync SDK.
//! Generates `.swift` files with `Codable` structs and enums.

mod emitter;
mod mapper;

pub use mapper::SwiftMapper;
