# Enums and Unions

typewriter fully supports Rust enums in all their forms.

## Unit Enums

Simple enums with no data:

```rust
#[derive(TypeWriter)]
#[sync_to(typescript, python, go)]
pub enum Role {
    Admin,
    User,
    Guest,
}
```

**TypeScript:**
```typescript
export type Role = "Admin" | "User" | "Guest";
```

**Python:**
```python
class Role(str, Enum):
    ADMIN = "Admin"
    USER = "User"
    GUEST = "Guest"
```

## Tuple Variants

Enums with unnamed fields:

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub enum Result {
    Ok(String),
    Err { message: String },
}
```

**TypeScript:**
```typescript
type Result =
    | { kind: "Ok"; value: string }
    | { kind: "Err"; message: string };
```

## Struct Variants

Enums with named fields:

```rust
#[derive(TypeWriter)]
#[serde(tag = "type")]
#[sync_to(typescript)]
pub enum Event {
    Click { x: u32, y: u32 },
    KeyPress { key: String },
}
```

**TypeScript:**
```typescript
type Event =
    | { type: "Click"; x: number; y: number }
    | { type: "KeyPress"; key: string };
```

## Mixed Variants

```rust
#[derive(TypeWriter)]
#[serde(tag = "type")]
#[sync_to(typescript)]
pub enum Message {
    Text { content: String },
    Data(Vec<u8>),
    Empty,
}
```

**TypeScript:**
```typescript
type Message =
    | { type: "Text"; content: string }
    | { type: "Data"; value: string }
    | { type: "Empty" };
```
