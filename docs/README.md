# 📖 typebridge Documentation

> Comprehensive guides for the typebridge type synchronization SDK.

---

## Guides

| Guide | Description |
|---|---|
| [Getting Started](getting-started.md) | Installation, first struct, build & output |
| [CLI Guide](cli.md) | `typewriter` / `cargo typewriter` commands, watch mode, CI drift checks |
| [Configuration](configuration.md) | `typewriter.toml` options — output dirs, file naming styles, readonly mode, Zod toggles |
| [Type Mappings](type-mappings.md) | Full Rust → TypeScript / Python / Go / Swift / Kotlin / GraphQL / JSON Schema type reference |
| [Serde Compatibility](serde-compatibility.md) | How `#[serde(...)]` attributes are handled |
| [Custom Attributes](custom-attributes.md) | `#[tw(skip)]`, `#[tw(rename)]`, `#[tw(optional)]`, `#[tw(zod)]` reference |
| [TypeScript Emitter](typescript-emitter.md) | Interfaces, discriminated unions, Zod schemas, readonly, file styles |
| [Python Emitter](python-emitter.md) | Pydantic v2 BaseModel, Enum, Union with Literal |
| [Go Emitter](go-emitter.md) | Structs, Interfaces, custom UnmarshalJSON |
| [Swift Emitter](swift-emitter.md) | Codable structs and enums, CodingKeys |
| [Kotlin Emitter](kotlin-emitter.md) | kotlinx.serialization data classes and sealed classes |
| [GraphQL Emitter](graphql-emitter.md) | SDL types, enums, unions, custom scalars |
| [JSON Schema Emitter](json-schema-emitter.md) | Draft 2020-12 object schemas, string enums, oneOf composition |

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
2. **[CLI Guide](cli.md)** — run project-wide generation, drift checks, and watch mode
3. **[Type Mappings](type-mappings.md)** — understand what Rust types become in each language
4. **[Serde Compatibility](serde-compatibility.md)** — leverage your existing serde attributes
5. **[Custom Attributes](custom-attributes.md)** — fine-tune output with `#[tw(...)]`
6. **[Configuration](configuration.md)** — customize output directories and file naming
7. **[TypeScript Emitter](typescript-emitter.md)** / **[Python Emitter](python-emitter.md)** / **[GraphQL Emitter](graphql-emitter.md)** / **[JSON Schema Emitter](json-schema-emitter.md)** — language-specific details
