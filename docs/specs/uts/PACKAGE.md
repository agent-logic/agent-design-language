# Versioned UTS package and Runtime consumer

`adl-uts` **0.1.0** owns the Rust declaration types, validators and bundled schema
assets. `adl/src/uts.rs` reexports this package to preserve existing `adl::uts`
imports. `adl/src/tool_registry.rs` loads declarations through the package;
`resident_tool_execution.rs` uses the same types in the production Runtime route
through UTS-to-ACC compilation, the freedom gate and the governed executor.

Package version and schema version are different contracts:

| Wire version | Implemented support in 0.1.0 |
|---|---|
| `uts.v1` | Guaranteed declaration baseline; validates before lossless additive normalization to the v1.1 internal representation. Side effects, replay safety, idempotence, sensitivity, resources and authentication are preserved. |
| `uts.v1.1` | Existing declaration types and semantic validation, compatible-version list, categories, side-effect tags, observability and planning metadata. Registry/compiler support is exercised locally. |
| Anything else, including `0.1.0` | Rejected by the package loader before registry dispatch. |

Bundling the v1.1 invocation JSON Schema does **not** implement a new invocation
transport, version-negotiation protocol or all proposed v1.1 runtime semantics.
Runtime continues to use its `ToolProposalV1` and governed receipts. The v1.1
normative document describes a larger target; field presence is not enforcement
proof. Schema acceptance grants no execution, replay, sensitive-data or
exfiltration authority. ACC policy, registry binding and the freedom gate retain
those decisions.

## Install and migrate

Use Rust 1.92 or newer. From a clean checkout of the intended revision:

```sh
cargo package --locked --manifest-path adl-uts/Cargo.toml
mkdir -p /your/isolated/vendor
# Extract target/package/adl-uts-0.1.0.crate into that vendor directory.
tar -xzf adl-uts/target/package/adl-uts-0.1.0.crate -C /your/isolated/vendor
```

In an isolated Rust consumer, add:

```toml
[dependencies]
adl-uts = { version = "=0.1.0", path = "vendor/adl-uts-0.1.0" }
```

Then `cargo build --manifest-path /your/isolated/Cargo.toml`. This is a library;
installation means consuming the packaged artifact through Cargo, not `cargo
install` of an executable. No external registry publication is required or
claimed. `adl/Cargo.toml` pins this initial package version with a local path.
The automated proof below packages a clean Git snapshot, extracts the artifact,
builds/runs an independent consumer, and records its artifact SHA-256.

Existing callers may keep `adl::uts::*`; new standalone callers use
`adl_uts::*`. Deserialize untrusted declarations with `load_tool_declaration`
so version validation precedes normalization. Direct typed construction remains
available for compatibility; validate constructed types before use. Runtime
registry validation also checks programmatically constructed declarations.
Old schema paths in `adl-spec/schemas/uts/` remain compatibility mirrors. The
package assets are canonical and the proof checks byte parity, so edits must
update those mirrors together; they are not separately maintained contracts.

## Required local proof

```sh
python3 adl-uts/tools/verify_install.py
cargo test --locked --manifest-path adl-uts/Cargo.toml
cargo test --locked --manifest-path adl/Cargo.toml --lib uts
cargo test --locked --manifest-path adl/Cargo.toml --lib tool_registry
cargo test --locked --manifest-path adl/Cargo.toml --lib governed_executor
cargo test --locked --manifest-path adl/Cargo.toml --lib resident_tool_execution
cargo test --locked --manifest-path adl/Cargo.toml --lib tick_routes_provider_output_through_runtime_acc_and_adapter
cargo run --locked --manifest-path adl/Cargo.toml --example uts_package_runtime
```

Do not set `ADL_TEST_LIVE_RESIDENT_MODEL` for this local proof. The resident-cycle
test uses a deterministic mock proposal provider and the actual Runtime-owned
observation adapter, with zero paid calls. The example runs the production
governed-dispatch function, records each supported schema version, invocation,
ACC decision and actual read-only adapter result, and verifies that malformed,
unsupported, registry mismatch, policy, authority, gate and replay denial never
call the adapter. It is not a compiler-only fixture and invokes no test-only
executor. Its aggregate snapshot is deliberately synthetic; it proves governed
local tool execution, not a live cloud fleet observation.

New proof surfaces use the required Runtime PVF lane: deterministic bounded local
CPU/disk, no provider/cloud spend. Package tests prove contract compatibility;
the installation script proves artifact consumption and mirror parity; the
example proves actual governed tool execution and denial-before-effect. Existing
conformance and named-consumer tests establish before/after behavior. Hosted CI
is a separate integration result, not implied by local success.
