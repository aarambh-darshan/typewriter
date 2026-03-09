# 🔗 Serde Compatibility

typewriter **automatically reads `#[serde(...)]` attributes** on your types. You don't need to duplicate them — if serde knows about it, typewriter does too.

---

## Supported Serde Attributes

### `#[serde(rename = "...")]`

Renames a field or variant in all generated output.

```rust
#[derive(Serialize, Deserialize, TypeWriter)]
#[sync_to(typescript)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
}
```

**TypeScript:**
```typescript
export interface User {
  userId: string;  // renamed from user_id
}
```

---

### `#[serde(skip)]` / `#[serde(skip_serializing)]`

Excludes the field from generated output entirely.

```rust
pub struct User {
    pub email: String,
    #[serde(skip)]
    pub password_hash: String,  // NOT in generated types
}
```

**TypeScript:**
```typescript
export interface User {
  email: string;
  // password_hash is NOT here
}
```

---

### `#[serde(tag = "...")]` — Internally Tagged Enums

Creates discriminated union types with a tag field.

```rust
#[derive(Serialize, Deserialize, TypeWriter)]
#[serde(tag = "type")]
#[sync_to(typescript)]
pub enum Event {
    Click { x: u32, y: u32 },
    Scroll { offset: f64 },
    KeyPress { key: String },
}
```

**TypeScript:**
```typescript
export type Event =
  | { type: "Click"; x: number; y: number }
  | { type: "Scroll"; offset: number }
  | { type: "KeyPress"; key: string };
```

**Python:**
```python
class Click(BaseModel):
    type: Literal["Click"] = "Click"
    x: int
    y: int

class Scroll(BaseModel):
    type: Literal["Scroll"] = "Scroll"
    offset: float

class KeyPress(BaseModel):
    type: Literal["KeyPress"] = "KeyPress"
    key: str

Event = Union[Click, Scroll, KeyPress]
```

---

### `#[serde(tag = "...", content = "...")]` — Adjacently Tagged

Tag and content in separate fields.

```rust
#[derive(Serialize, Deserialize, TypeWriter)]
#[serde(tag = "t", content = "c")]
#[sync_to(typescript)]
pub enum Wrapper {
    Text(String),
    Number(u32),
}
```

**TypeScript:**
```typescript
export type Wrapper =
  | { t: "Text"; c: string }
  | { t: "Number"; c: number };
```

---

### `#[serde(untagged)]` — Untagged Enums

No discriminator field — matched by structure.

```rust
#[derive(Serialize, Deserialize, TypeWriter)]
#[serde(untagged)]
#[sync_to(typescript)]
pub enum StringOrNumber {
    Str(String),
    Num(u32),
}
```

---

### `#[serde(flatten)]`

Detected on fields (used for future inline/flatten support).

```rust
pub struct User {
    pub name: String,
    #[serde(flatten)]
    pub metadata: UserMetadata,  // flatten detected
}
```

---

## Priority: `#[tw(...)]` vs `#[serde(...)]`

If both `#[tw(rename = "x")]` and `#[serde(rename = "y")]` are present on the same field, the **`#[tw(...)]` attribute takes priority**. This lets you keep serde behavior for JSON while customizing the generated type output independently.

```rust
pub struct User {
    #[serde(rename = "user_name")]  // for JSON serialization
    #[tw(rename = "userName")]       // for generated TypeScript/Python
    pub username: String,
}
```

In the generated output, the field is named `userName` (from `#[tw]`), not `user_name`.
