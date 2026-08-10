# ADR 0065: ACIP Schema Catalog And Governed Projection Boundary

## Status

Status: **Proposed**

## Context

Agent communication must be versioned and inspectable without treating JSON,
WebSocket, or transport metadata as execution authority.

## Decision

ACIP uses versioned protobuf schemas and a public schema catalog. Deterministic
JSON projection is an inspection boundary. Negotiation, correlation, replay,
authority, and semantic bindings fail closed before governed Runtime handling.

## Consequences

Clients can negotiate and inspect ACIP while the Guardian and Runtime retain
admission authority.

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

Human review must separately promote this candidate into `docs/adr/`.
