# 🔷 GraphQL SDL Emitter

The GraphQL SDL emitter translates Rust data structures into GraphQL Schema Definition Language (SDL) types, enums, and unions.

## File Organization

All generated `.graphql` types are output into a single directory configured via `typewriter.toml` using `output_dir`. Typebridge generates one `.graphql` file per Rust type.

Custom scalar declarations (`DateTime`, `JSON`) are automatically added at the top of each file when the type uses `DateTime<Utc>`, `serde_json::Value`, `HashMap`, or tuple types.

## Structs

Rust `struct`s become GraphQL `type` definitions.

```rust
#[derive(TypeWriter)]
#[sync_to(graphql)]
pub struct UserProfile {
    /// Unique identifier
    pub id: Uuid,
    pub email: String,
    pub age: Option<u32>,
}
```

```graphql
scalar DateTime

"""
UserProfile
"""
type UserProfile {
  """ Unique identifier """
  id: ID!
  email: String!
  age: Int
}
```

**Key behaviors:**
- Non-optional fields get `!` (non-null) suffix
- `Option<T>` fields are nullable (no `!`)
- `Uuid` maps to `ID`, `DateTime` and `JSON` are emitted as custom scalars
- Rust doc comments (`///`) become GraphQL description blocks (`"""`)

## Simple Enums

Enums with only unit variants are generated as GraphQL `enum` types.

```rust
#[derive(TypeWriter)]
#[sync_to(graphql)]
pub enum Priority {
    High,
    Medium,
    Low,
}
```

```graphql
enum Priority {
  High
  Medium
  Low
}
```

## Data-Carrying Enums

GraphQL does not support discriminated unions natively. Typebridge generates a `union` declaration combined with individual `type` definitions for each variant.

```rust
#[derive(TypeWriter)]
#[serde(tag = "type")]
#[sync_to(graphql)]
pub enum Event {
    Click { x: i32, y: i32 },
    Hover,
}
```

```graphql
type EventClick {
  """ Discriminator: always \"Click\" """
  type: String!
  x: Int!
  y: Int!
}

type EventHover {
  """ Discriminator: always \"Hover\" """
  type: String!
}

union Event = EventClick | EventHover
```

Each variant type is named `{EnumName}{VariantName}` (e.g. `EventClick`). For internally tagged enums, a discriminator field is automatically added.

## Enum Representations

Typebridge supports all four serde enum representations for GraphQL:

| Representation | Discriminator in GraphQL |
|---|---|
| `External` | No explicit discriminator field |
| `Internal { tag }` | `tag: String!` field with variant name |
| `Adjacent { tag, content }` | `tag: String!` field, content fields prefixed with content key |
| `Untagged` | No explicit discriminator field |

## Type Mapping

| Rust Type | GraphQL SDL |
|---|---|
| `String` | `String` |
| `bool` | `Boolean` |
| `u8`–`u32`, `i8`–`i32` | `Int` |
| `f32`, `f64` | `Float` |
| `u64`, `u128`, `i64`, `i128` | `String` |
| `Option<T>` | nullable (no `!`) |
| `Vec<T>` | `[T!]` |
| `HashMap<K, V>` | `JSON` (custom scalar) |
| `(A, B)` (tuples) | `JSON` (custom scalar) |
| `Uuid` | `ID` |
| `DateTime<Utc>` | `DateTime` (custom scalar) |
| `NaiveDate` | `DateTime` (custom scalar) |
| `serde_json::Value` | `JSON` (custom scalar) |

## Configuration

Configure the GraphQL emitter in your `typewriter.toml`:

```toml
[graphql]
# Where generated .graphql files are written
# Default: "./generated/graphql"
output_dir = "../schema/types"

# File naming convention for output files
# Default: "snake_case"
# Options: "snake_case", "kebab-case", "PascalCase"
file_style = "snake_case"
```
