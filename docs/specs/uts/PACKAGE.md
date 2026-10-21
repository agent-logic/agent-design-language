# Versioned UTS package and Runtime consumer

[`adl-uts`](../../../adl-uts/README.md) **0.1.0** owns public Rust declaration
types, validators and bundled schema assets. Consumers import `adl_uts::*`.
The former `adl::uts` re-export and Runtime registry/execution source are no longer
in this public CLI. Runtime consumer qualification belongs to the separately
installed [Runtime owner](../../REPOSITORIES.md).

Package version and schema version are different contracts:

| Wire version | Implemented support in 0.1.0 |
|---|---|
| `uts.v1` | Supported declaration contract; validates the original v1 constraints before additive normalization and validates the resulting v1.1 representation. Side effects, replay safety, idempotence, sensitivity, resources and authentication are preserved. |
| `uts.v1.1` | Canonical declaration constraints, compatible-version list, lifecycle, categories, side-effect tags, observability and planning metadata. Public declaration support does not establish Runtime registry/compiler execution. |
| Anything else, including `0.1.0` | Rejected by the package loader before registry dispatch. |

The normative owner is `agent-logic/universal-tool-schema`. The bundled resource
is `0.1.0-rc.1`, payload revision
`09ec2659c8f82365643341d4f66bac85f1bcaf60`, packaging revision
`4f923dcc2043a7d99572f9d4ff44fd1e4f17925e`, archive SHA-256
`3ce3da2a5bcf7fd8a1cf0bf4ccc00e32012d623c8c9fab72341ddfa3ea786761`.
The [resource pin](../../../adl-uts/resources-pin.json) and
[upstream manifest](../../../adl-uts/resources/RESOURCE_MANIFEST.json) bind the
schemas, specifications, license/NOTICE and canonical corpus. Local historical
proposal documents do not override these pinned contracts.

Lifecycle metadata is descriptive. Optional fields are omitted when absent and
reject explicit null; descriptions count Unicode characters without trimming.
The v1 numeric version/token rules remain stricter than the corresponding v1.1
nonempty string fields. Required-extension metadata is structurally valid;
consumers separately refuse behavior they cannot support.

The embedded invocation schema and canonical corpus support offline structural
validation. They do not implement an invocation transport or establish Runtime
execution qualification. Schema acceptance grants no execution, replay,
sensitive-data or exfiltration authority. ACC policy, registry binding and the
freedom gate remain Runtime owner decisions.

## Install and migrate

Use Rust 1.92 or newer. From a clean checkout of the intended revision:

```sh
cargo package --locked --manifest-path adl-uts/Cargo.toml --target-dir adl-uts/target
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
package assets and compatibility mirrors must match the pinned upstream bytes;
update the reviewed resource pin when adopting a new upstream version rather
than maintaining a separate normative contract.

## Public package proof

```sh
python3 tools/public_adl/verify_install_validator.py --issue 1335
cargo test --locked --manifest-path adl-uts/Cargo.toml
```

Run the verifier from a committed candidate on macOS with `sandbox-exec` and
cached Cargo dependencies. It consumes five extracted public packages with the
network, producer source and private credentials denied. It reconstructs the
33-member UTS resource archive, checks the pinned hash and schema mirrors, and
reports 23 canonical cases separately from 21 historical ADL cases. The canonical
report distinguishes 13 declaration binding/normalization cases from ten
invocation schema-engine cases. The historical suite includes required-extension
admission refusal. No case grants execution authority. Package tests exercise
declaration compatibility; they do not execute Runtime governance. Use [the public package lane](../../../tools/public_adl/run_package_checks.sh)
for the selected public libraries.

Earlier `adl --lib tool_registry`, governed-executor, resident-execution and
`uts_package_runtime` examples belonged to the pre-split Runtime implementation.
They are not public CLI validation commands. Their historical proof remains in
Git and owner custody; current installed-owner qualification must come from the
Runtime owner, with its exact version and evidence.
