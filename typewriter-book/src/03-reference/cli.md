# CLI Commands

## Installation

```bash
cargo install typebridge-cli
```

## Commands

### `typewriter generate`

Generate type files from Rust source definitions.

```bash
# Generate from a single file
typewriter generate src/models.rs

# Generate from all Rust files
typewriter generate --all

# Generate only specific languages
typewriter generate --all --lang typescript,python

# Show unified diffs for changed files
typewriter generate --all --diff
```

### `typewriter check`

Check if generated files are in sync with Rust source.

```bash
# Check all types
typewriter check

# Exit non-zero on drift (for CI)
typewriter check --ci

# Output as JSON
typewriter check --json

# Write JSON report to file
typewriter check --json-out drift-report.json

# Check specific languages
typewriter check --lang typescript,python
```

### `typewriter watch`

Watch Rust files and regenerate on save.

```bash
# Watch src directory
typewriter watch

# Watch custom path
typewriter watch src/models/

# Specific languages
typewriter watch --lang typescript,python

# Adjust debounce interval (ms)
typewriter watch --debounce-ms 100
```

## Cargo Plugin

After installing, use via cargo:

```bash
cargo typewriter generate --all
cargo typewriter check --ci
cargo typewriter watch
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success (no drift for `check --ci`) |
| 1 | Error or drift detected |
