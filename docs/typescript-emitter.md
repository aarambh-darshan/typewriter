# 🟦 TypeScript Emitter

Detailed guide for the TypeScript code generation emitter.

---

## Overview

The TypeScript emitter generates `.ts` files containing typed `export interface` declarations for structs and `export type` declarations for enums.

---

## Struct → Interface

Every Rust struct becomes a TypeScript `export interface`:

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub struct Order {
    pub id: String,
    pub total: f64,
    pub items: Vec<OrderItem>,
    pub discount: Option<f64>,
    pub metadata: HashMap<String, String>,
}
```

```typescript
export interface Order {
  id: string;
  total: number;
  items: OrderItem[];
  discount?: number | undefined;
  metadata: Record<string, string>;
}
```

### Key behaviors:

- `Option<T>` fields become **optional**: `field?: T | undefined`
- `Vec<T>` becomes `T[]`
- `HashMap<K, V>` becomes `Record<K, V>`
- Doc comments become JSDoc: `/** ... */`
- File naming: **kebab-case** (configurable)

---

## Simple Enum → String Literal Union

Enums where all variants are unit variants become string literal unions:

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub enum Status {
    Active,
    Inactive,
    Suspended,
}
```

```typescript
export type Status =
  | "Active"
  | "Inactive"
  | "Suspended";
```

---

## Tagged Enum → Discriminated Union

Enums with data and `#[serde(tag = "...")]` become TypeScript discriminated unions:

```rust
#[derive(TypeWriter)]
#[serde(tag = "kind")]
#[sync_to(typescript)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Point,
}
```

```typescript
export type Shape =
  | { kind: "Circle"; radius: number }
  | { kind: "Rectangle"; width: number; height: number }
  | { kind: "Point" };
```

This pattern enables TypeScript's **type narrowing**:

```typescript
function area(shape: Shape): number {
  switch (shape.kind) {
    case "Circle": return Math.PI * shape.radius ** 2;
    case "Rectangle": return shape.width * shape.height;
    case "Point": return 0;
  }
}
```

---

## Readonly Mode

Enable via `typewriter.toml` or `#[tw(readonly)]`:

```typescript
export interface Config {
  readonly name: string;
  readonly value: number;
}
```

---

## File Naming

Default: **kebab-case**

| Rust Type | Output File |
|---|---|
| `UserProfile` | `user-profile.ts` |
| `APIResponse` | `api-response.ts` |
| `OrderItem` | `order-item.ts` |

Configurable in `typewriter.toml`:

```toml
[typescript]
file_style = "PascalCase"  # → UserProfile.ts
```

---

## Output Directory

Default: `./generated/typescript`

Configure globally:
```toml
[typescript]
output_dir = "../frontend/src/types"
```

Or per-type:
```rust
#[tw(output_dir = "./custom/types")]
pub struct SpecialType { ... }
```
