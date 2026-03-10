# 📖 typebridge Documentation

> Comprehensive guides for the typebridge type synchronization SDK.

---

## Guides

| Guide | Description |
|---|---|
| [Getting Started](getting-started.md) | Installation, first struct, build & output |
| [Configuration](configuration.md) | `typewriter.toml` options — output dirs, file naming styles, readonly mode |
| [Type Mappings](type-mappings.md) | Full Rust → TypeScript / Python type reference |
| [Serde Compatibility](serde-compatibility.md) | How `#[serde(...)]` attributes are handled |
| [Custom Attributes](custom-attributes.md) | `#[tw(skip)]`, `#[tw(rename)]`, `#[tw(optional)]` reference |
| [TypeScript Emitter](typescript-emitter.md) | Interfaces, discriminated unions, readonly, file styles |
| [Python Emitter](python-emitter.md) | Pydantic v2 BaseModel, Enum, Union with Literal |

---

## Quick Links

- 📦 [crates.io/crates/typebridge](https://crates.io/crates/typebridge)
- 📚 [docs.rs/typebridge](https://docs.rs/typebridge)
- 🗺️ [Roadmap](../ROADMAP.md)
- 📝 [Changelog](../CHANGELOG.md)
- 🤝 [Contributing](../CONTRIBUTING.md)

---

## Reading Order

If you're new to typebridge, we recommend reading in this order:

1. **[Getting Started](getting-started.md)** — set up your first project
2. **[Type Mappings](type-mappings.md)** — understand what Rust types become in each language
3. **[Serde Compatibility](serde-compatibility.md)** — leverage your existing serde attributes
4. **[Custom Attributes](custom-attributes.md)** — fine-tune output with `#[tw(...)]`
5. **[Configuration](configuration.md)** — customize output directories and file naming
6. **[TypeScript Emitter](typescript-emitter.md)** / **[Python Emitter](python-emitter.md)** — language-specific details
