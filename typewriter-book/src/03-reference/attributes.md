# Attributes Reference

## `#[sync_to(...)]`

Specifies which languages to generate types for:

```rust
#[derive(TypeWriter)]
#[sync_to(typescript, python, go, swift, kotlin, graphql, json_schema)]
pub struct MyType { ... }

// Shorthand aliases work too:
#[sync_to(ts, py, gql)]  // ts=typescript, py=python, gql=graphql
```

**Supported languages:**
- `typescript` / `ts`
- `python` / `py`
- `go` / `golang`
- `swift`
- `kotlin` / `kt`
- `graphql` / `gql`
- `json_schema` / `jsonschema`

## `#[tw(...)]`

Fine-tune the generated output:

### `#[tw(skip)]`

Exclude a field from generated output:

```rust
pub struct User {
    pub id: String,
    #[tw(skip)]          // Not included in generated types
    pub password_hash: String,
}
```

### `#[tw(rename = "...")]`

Override the field/variant name in output:

```rust
pub struct User {
    #[tw(rename = "displayName")]
    pub username: String,
}
```

### `#[tw(optional)]`

Force a field to be optional:

```rust
pub struct Config {
    #[tw(optional)]      // Generated as optional even if not Option<T>
    pub timeout: u32,
}
```

### `#[tw(type = "...")]`

Override the generated type string:

```rust
pub struct Custom {
    #[tw(type = "Record<string, number>")]
    pub metrics: serde_json::Value,
}
```

### `#[tw(zod)]` / `#[tw(zod = false)]`

Control Zod schema generation (TypeScript only):

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(zod)]                    // Enable Zod (default for TypeScript)
pub struct WithZod { ... }

#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(zod = false)]            // Disable Zod
pub struct NoZod { ... }
```

Disable globally in `typewriter.toml`:

```toml
[typescript]
zod = false
```

## Serde Attributes

typewriter automatically reads serde attributes:

```rust
#[derive(Serialize, Deserialize, TypeWriter)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    #[serde(rename = "in_progress")]
    InProgress,
    Completed,
}
```
