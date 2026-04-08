# Serde Compatibility

typewriter automatically reads and respects serde attributes.

## Field Renaming

```rust
#[serde(rename = "userId")]
pub id: String,
```

## Field Skip

```rust
#[serde(skip)]
pub internal_field: String,
```

## Tagged Enums

### Internally Tagged (`tag = "type"`)

```rust
#[derive(TypeWriter)]
#[serde(tag = "type")]
pub enum Event {
    Click { x: u32, y: u32 },
    KeyPress(char),
}
```

**TypeScript:**
```typescript
type Event =
    | { type: "Click"; x: number; y: number }
    | { type: "KeyPress"; value: string };
```

### Adjacently Tagged

```rust
#[derive(TypeWriter)]
#[serde(tag = "event", content = "data")]
pub enum Event {
    Click { x: u32, y: u32 },
}
```

**TypeScript:**
```typescript
type Event =
    | { event: "Click"; data: { x: number; y: number } };
```

### Untagged

```rust
#[derive(TypeWriter)]
#[serde(untagged)]
pub enum Event {
    Click { x: u32, y: u32 },
    String(String),
}
```

**TypeScript:**
```typescript
type Event = { x: number; y: number } | string;
```

## Field Flatten

```rust
#[serde(flatten)]
pub extra: ExtraData,
```

## Rename All Variants

```rust
#[derive(TypeWriter)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    InProgress,
    UserId,
}
```

Generates: `IN_PROGRESS`, `USER_ID` (in appropriate casing for each language).
