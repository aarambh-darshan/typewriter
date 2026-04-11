# Plugin System Overview

typewriter v0.5.2 introduces a **dynamic plugin architecture** that allows community-contributed language emitters to be developed as standalone crates.

## How It Works

1. Plugin authors implement the `EmitterPlugin` trait from the `typewriter-plugin` crate
2. Plugins are compiled as shared libraries (`.so` / `.dylib` / `.dll`)
3. The CLI dynamically loads plugins at startup via `libloading`
4. Plugins provide `TypeMapper` implementations just like built-in emitters

## Architecture

```text
┌─────────────────┐       ┌──────────────────┐
│  typewriter-cli │──────▶│  PluginRegistry   │
└─────────────────┘       └────────┬─────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    ▼              ▼              ▼
            ┌──────────┐  ┌──────────┐  ┌──────────┐
            │   Ruby   │  │   PHP    │  │   Dart   │
            │  .so/.dll│  │  .so/.dll│  │  .so/.dll│
            └──────────┘  └──────────┘  └──────────┘
```

## Key Components

| Component | Crate | Purpose |
|-----------|-------|---------|
| `EmitterPlugin` trait | `typewriter-plugin` | Interface for plugin implementations |
| `declare_plugin!` macro | `typewriter-plugin` | Generates C ABI entry points |
| `PluginRegistry` | `typewriter-engine` | Loads and manages plugins |
| `PluginConfig` | `typewriter-plugin` | Plugin-specific config from TOML |

## Bundled Plugins

| Plugin | Language ID | Extension | Description |
|--------|-------------|-----------|-------------|
| `typewriter-plugin-ruby` | `ruby` | `.rbi` | Sorbet type signatures |
| `typewriter-plugin-php` | `php` | `.php` | PHP 8.1+ readonly classes |
| `typewriter-plugin-dart` | `dart` | `.dart` | json_serializable classes |

## Limitations

- **CLI only** — plugins are loaded at CLI startup, not during `cargo build` proc-macro expansion
- **No hot reload** — plugins are loaded once; restart CLI after changes
- **ABI versioning** — plugins must match `PLUGIN_API_VERSION`
