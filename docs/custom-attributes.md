# 🏷️ Custom Attributes

typewriter provides `#[tw(...)]` attributes for controlling type generation beyond what `#[serde(...)]` offers.

---

## Per-Type Attributes

Applied to the struct or enum itself.

### `#[sync_to(...)]` — Target Languages

**Required.** Specifies which languages to generate for.

```rust
#[derive(TypeWriter)]
#[sync_to(typescript, python)]
pub struct User { ... }
```

Supported values: `typescript` / `ts`, `python` / `py`, `go` / `golang`, `swift`, `kotlin` / `kt`

---

### `#[tw(rename = "...")]` — Type Renaming

Changes the name of the type in all generated output.

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(rename = "UserDTO")]
pub struct InternalUser { ... }
```

**TypeScript:** `export interface UserDTO { ... }` (not `InternalUser`)

---

### `#[tw(skip)]` — Skip Type Entirely

Excludes this type from all code generation. The derive macro becomes a no-op.

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(skip)]
pub struct InternalOnly { ... }
// Nothing generated
```

---

### `#[tw(readonly)]` — Readonly Fields

Makes all fields `readonly` in TypeScript output.

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(readonly)]
pub struct Config {
    pub name: String,
    pub value: u32,
}
```

**TypeScript:**
```typescript
export interface Config {
  readonly name: string;
  readonly value: number;
}
```

---

### `#[tw(output_dir = "...")]` — Custom Output Directory

Overrides the `typewriter.toml` output directory for this type only.

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(output_dir = "./custom/types")]
pub struct SpecialType { ... }
```

---

## Per-Field Attributes

Applied to individual struct fields.

### `#[tw(skip)]` — Skip Field

Excludes this field from generated output.

```rust
pub struct User {
    pub email: String,
    #[tw(skip)]
    pub password_hash: String,  // NOT in generated types
}
```

---

### `#[tw(rename = "...")]` — Rename Field

Override the field name in generated output. Takes priority over `#[serde(rename)]`.

```rust
pub struct User {
    #[tw(rename = "displayName")]
    pub username: String,
}
```

**TypeScript:** `displayName: string;`

---

### `#[tw(optional)]` — Force Optional

Makes a field optional even if it's not `Option<T>`.

```rust
pub struct User {
    #[tw(optional)]
    pub legacy_field: String,  // not Option<String>, but marked optional
}
```

**TypeScript:** `legacy_field?: string;`
**Python:** `legacy_field: str = None`

---

## Combining Attributes

You can combine multiple `#[tw(...)]` attributes:

```rust
pub struct User {
    #[tw(rename = "userName")]
    pub username: String,

    #[tw(skip)]
    pub internal_id: u64,

    #[tw(optional)]
    pub nickname: String,
}
```

You can also combine `#[tw(...)]` with `#[serde(...)]`:

```rust
#[derive(Serialize, Deserialize, TypeWriter)]
#[serde(rename_all = "camelCase")]
#[sync_to(typescript)]
pub struct ApiResponse {
    #[serde(rename = "statusCode")]
    pub status_code: u32,

    #[tw(skip)]
    pub internal_metadata: String,
}
```

---

## Attribute Priority

When both `#[tw(...)]` and `#[serde(...)]` specify the same thing (e.g., rename), `#[tw]` always wins:

| Conflict | Winner | Reason |
|---|---|---|
| `#[tw(rename = "x")]` vs `#[serde(rename = "y")]` | `#[tw]` → `"x"` | `#[tw]` is typewriter-specific |
| `#[tw(skip)]` vs no serde skip | Field is skipped | `#[tw]` adds behavior |
| No tw skip vs `#[serde(skip)]` | Field is skipped | Serde compat works as fallback |
