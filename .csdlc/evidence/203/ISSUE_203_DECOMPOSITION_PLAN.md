# Issue #203 decomposition plan

Status: `split_required`

Basis:

- Canonical ADL provider-adapter Gemini review:
  `.csdlc/evidence/203/provider-reviews/gemini-adapter-decomposition-result.json`
- Provider run log:
  `.csdlc/evidence/203/provider-reviews/gemini-adapter-decomposition-run.jsonl`
- Current monolith size against `origin/main` when reviewed:
  `56 files changed, +5157 -477`
- Code-only reviewed delta:
  `+2988 -477`, net `+2511`

Gemini's architecture finding is that #203 currently mixes the critical
security boundary with broad runtime migrations. That makes the raw-store
bypass surface harder to review and makes one monolithic PR review-hostile.

## Delivery plan

### Slice 1: authority store security boundary

Purpose:

- Establish the actual security boundary for raw certificate, lease, and
  fencing stores.
- Seal raw mutation/open paths behind typed authority access tokens.
- Expose a governed published receipt view with the fields needed by the
  serving authority contract.

Paths:

- `adl-runtime/src/distributed/authority_protocol.rs`
- `adl-runtime/src/distributed/authority_store_adapters.rs`
- `adl-runtime/src/distributed/certificates.rs`
- `adl-runtime/src/distributed/fencing.rs`
- `adl-runtime/src/distributed/lease.rs`
- `adl-runtime/src/distributed/mod.rs`
- `adl-runtime/tests/distributed_identity_lease_authority.rs`

Validation:

- `cargo check --manifest-path adl-runtime/Cargo.toml`
- `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- --nocapture --test-threads=1`
- issue-local proof validator if retained proof is updated for this slice

Review focus:

- No production caller can mutate raw stores without typed authority access.
- Test-only fixture access is visibly separated from production access.
- Published receipt view exposes lineage, action class, adapter kind/version,
  generation, canonical result digest, and receipt digest.

### Slice 2: governed transport integration

Purpose:

- Route governed transport certificate/authority flows through the slice 1
  boundary.
- Keep network/runtime transport proof separate from raw-store sealing review.

Paths:

- `adl-runtime/src/distributed/transport/core.rs`
- `adl-runtime/src/distributed/transport/governed/**`
- `adl-runtime/tests/distributed_runtime_transport.rs`
- `adl-runtime/tests/distributed_transport.rs`

Depends on:

- Slice 1

Validation:

- focused runtime transport tests only

### Slice 3: migration, recovery, and peripheral caller migration

Purpose:

- Migrate non-transport distributed runtime callers to the governed adapter
  facade after the boundary is reviewed.

Paths:

- `adl-runtime/src/distributed/migration.rs`
- `adl-runtime/src/distributed/recovery.rs`
- `adl-runtime/src/distributed/authority_reconciliation.rs`
- `adl-runtime/src/distributed/placement.rs`
- `adl-runtime/src/distributed/projection.rs`
- `adl-runtime/src/distributed/resource_weather.rs`
- `adl-runtime/src/distributed/snapshot_catalog.rs`
- `adl-runtime/src/distributed/capability_advertisement.rs`
- corresponding focused distributed tests

Depends on:

- Slice 1
- Slice 2 when transport-specific integration is involved

Validation:

- focused migration/recovery/peripheral tests touched by this slice

### Slice 4: lifecycle/proof truth

Purpose:

- Record final proof and lifecycle evidence after the executable slices are
  stable.
- Avoid masking code review with thousands of generated or rendered lifecycle
  lines.

Paths:

- `.csdlc/issues/203/**`
- `.csdlc/prepared/issues/203/**`
- `.csdlc/evidence/203/**`

Depends on:

- The executable slices being stable enough to state truthful validation and
  review results.

Validation:

- focused proof producer/validator
- typed C-SDLC v2 validation

## Non-claims

- This plan does not publish a PR.
- This plan does not close #203.
- This plan does not create tracker follow-ons by itself.
- The retained Gemini review is advisory evidence, not typed lifecycle
  authority.
