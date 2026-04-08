# Contributing

## Getting Started

1. Fork the repository
2. Clone your fork
3. Create a branch for your change

```bash
git clone https://github.com/<your-username>/typewriter.git
cd typewriter
git checkout -b feature/my-feature
```

## Development Setup

### Prerequisites

- Rust stable toolchain (1.70+)
- `cargo-insta` for snapshot testing

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

## Project Structure

```
typewriter/
├── typewriter-core/       # IR types, TypeMapper trait, config
├── typewriter-engine/      # Parser, emitter, drift detection
├── typewriter-macros/     # #[derive(TypeWriter)] proc macro
├── typewriter-typescript/  # TypeScript emitter
├── typewriter-python/      # Python emitter
├── typewriter-go/          # Go emitter
├── typewriter-swift/       # Swift emitter
├── typewriter-kotlin/      # Kotlin emitter
├── typewriter-graphql/     # GraphQL SDL emitter
├── typewriter-json-schema/ # JSON Schema emitter
├── typewriter-cli/         # CLI tools
├── typewriter/             # Main crate (typebridge)
└── typewriter-test/        # Snapshot tests
```

## Adding a New Language Emitter

1. Create a new crate `typewriter-<lang>`
2. Implement the `TypeMapper` trait
3. Add to workspace and feature flags
4. Add tests and snapshots

See the main `CONTRIBUTING.md` for detailed instructions.

## Commit Messages

Use conventional commits:

```
feat: add Swift emitter
fix: handle Option<Option<T>> correctly
docs: add Go emitter guide
test: add snapshot tests for enums
```

## Pull Request Process

1. Ensure all tests pass
2. Format your code
3. Run clippy
4. Update documentation
5. Add changelog entry
6. Open PR with description
