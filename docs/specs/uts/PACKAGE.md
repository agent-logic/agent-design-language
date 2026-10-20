# Versioned UTS package and Runtime consumer

[`adl-uts`](../../../adl-uts/README.md) **0.1.0** owns public Rust declaration
types, validators and bundled schema assets. Consumers import `adl_uts::*`.
The former `adl::uts` re-export and Runtime registry/execution source are no longer
in this public CLI. Runtime consumer qualification belongs to the separately
installed [Runtime owner](../../REPOSITORIES.md).

Package version and schema version are different contracts:

| Wire version | Implemented support in 0.1.0 |
|---|---|
| `uts.v1` | Guaranteed declaration baseline; validates before lossless additive normalization to the v1.1 internal representation. Side effects, replay safety, idempotence, sensitivity, resources and authentication are preserved. |
| `uts.v1.1` | Existing declaration types and semantic validation, compatible-version list, categories, side-effect tags, observability and planning metadata. Public declaration support does not establish Runtime registry/compiler execution. |
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
claimed. Public standalone consumers select `adl-uts/Cargo.toml`; the public CLI does not depend on this package.
The automated proof below packages a clean Git snapshot, extracts the artifact,
builds/runs an independent consumer, and records its artifact SHA-256.

Standalone callers use `adl_uts::*`; prior owner-side `adl::uts::*` imports
must follow the Runtime owner migration contract. Deserialize untrusted declarations with `load_tool_declaration`
so version validation precedes normalization. Direct typed construction remains
available for compatibility; validate constructed types before use. Runtime
registry validation also checks programmatically constructed declarations.
Old schema paths in `adl-spec/schemas/uts/` remain compatibility mirrors. The
package assets are canonical and the proof checks byte parity, so edits must
update those mirrors together; they are not separately maintained contracts.

## Public package proof

```sh
python3 adl-uts/tools/verify_install.py
cargo test --locked --manifest-path adl-uts/Cargo.toml
```

The installation verifier checks artifact consumption and schema mirror parity.
Package tests exercise declaration compatibility; they do not execute Runtime
governance. Use [the public package lane](../../../tools/public_adl/run_package_checks.sh)
for the selected public libraries.

Earlier `adl --lib tool_registry`, governed-executor, resident-execution and
`uts_package_runtime` examples belonged to the pre-split Runtime implementation.
They are not public CLI validation commands. Their historical proof remains in
Git and owner custody; current installed-owner qualification must come from the
Runtime owner, with its exact version and evidence.
