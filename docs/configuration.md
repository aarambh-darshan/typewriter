# ⚙️ Configuration Guide

typewriter is configured via a `typewriter.toml` file at your project root. **All fields are optional** — sensible defaults are used when not specified.

---

## Quick Start

Create a `typewriter.toml` file in your project root:

```toml
[typescript]
output_dir = "../frontend/src/types"
file_style = "kebab-case"
readonly = false
zod = true

[python]
output_dir = "../api/schemas"
pydantic_v2 = true

[go]
output_dir = "../backend/types"
package_name = "api_types"

[graphql]
output_dir = "../schema/types"
file_style = "snake_case"
```

---

## TypeScript Configuration

```toml
[typescript]
# Where generated TypeScript artifacts are written
# - <type>.ts (interfaces/types)
# - <type>.schema.ts (Zod schemas)
# Default: "./generated/typescript"
output_dir = "../frontend/src/types"

# File naming convention for output files
# Options: "kebab-case" (default), "snake_case", "PascalCase"
# kebab-case:  UserProfile → user-profile.ts
# snake_case:  UserProfile → user_profile.ts
# PascalCase:  UserProfile → UserProfile.ts
file_style = "kebab-case"

# If true, all interface fields become readonly
# Default: false
readonly = false

# Toggle sibling Zod schema generation for TypeScript outputs
# Default: true
# Runtime dependency when enabled: npm install zod
zod = true
```

### File Style Examples

| Style | `UserProfile` | `APIResponse` | `OrderItem` |
|---|---|---|---|
| `kebab-case` | `user-profile.ts` | `api-response.ts` | `order-item.ts` |
| `snake_case` | `user_profile.ts` | `api_response.ts` | `order_item.ts` |
| `PascalCase` | `UserProfile.ts` | `APIResponse.ts` | `OrderItem.ts` |

When `zod = true` (default), typewriter also generates a sibling schema file for each type (for example `UserProfile.schema.ts`).

---

## Python Configuration

```toml
[python]
# Where generated .py files are written
# Default: "./generated/python"
output_dir = "../api/schemas"

# Use Pydantic v2 BaseModel
# Default: true
pydantic_v2 = true

# Use @dataclass instead of BaseModel (future)
# Default: false
use_dataclass = false
```

---

## Go Configuration

```toml
[go]
# Where generated .go files are written
# Default: "./generated/go"
output_dir = "../backend/types"

# File naming convention for output files
# Default: "snake_case"
file_style = "snake_case"

# Go package name used inside the generated files
# Default: "types"
package_name = "api_types"
```

---

## GraphQL Configuration

```toml
[graphql]
# Where generated .graphql files are written
# Default: "./generated/graphql"
output_dir = "../schema/types"

# File naming convention for output files
# Options: "snake_case" (default), "kebab-case", "PascalCase"
# snake_case: UserProfile → user_profile.graphql
# kebab-case: UserProfile → user-profile.graphql
# PascalCase: UserProfile → UserProfile.graphql
file_style = "snake_case"
```

---

## Default Behavior (No Config File)

If no `typewriter.toml` exists, typewriter uses these defaults:

| Setting | Default |
|---|---|
| TypeScript output dir | `./generated/typescript` |
| TypeScript file style | `kebab-case` |
| TypeScript readonly | `false` |
| TypeScript Zod schemas | enabled (`<type>.schema.ts`) unless `[typescript].zod = false` |
| Python output dir | `./generated/python` |
| Python pydantic_v2 | `true` |
| Go output dir | `./generated/go` |
| Go package name | `types` |
| Swift output dir | `./generated/swift` |
| Swift file style | `PascalCase` |
| Kotlin output dir | `./generated/kotlin` |
| Kotlin file style | `PascalCase` |
| GraphQL output dir | `./generated/graphql` |
| GraphQL file style | `snake_case` |

---

## Config File Discovery

typewriter looks for `typewriter.toml` in this order:

1. The `CARGO_MANIFEST_DIR` of the crate being compiled
2. Parent directories, walking upward (for workspace setups)

This means you can place one `typewriter.toml` at your workspace root and it applies to all crates.

---

## Type-Level Zod Overrides

Use `#[tw(zod)]` and `#[tw(zod = false)]` on structs/enums to override the global `[typescript].zod` setting for a specific type.

- With global `zod = false`, add `#[tw(zod)]` to generate only selected schema files.
- With global `zod = true`, add `#[tw(zod = false)]` to skip schema generation for a type.

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
pub struct UserProfile {
    pub id: String,
}

#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(zod)]
pub struct Address {
    pub city: String,
}

#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(zod = false)]
pub struct Order {
    pub id: String,
}
```

If `[typescript].zod = false`, only `Address.schema.ts` is generated in this example.

---

## Per-Type Overrides

You can override output directories per type using `#[tw(...)]` attributes:

```rust
#[derive(TypeWriter)]
#[sync_to(typescript)]
#[tw(output_dir = "./custom/types")]  // overrides typewriter.toml for this type only
pub struct SpecialType { ... }
```

See [Custom Attributes](custom-attributes.md) for the full attribute reference.
