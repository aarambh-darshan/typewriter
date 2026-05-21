# CLI Commands

The primary CLI is `typebridge`. The `typewriter` binary is kept as a compatibility alias, and `cargo typewriter` remains available via `cargo-typewriter`.

## Installation

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/aarambh-darshan/typewriter/releases/latest/download/typebridge-installer.sh | sh
```

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/aarambh-darshan/typewriter/releases/latest/download/typebridge-installer.ps1 | iex"
```

## Global Options

- `--config <PATH>`
- `--format text|json`
- `--verbose`
- `--dry-run`
- `--version`

## Commands

### `typebridge generate`

```bash
typebridge generate src/models.rs
typebridge generate --all
typebridge generate --all --lang typescript,python
typebridge generate --all --diff
```

### `typebridge check`

```bash
typebridge check
typebridge check --ci
typebridge check --json
typebridge check --json-out drift-report.json
typebridge check --lang typescript,python
```

### `typebridge watch`

```bash
typebridge watch
typebridge watch src/models/
typebridge watch --lang typescript,python
typebridge watch --debounce-ms 100
```

### `typebridge init`

```bash
typebridge init
typebridge init --force
typebridge --dry-run init
```

### `typebridge doctor`

```bash
typebridge doctor
typebridge --format json doctor
```

### `typebridge plugin`

Plugin commands are experimental in v1.0.0:

```bash
typebridge plugin list
typebridge plugin validate ./target/release/libtypewriter_plugin_ruby.so
typebridge plugin info ruby
```

## Cargo Plugin

```bash
cargo typewriter generate --all
cargo typewriter check --ci
cargo typewriter watch
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Success, help/version output, or no drift for `check --ci` |
| 1 | Runtime error or drift detected |
| 2 | Invalid CLI usage |
