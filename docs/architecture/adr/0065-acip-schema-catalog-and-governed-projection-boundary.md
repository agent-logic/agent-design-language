# ADR 0065: ACIP Schema Catalog And Governed Projection Boundary

## Status

Status: **Deferred**

## Context

The ACIP implementation and focused tests are present, but the retained
issue-owned native receipt is empty and #5832 records no machine-readable
validation outcomes.

## Decision

Defer this decision until ACIP validation is rerun at an exact revision and a
non-empty machine-readable receipt proves the versioned schema catalog,
deterministic projection, negotiation, replay, authority, and semantic-binding
contracts. Source and tests alone are insufficient publication authority.

## Consequences

The implementation remains bounded evidence, but this ADR packet does not
promote its unretained execution outcome into a durable architecture claim.

## Alternatives Considered

Unversioned JSON-only messages and transport-as-authority were rejected.

## Source Evidence

- `adl-runtime/schemas/acip/v1/acip.proto`
- `adl-runtime/schemas/acip/v1/catalog.json`
- `adl-runtime/src/acip.rs`

## Validation Evidence

- `adl-runtime/tests/acip_version_negotiation.rs`
- `adl-runtime-kernel/tests/production_acip_wss.rs`
- `.csdlc/evidence/5832/acip-native-receipts.json`

## Supersession Relationships

Refines ADR 0017 and remains subordinate to ADR 0054 Guardian authority.

## Non-Claims

Does not grant message-content access, bypass permits, or make WebSocket an
authority source.

## Approval Boundary

Issue #5832 needs truthful revision-bound validation evidence and human review
before this record can become Proposed.
