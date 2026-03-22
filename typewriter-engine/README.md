# typewriter-engine

> Shared parser, scanner, emitter, and drift-detection engine used by both `typewriter-macros` and `typewriter-cli`.

[![Crates.io](https://img.shields.io/crates/v/typewriter-engine.svg)](https://crates.io/crates/typewriter-engine)
[![Docs.rs](https://docs.rs/typewriter-engine/badge.svg)](https://docs.rs/typewriter-engine)

## What's Inside

- **Parser** — converts `syn::DeriveInput` into typewriter IR (`TypeDef`)
- **Scanner** — discovers `#[derive(TypeWriter)]` items across project files
- **Emitter orchestration** — renders and writes output files for all supported languages
- **Drift detection** — compares expected generated outputs vs disk state for `check --ci`
- **Project helpers** — project root and config (`typewriter.toml`) discovery

## Primary Consumers

- `typewriter-macros` (compile-time generation during `cargo build`)
- `typewriter-cli` (`generate`, `check`, `watch` commands)

## Usage

This crate is primarily internal workspace infrastructure. Most users should use:

- [`typebridge`](https://crates.io/crates/typebridge) for proc-macro-driven generation
- [`typewriter-cli`](https://crates.io/crates/typewriter-cli) for standalone CLI workflows

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
