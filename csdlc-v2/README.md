# C-SDLC v2

This is the standalone clean-room C-SDLC v2 workspace. It does not depend on
ADL or Runtime crates and does not reuse their lifecycle implementation,
schemas, templates, tests, fixtures, or skills.

Gate 2 provides the typed lifecycle/card engine, whole-record transactions,
`csdlc-edit`, and the offline read-only `csdlc-doctor`. Later gates add init and
binding, PVF, review truth, publication, and closeout without widening this
core's authority.

## Focused validation

```text
cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check
cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path csdlc-v2/Cargo.toml
```

## Contracts

`csdlc-edit schema` prints the versioned JSON Schema bundle. `csdlc-edit
bootstrap --request <json>` atomically creates an issue record and all six
cards from typed values. `csdlc-edit apply --request <json>` performs one
guarded semantic operation. `csdlc-doctor --repo <path> --issue <n>` emits
stable JSON and performs no network or mutation.

Bootstrap selects a typed planning profile and automatically writes explicit
SPP time/token estimates and VPP time/token budgets. There is no follow-up
manual budget-filling stage.

Markdown files are generated projections. The engine renders deterministic
Markdown from typed values, parses it with `markdown.rs`, validates semantic
anchors, and records values/rendered/AST digests. Direct Markdown edits fail
doctor as corruption.
