//! # typewriter-typescript
//!
//! TypeScript / JavaScript emitter for the typewriter type sync SDK.
//! Generates `.ts` typed interfaces and discriminated union types, plus sibling `.schema.ts` Zod schemas.
//!
//! ## Type Mappings
//!
//! | Rust Type | TypeScript Type |
//! |-----------|-----------------|
//! | `String` | `string` |
//! | `bool` | `boolean` |
//! | `u8`, `u16`, `i8`, `i16` | `number` |
//! | `u32`, `u64`, `i32`, `i64`, `f32`, `f64` | `number` |
//! | `u64`, `i64`, `u128`, `i128` | `bigint` |
//! | `Uuid` | `string` |
//! | `DateTime<Utc>` | `string` |
//! | `Option<T>` | `T \| undefined` |
//! | `Vec<T>` | `T[]` |
//! | `HashMap<K, V>` | `Record<K, V>` |
//! | Custom struct/enum | Type reference |
//!
//! ## Configuration
//!
//! In `typewriter.toml`:
//!
//! ```toml
//! [typescript]
//! output_dir = "./generated/typescript"
//! file_style = "kebab-case"  # kebab-case, snake_case, or PascalCase
//! readonly = false             # Make all fields readonly
//! zod = true                  # Generate Zod schema files
//! ```
//!
//! ## Example
//!
//! **Rust Input:**
//! ```rust,ignore
//! #[derive(TypeWriter)]
//! #[sync_to(typescript)]
//! pub struct User {
//!     pub id: String,
//!     pub email: String,
//!     pub age: Option<u32>,
//! }
//! ```
//!
//! **Generated TypeScript:**
//! ```typescript
//! export interface User {
//!   id: string;
//!   email: string;
//!   age?: number | undefined;
//! }
//! ```
//!
//! **Generated Zod Schema:**
//! ```typescript
//! import { z } from 'zod';
//!
//! export const UserSchema = z.object({
//!   "id": z.string(),
//!   "email": z.string(),
//!   "age": z.number().optional(),
//! });
//! ```

mod emitter;
mod mapper;

pub use mapper::TypeScriptMapper;
