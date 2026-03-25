# typewriter-typescript

> TypeScript emitter for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

[![Crates.io](https://img.shields.io/crates/v/typewriter-typescript.svg)](https://crates.io/crates/typewriter-typescript)
[![Docs.rs](https://docs.rs/typewriter-typescript/badge.svg)](https://docs.rs/typewriter-typescript)

## What It Generates

For each Rust type targeting TypeScript, this crate generates:

- `<type>.ts` with static TypeScript interfaces/types
- `<type>.schema.ts` with runtime Zod schemas

| Rust | TypeScript |
|---|---|
| `struct` | `export interface` + `export const NameSchema = z.object(...)` |
| Simple `enum` | String literal union + `z.enum([...])` |
| Tagged `enum` | Discriminated union + `z.discriminatedUnion(...)` |
| `Option<T>` | `field?: T \| undefined` + `.optional()` |
| `Vec<T>` | `T[]` + `z.array(...)` |
| `HashMap<K,V>` | `Record<K, V>` + `z.record(...)` |

## Example Output

```typescript
// user-profile.ts
export interface UserProfile {
  id: string;
  email: string;
  age?: number | undefined;
  tags: string[];
}

// user-profile.schema.ts
import { z } from 'zod';

export const UserProfileSchema = z.object({
  "id": z.string(),
  "email": z.string(),
  "age": z.number().optional(),
  "tags": z.array(z.string()),
});
```

## Runtime Dependency

Generated schema files import from `zod`:

```bash
npm install zod
```

## Usage

Used internally by `typewriter-macros`. Most users should depend on the main [`typewriter`](https://crates.io/crates/typewriter) crate.

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
