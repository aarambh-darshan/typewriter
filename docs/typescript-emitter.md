# 🟦 TypeScript Emitter

Detailed guide for the TypeScript code generation emitter.

---

## Overview

The TypeScript emitter always generates the typed `name.ts` artifact and, when Zod is enabled, also generates a sibling `name.schema.ts` artifact:

- `name.ts` — typed `export interface` / `export type` declarations
- `name.schema.ts` — runtime Zod schemas (`export const NameSchema = ...`) when Zod generation is enabled

Install `zod` in your TypeScript project to use generated schemas at runtime.

Disable schemas globally with `typewriter.toml`:

```toml
[typescript]
zod = false
```

Override per type:

- `#[tw(zod)]` or `#[tw(zod = true)]` forces schema generation for that type.
- `#[tw(zod = false)]` skips schema generation for that type.

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

## Zod Schema Output

When Zod output is enabled, each generated interface/type has a sibling schema file:

- `user-profile.ts`
- `user-profile.schema.ts`

Example schema output:

```typescript
import { z } from 'zod';
import { UserRoleSchema } from './user-role.schema';

export const UserProfileSchema = z.object({
  "id": z.string(),
  "email": z.string(),
  "age": z.number().optional(),
  "role": z.lazy(() => UserRoleSchema),
});
```

### Zod behaviors:

- Primitive mapping: `string`/`boolean`/`number`/`bigint`/`unknown` → `z.string()`/`z.boolean()`/`z.number()`/`z.bigint()`/`z.unknown()`
- `Vec<T>` → `z.array(...)`
- `Tuple` → `z.tuple([...])`
- `HashMap<K, V>` → `z.record(kSchema, vSchema)`
- Named type refs → `z.lazy(() => RefSchema)`
- Generic refs → `z.lazy(() => RefSchema(...))`
- `#[tw(type = "...")]` fields use a broad fallback schema (`z.any()`) because free-form TS strings are not safely parseable to Zod.

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

For the same enum, the schema file emits:

```typescript
export const StatusSchema = z.enum(["Active", "Inactive", "Suspended"]);
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

Schema behavior by enum representation:

- `Internal` / `Adjacent` → `z.discriminatedUnion(...)`
- `External` / `Untagged` → `z.union([...])`

---

## Generic Structs

Generic Rust structs become generic TypeScript interfaces:

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub struct Pagination<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
}
```

```typescript
export interface Pagination<T> {
  items: T[];
  total: bigint;
  page: number;
}
```

The schema file emits a generic schema factory:

```typescript
export const PaginationSchema = <TSchema extends z.ZodTypeAny>(tSchema: TSchema) =>
  z.object({
    "items": z.array(tSchema),
    "total": z.bigint(),
    "page": z.number(),
  });
```

Nested generics still work: `Vec<Pagination<User>>` becomes `Pagination<User>[]` and `PaginationSchema(UserSchema)` wiring in schema output.

---

## Cross-File Imports

When a struct references another custom type, typebridge auto-generates imports.

Type file (`.ts`):

```typescript
import type { FilterUserDto } from './filter-user-dto';
import type { UserRole } from './user-role';
```

Schema file (`.schema.ts`):

```typescript
import { FilterUserDtoSchema } from './filter-user-dto.schema';
import { UserRoleSchema } from './user-role.schema';
```

Self-references are generated as `z.lazy(() => SelfSchema)` without self-imports.

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

When Zod output is enabled, schema files follow the same base naming as type files.

| Rust Type | Type File | Schema File |
|---|---|---|
| `UserProfile` | `user-profile.ts` | `user-profile.schema.ts` |
| `APIResponse` | `api-response.ts` | `api-response.schema.ts` |
| `OrderItem` | `order-item.ts` | `order-item.schema.ts` |

Configurable in `typewriter.toml`:

```toml
[typescript]
file_style = "PascalCase"  # → UserProfile.ts + UserProfile.schema.ts
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
