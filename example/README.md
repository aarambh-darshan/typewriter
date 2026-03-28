# typewriter — Example

> Working examples demonstrating the typewriter SDK.

## Running

```bash
cargo build -p typewriter-example
```

This generates type definitions automatically in:
- `generated/typescript/` — TypeScript interfaces and unions
- `generated/python/` — Pydantic BaseModel classes and enums
- `generated/go/` — Go structs and interface unions
- `generated/graphql/` — GraphQL SDL types, enums, and unions

## Examples Included

| # | Type | Demonstrates |
|---|---|---|
| 1 | `UserProfile` | Simple struct with optional fields and Vec |
| 2 | `ApiResponse` | Struct with various types |
| 3 | `UserRole` | Simple enum → string union / GraphQL enum |
| 4 | `Notification` | Tagged enum → discriminated union / GraphQL union |
| 5 | `UserRecord` | `#[serde(skip)]` + `#[tw(skip)]` field exclusion |
| 6 | `Pagination<T>` | Generic struct → generic interface / GraphQL type |

## License

Apache-2.0 — [Darshan Vichhi](https://github.com/aarambh-darshan)
