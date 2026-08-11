# ADR 0070: Cross-Polis Continuity Transfer Planning Boundary

## Status

Status: **Proposed**

## Context

Future movement between polis authorities must distinguish continuity from
copying, backup, restore, relocation, and identity fork.

## Decision

The planning boundary requires source and destination authority, continuity
head, evidence manifest, handoff state, ambiguity markers, and rollback intent.
Copied state is never migration proof. Operational migration is deferred until
a separately authorized implementation provides executable transfer,
interruption, replay, privacy, and rollback proof.

## Consequences

v0.92 can preserve a durable design rule without claiming that cross-polis
movement works in production.

## Alternatives Considered

Treating repository, checkpoint, or state copy as continuity transfer was
rejected because it cannot establish authority or a unique continuation.

## Source Evidence

- `docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md`
- `.csdlc/issues/5835/cards/stp.md`

## Validation Evidence

- `adl-runtime/tests/distributed_migration.rs`

## Supersession Relationships

Refines planning around ADR 0013 and ADR 0053 without changing either accepted
decision.

## Non-Claims

No production cross-polis migration, transfer service, global identity
authority, or operational rollback is claimed.

## Approval Boundary

This planning decision still requires human promotion; operational capability
requires a separate implementation ADR and proof.
