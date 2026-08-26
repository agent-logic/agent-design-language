# Runtime v2/v3 Authority Topology

This document is the DEC-01 authority contract for v0.92.1. It separates Runtime v2 and Runtime v3 ownership without deleting Runtime v2, making Runtime v3 the default, or admitting Runtime v4; Runtime v4 is excluded.

## Source Authority

| Surface | Owner | Disposition |
| --- | --- | --- |
| `adl/src/runtime_v2/**` | Runtime v2 | Authoritative Runtime v2 source, compatibility, fixtures, and tests. |
| `adl-runtime/**` | Runtime v3 guardian/runtime | Authoritative Runtime v3 guardian and outer runtime source. |
| `adl-runtime-kernel/**` | Runtime v3 kernel | Authoritative Runtime v3 kernel source, tests, durable-state, control, and proof contracts. |
| `docs/runtime/**` | Shared runtime documentation | Human-readable topology and transition records; no runtime authority by itself. |
| `docs/milestones/v0.92.1/evidence/runtime-decoupling/**` | DEC-01 evidence | Machine-readable topology and executable validation; Runtime v4 authority is excluded. |

## Reverse-Reference Dispositions

Runtime v2 and Runtime v3 may mention each other only through declared dispositions:

- `runtime-v2-source`: Runtime v2 internal source, tests, fixtures, or compatibility records.
- `runtime-v2-to-v3-compatibility-bridge`: the explicit `reasoning_runtime_bridge` compatibility bridge from Runtime v2 into the existing `adl_runtime::reasoning_runtime` surface.
- `runtime-v3-source`: Runtime v3 guardian or kernel source.
- `runtime-v3-compatibility-metadata`: Runtime v3 records historical Runtime v2 schema/source references as metadata without acquiring Runtime v2 authority.
- `runtime-v3-proof`: Runtime v3 tests and parity contracts that prove Runtime v2 is preserved, not deleted, and not silently reused.
- `runtime-docs`: milestone and runtime documentation references.
- `dec-01-evidence`: DEC-01 manifest, validator, and lifecycle evidence.

Any reverse reference outside those dispositions fails the DEC-01 validator.

## Compatibility

Supported compatibility is proven by the focused Runtime v3 contract test:

```text
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test contracts parity_baseline_manifest_is_a_captured_inventory_not_a_live_repo_dependency
```

That test keeps the existing Runtime v3 baseline inventory as a captured compatibility record rather than a live source dependency. DEC-01 adds a wrapper validator that executes this proof and then validates the live topology manifest.

## Migration Contract

Migration is a dry-run contract for consumers that need to select a runtime generation:

1. Identify the source reference through the manifest.
2. Confirm the reference has exactly one owner and disposition.
3. Preserve Runtime v2 behavior unless the disposition explicitly says the consumer is Runtime v3-owned.
4. Move documentation or consumer metadata only after the validator passes.
5. Stop for replanning if Runtime v4 is required because Runtime v4 is excluded.

The executable dry-run is:

```text
bash docs/milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh --migration-dry-run
```

## Rollback Contract

Rollback is equally explicit:

1. Runtime v2 remains under `adl/src/runtime_v2/**`.
2. Runtime v3 remains under `adl-runtime/**` and `adl-runtime-kernel/**`.
3. Shared docs and DEC-01 evidence can be reverted without changing either runtime's source owner.
4. Runtime v4 remains excluded.

The executable dry-run is:

```text
bash docs/milestones/v0.92.1/evidence/runtime-decoupling/validate-runtime-authority-topology.sh --rollback-dry-run
```

## Stop Conditions

- A supported consumer is unclassified.
- Runtime v2 or Runtime v3 silently acquires the other's authority.
- Runtime v4 becomes necessary despite the explicit Runtime v4 excluded boundary.
- Migration or rollback cannot be executed as dry-run proof.
