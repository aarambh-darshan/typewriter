# 🤝 Contributing to typewriter

First off, thank you for considering contributing to typewriter! Every contribution matters, whether it's a bug report, feature request, documentation improvement, or code change.

---

## 📋 Table of Contents

- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Project Structure](#project-structure)
- [How to Add a New Language Emitter](#how-to-add-a-new-language-emitter)
- [Code Style](#code-style)
- [Testing](#testing)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)

---

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/<your-username>/typewriter.git
   cd typewriter
   ```
3. **Create a branch** for your change:
   ```bash
   git checkout -b feature/my-feature
   ```

---

## Development Setup

### Prerequisites

- **Rust** — stable toolchain (1.70+)
- **cargo-insta** — for snapshot testing:
  ```bash
  cargo install cargo-insta
  ```

### Build & Test

```bash
# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace

# Run tests and accept new snapshots
cargo insta test --accept --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings
```

---

## Project Structure

```
typewriter/
├── typewriter-core/            ← IR types, TypeMapper trait, config
│   └── src/
│       ├── ir.rs               ← TypeDef, FieldDef, TypeKind, etc.
│       ├── mapper.rs           ← TypeMapper trait
│       └── config.rs           ← typewriter.toml parsing
│
├── typewriter-macros/          ← Proc macro (#[derive(TypeWriter)])
│   └── src/
│       ├── lib.rs              ← Entry point
│       ├── parser.rs           ← syn DeriveInput → IR
│       └── emitter.rs          ← Dispatch to language emitters
│
├── typewriter-typescript/      ← TypeScript emitter
│   └── src/
│       ├── mapper.rs           ← TypeMapper impl for TS
│       └── emitter.rs          ← TS-specific rendering
│
├── typewriter-python/          ← Python emitter
│   └── src/
│       ├── mapper.rs           ← TypeMapper impl for Python
│       └── emitter.rs          ← Python-specific rendering
│
├── typewriter/                 ← Main crate (re-exports)
├── typewriter-test/            ← Snapshot tests
├── example/                    ← Working examples
└── docs/                       ← Documentation guides
```

---

## How to Add a New Language Emitter

Adding a new language (e.g., Go) is straightforward — you only need to implement the `TypeMapper` trait:

### 1. Create the crate

```bash
mkdir typewriter-go
mkdir -p typewriter-go/src
```

### 2. Set up `Cargo.toml`

```toml
[package]
name = "typewriter-go"
version = "0.1.1"
edition = "2021"

[dependencies]
typewriter-core = { path = "../typewriter-core" }
```

### 3. Implement `TypeMapper`

```rust
// typewriter-go/src/mapper.rs
use typewriter_core::ir::*;
use typewriter_core::mapper::TypeMapper;

pub struct GoMapper {
    pub package_name: String,
}

impl TypeMapper for GoMapper {
    fn map_primitive(&self, ty: &PrimitiveType) -> String {
        match ty {
            PrimitiveType::String => "string".to_string(),
            PrimitiveType::Bool => "bool".to_string(),
            PrimitiveType::U32 => "uint32".to_string(),
            // ... map all types
        }
    }

    fn map_option(&self, inner: &TypeKind) -> String {
        format!("*{}", self.map_type(inner))  // Go pointer
    }

    fn emit_struct(&self, def: &StructDef) -> String {
        // Generate Go struct with json tags
        // ...
    }

    // ... implement remaining methods
}
```

### 4. Add to workspace

Add the crate to the workspace `Cargo.toml`:
```toml
members = [
    # ...existing crates...
    "typewriter-go",
]
```

### 5. Wire up in `typewriter-macros`

Add the dependency and feature flag, then add the dispatch case in `emitter.rs`.

### 6. Add tests

Create snapshot tests in `typewriter-test/tests/go_tests.rs` using the same pattern as existing tests.

---

## Code Style

- **Format**: Always run `cargo fmt` before committing
- **Lints**: All code must pass `cargo clippy -- -D warnings`
- **Documentation**: Public items must have doc comments (`///`)
- **Tests**: All new features must include tests
- **Naming**: Follow Rust conventions — `snake_case` for functions, `PascalCase` for types

### Commit Messages

Use conventional commit format:

```
feat: add Go struct emitter with json tags
fix: handle nested Option<Option<T>> correctly
docs: add Go emitter guide to docs/
test: add snapshot tests for Go enum rendering
refactor: extract common naming utilities to core
```

---

## Testing

### Unit Tests

Every crate has its own unit tests in the source files:

```bash
cargo test -p typewriter-core
cargo test -p typewriter-typescript
cargo test -p typewriter-python
```

### Snapshot Tests

We use [insta](https://insta.rs) for snapshot testing. When you add new tests or change output:

```bash
# Run tests (new snapshots will fail)
cargo test -p typewriter-test

# Review and accept new snapshots
cargo insta review

# Or auto-accept all
cargo insta test --accept -p typewriter-test
```

Snapshot files are stored in `typewriter-test/tests/snapshots/` and are committed to git.

### What to Test

For a new emitter, test at minimum:
- Simple struct → generated output
- Struct with `Option`, `Vec`, `HashMap` fields
- Simple enum (all unit variants)
- Tagged enum with data-carrying variants
- Skipped and renamed fields

---

## Pull Request Process

1. **Ensure all tests pass**: `cargo test --workspace`
2. **Format your code**: `cargo fmt --all`
3. **Run clippy**: `cargo clippy --workspace -- -D warnings`
4. **Update documentation** if you changed public APIs
5. **Add a changelog entry** in `CHANGELOG.md` under `[Unreleased]`
6. **Open the PR** with a clear description of what changed and why
7. **Link any related issues** using `Fixes #123` or `Closes #123`

### PR Title Format

```
feat(typescript): add readonly support for interfaces
fix(parser): handle deeply nested generic types
docs: add Swift emitter configuration guide
```

---

## Issue Guidelines

### Bug Reports

Include:
- Rust version (`rustc --version`)
- typewriter version
- Minimal reproduction case (Rust struct/enum that triggers the bug)
- Expected vs actual generated output

### Feature Requests

Include:
- Use case description
- Example Rust input
- Expected generated output for each language

---

## 📜 License

By contributing to typewriter, you agree that your contributions will be licensed under the [Apache License 2.0](LICENSE).

---

<div align="center">

**Thank you for making typewriter better! 🦀**

*Maintained by [Darshan Vichhi](https://github.com/aarambh-darshan)*</div>
