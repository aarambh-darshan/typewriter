//! # typewriter-graphql
//!
//! GraphQL Schema Definition Language (SDL) emitter for the typewriter type sync SDK.
//! Generates `.graphql` type definitions, enums, and unions from Rust structs and enums.

mod emitter;
mod mapper;

pub use mapper::GraphQLMapper;
