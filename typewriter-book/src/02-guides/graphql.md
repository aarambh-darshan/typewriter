# GraphQL

Generates GraphQL Schema Definition Language (SDL) types.

## Type Mappings

| Rust Type | GraphQL Type |
|-----------|--------------|
| `String` | `String` |
| `bool` | `Boolean` |
| `u8`, `u16`, `u32`, `i8`, `i16`, `i32` | `Int` |
| `f32`, `f64` | `Float` |
| `Uuid` | `ID` |
| `DateTime<Utc>` | `DateTime` (custom scalar) |
| `Option<T>` | Nullable (no `!`) |
| `Vec<T>` | `[T!]` |
| `HashMap<K, V>` | `JSON` (custom scalar) |

## Example

**Rust:**
```rust
#[derive(TypeWriter)]
#[sync_to(graphql)]
pub struct User {
    pub id: String,
    pub email: String,
    pub age: Option<u32>,
}
```

**Generated:**
```graphql
scalar DateTime
scalar JSON

type User {
  id: ID!
  email: String!
  age: Int
}
```

## Enums

Simple enums become GraphQL enums:

**Rust:**
```rust
#[derive(TypeWriter)]
#[sync_to(graphql)]
pub enum Role {
    Admin,
    User,
    Guest,
}
```

**GraphQL:**
```graphql
enum Role {
  ADMIN
  USER
  GUEST
}
```

## File Naming

Files use snake_case by default: `UserProfile` → `user_profile.graphql`

```toml
[graphql]
file_style = "kebab-case"  # user-profile.graphql
```
