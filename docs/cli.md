# 💻 CLI Guide

Standalone CLI workflows for typewriter without relying on proc-macro build hooks.

---

## Installation

```bash
cargo install typebridge-cli
```

You can run either:

```bash
typewriter <command>
# or
cargo typewriter <command>
```

---

## Commands

### `generate`

Generate output files from Rust `#[derive(TypeWriter)]` definitions.

```bash
# Generate from one file
typewriter generate src/models/user.rs

# Generate all Rust files in the project
typewriter generate --all

# Restrict to languages
typewriter generate --all --lang typescript,python

# Show unified diffs for changed files
typewriter generate --all --diff
```

### `check`

Compare expected generated output against current files.

```bash
# Human-readable drift report
typewriter check

# CI gate (exit code 1 if any drift exists)
typewriter check --ci

# JSON report to stdout
typewriter check --json

# JSON report to file
typewriter check --json --json-out reports/drift.json
```

Drift statuses:

- `up_to_date`
- `out_of_sync`
- `missing`
- `orphaned`

### `watch`

Watch `.rs` files and regenerate on save.

```bash
# Watch ./src by default
typewriter watch

# Watch a custom path
typewriter watch ./src/models

# Adjust debounce and language filters
typewriter watch ./src --debounce-ms 75 --lang typescript,python
```

---

## JSON Report Shape

`typewriter check --json` emits:

- `project_root`: absolute root path
- `generated_at`: unix timestamp (string)
- `summary`: `{ up_to_date, out_of_sync, missing, orphaned }`
- `entries`: list of per-file results

Each `entries[]` item contains:

- `type_name`
- `language`
- `output_path`
- `status`
- `reason`

---

## CI Example

```yaml
# .github/workflows/typewriter-check.yml
name: Type Sync Check

on: [push, pull_request]

jobs:
  check-types:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install typebridge-cli
      - run: typewriter check --ci
```

---

## Notes

- `--lang` filters are intersected with each type's `#[sync_to(...)]` targets.
- `generate --all` recursively scans Rust files while skipping `.git` and `target`.
