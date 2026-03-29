# typebridge-cli

> Standalone CLI for typewriter generation, drift checking, and watch mode.

[![Crates.io](https://img.shields.io/crates/v/typebridge-cli.svg)](https://crates.io/crates/typebridge-cli)
[![Docs.rs](https://docs.rs/typebridge-cli/badge.svg)](https://docs.rs/typebridge-cli)

## Commands

| Command | Purpose |
|---|---|
| `typewriter generate <file>` | Generate output for one Rust source file |
| `typewriter generate --all` | Generate output for all project Rust files |
| `typewriter check` | Detect drift between expected and existing generated files |
| `typewriter check --ci` | CI gate: exit non-zero when drift is found |
| `typewriter check --json` | Print structured JSON drift report |
| `typewriter watch [path]` | Watch for `.rs` changes and regenerate incrementally |

## Installation

```bash
cargo install typebridge-cli
```

## Cargo Plugin Usage

`typebridge-cli` also ships `cargo-typewriter`, so you can run:

```bash
cargo typewriter generate --all
cargo typewriter check --ci
```

## License

Apache-2.0 — [Aarambh Dev Hub](https://github.com/aarambh-darshan)
