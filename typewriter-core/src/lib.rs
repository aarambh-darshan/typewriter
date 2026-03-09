//! # typewriter-core
//!
//! Core types, traits, and configuration for the typewriter type sync SDK.
//!
//! This crate has **zero proc-macro dependencies** — it can be used in build scripts,
//! CLI tools, and regular application code.

pub mod config;
pub mod ir;
pub mod mapper;

pub use config::TypewriterConfig;
pub use ir::*;
pub use mapper::TypeMapper;
