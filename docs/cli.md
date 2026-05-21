# CLI Guide

`typebridge` is the primary CLI for project-wide generation, drift checks, watch mode, and diagnostics. The `typewriter` binary remains as a compatibility alias, and `cargo typewriter` remains available via `cargo-typewriter`.

v1.0.0 is intentionally scoped to Rust-source generation from `#[derive(TypeWriter)]` and `#[sync_to(...)]`. It does not include JSON Schema/OpenAPI input adapters, any-to-any conversion, or a stable plugin registry.

## Installation

End users can install prebuilt binaries without installing Rust:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/aarambh-darshan/typewriter/releases/latest/download/typebridge-installer.sh | sh
```

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/aarambh-darshan/typewriter/releases/latest/download/typebridge-installer.ps1 | iex"
```

Rust users can install from crates.io:

```bash
cargo install typebridge-cli
```

## Global Options

- `--config <PATH>`: load a specific `typewriter.toml`
- `--format text|json`: choose human text or structured JSON where supported
- `--verbose`: print extra diagnostic context
- `--dry-run`: report planned writes without changing files for `generate`, `init`, and `check --json-out`
- `--version`: print the CLI version

## Commands

### `typebridge generate`

Generate output files from Rust `#[derive(TypeWriter)]` definitions.

```bash
typebridge generate src/models/user.rs
typebridge generate --all
typebridge generate --all --lang typescript,python
typebridge --dry-run generate --all --diff
typebridge --format json generate --all
```

### `typebridge check`

Compare expected generated output against current files.

```bash
typebridge check
typebridge check --ci
typebridge check --json
typebridge --format json check
typebridge check --json-out reports/drift.json
typebridge check --lang typescript,python
```

`check --ci` exits `1` when drift is detected.

### `typebridge watch`

Watch `.rs` files and regenerate on save.

```bash
typebridge watch
typebridge watch ./src/models
typebridge watch ./src --debounce-ms 75 --lang typescript,python
```

`watch` rejects `--dry-run` and `--format json` because it is a long-running writer.

### `typebridge init`

Create a starter config file.

```bash
typebridge init
typebridge init --force
typebridge --dry-run init
typebridge --config ./config/typewriter.toml init
```

`init` refuses to overwrite an existing file unless `--force` is passed.

### `typebridge doctor`

Print diagnostics for the current project and CLI installation.

```bash
typebridge doctor
typebridge --format json doctor
```

The report includes the CLI version, current directory, discovered project root, config path/status, built-in targets, and experimental plugin status.

### `typebridge plugin`

Plugin commands remain experimental in v1.0.0:

```bash
typebridge plugin list
typebridge plugin validate ./target/release/libtypewriter_plugin_ruby.so
typebridge plugin info ruby
```

There is no `plugin add` command, registry marketplace, or stable community plugin ABI in v1.0.0.

## JSON Report Shape

`typebridge check --json` emits:

- `project_root`: absolute root path
- `generated_at`: unix timestamp string
- `summary`: `{ up_to_date, out_of_sync, missing, orphaned }`
- `entries`: list of per-file results

Each `entries[]` item contains `type_name`, `language`, `output_path`, `status`, and `reason`.

## CI Example

```yaml
name: Type Sync Check

on: [push, pull_request]

jobs:
  check-types:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: typebridge check --ci
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success, help/version output, or no drift for `check --ci` |
| 1 | Runtime error or drift detected in `check --ci` |
| 2 | Invalid CLI usage from Clap parsing |
