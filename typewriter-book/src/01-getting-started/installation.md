# Installation

## Prerequisites

- Rust stable toolchain (1.70+)
- A Cargo project with `serde` for serialization

## Add Dependencies

Add `typebridge` and `serde` to your `Cargo.toml`:

```toml
[dependencies]
typebridge = "0.5.0"
serde = { version = "1", features = ["derive"] }
```

By default, all language emitters are enabled:
- TypeScript
- Python
- Go
- Swift
- Kotlin
- GraphQL
- JSON Schema

## Feature Flags

To reduce compile times, enable only the languages you need:

```toml
# TypeScript only
typebridge = { version = "0.5.0", default-features = false, features = ["typescript"] }

# TypeScript + Python
typebridge = { version = "0.5.0", default-features = false, features = ["typescript", "python"] }

# All languages (default)
typebridge = "0.5.0"
```

## CLI Installation

For project-wide generation, drift checking, and watch mode:

```bash
cargo install typebridge-cli
```

## Next Steps

- [Your First Type](first-steps.md) — Generate your first types
- [Configuration](configuration.md) — Customize output directories and options
