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

[python]
output_dir = "../api/schemas"
pydantic_v2 = true
```

---

## TypeScript Configuration

```toml
[typescript]
# Where generated .ts files are written
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
```

### File Style Examples

| Style | `UserProfile` | `APIResponse` | `OrderItem` |
|---|---|---|---|
| `kebab-case` | `user-profile.ts` | `api-response.ts` | `order-item.ts` |
| `snake_case` | `user_profile.ts` | `api_response.ts` | `order_item.ts` |
| `PascalCase` | `UserProfile.ts` | `APIResponse.ts` | `OrderItem.ts` |

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

## Default Behavior (No Config File)

If no `typewriter.toml` exists, typewriter uses these defaults:

| Setting | Default |
|---|---|
| TypeScript output dir | `./generated/typescript` |
| TypeScript file style | `kebab-case` |
| TypeScript readonly | `false` |
| Python output dir | `./generated/python` |
| Python pydantic_v2 | `true` |

---

## Config File Discovery

typewriter looks for `typewriter.toml` in this order:

1. The `CARGO_MANIFEST_DIR` of the crate being compiled
2. Parent directories, walking upward (for workspace setups)

This means you can place one `typewriter.toml` at your workspace root and it applies to all crates.

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
