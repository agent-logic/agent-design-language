# ADR 0065: ACIP Schema Catalog And Governed Projection Boundary

## Status

Status: **Proposed**

## Context

The original retained #5832 native receipt was empty, but #283 reconciled that
stale evidence against exact replacement terminal authority from #209 / PR #215.
The replacement local and native validation manifests contain non-empty
machine-readable ACIP outcomes and are now the bounded evidence for review.

## Decision

Propose the ACIP schema catalog and governed projection boundary for human ADR
review. ACIP messages remain versioned, cataloged, deterministically projected,
and bound to runtime authority. The proposal is based on #283's replacement
terminal evidence and does not itself accept the ADR.

## Consequences

The stale #5832 receipt remains historical evidence only. The current review
candidate cites #283/#209 replacement authority and remains outside accepted ADR
authority until a separate human acceptance change promotes it under `docs/adr/`.

## Alternatives Considered

Unversioned JSON-only messages and transport-as-authority were rejected.

## Source Evidence

- `adl-runtime/schemas/acip/v1/acip.proto`
- `adl-runtime/schemas/acip/v1/catalog.json`
- `adl-runtime/src/acip.rs`

## Validation Evidence

- `adl-runtime/tests/acip_version_negotiation.rs`
- `adl-runtime-kernel/tests/production_acip_wss.rs`
- `.csdlc/evidence/283/evidence-manifest.json`
- `.csdlc/evidence/209/local-validation-manifest.json`
- `.csdlc/evidence/209/native-validation-manifest.json`

## Supersession Relationships

Refines ADR 0017 and remains subordinate to ADR 0054 Guardian authority.

## Non-Claims

Does not grant message-content access, bypass permits, or make WebSocket an
authority source.

## Approval Boundary

This record is Proposed from #283 replacement terminal evidence. A separate
human approval change is still required before it can become Accepted.
