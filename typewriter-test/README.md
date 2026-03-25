# typewriter-test

> Integration and snapshot tests for the [typewriter](https://github.com/aarambh-darshan/typewriter) SDK.

## Running Tests

```bash
# Run all snapshot tests
cargo test -p typewriter-test

# Accept new/changed snapshots
cargo insta test --accept -p typewriter-test
```

## Test Coverage

| Language | Tests | What's Covered |
|---|---|---|
| TypeScript | 15 snapshots | Interfaces, enum representations, readonly mode, Zod schema output, generics, self-references/imports |
| Python | 7 snapshots | Simple struct, collections, and all enum representations |
| Go | 7 snapshots | Structs, collections, and all enum representations |
| Swift | 7 snapshots | Structs, collections, and all enum representations |
| Kotlin | 7 snapshots | Structs, collections, and all enum representations |

Snapshots are stored in `tests/snapshots/` and committed to git.

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
