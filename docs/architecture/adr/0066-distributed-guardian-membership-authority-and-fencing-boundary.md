# ADR 0066: Distributed Guardian Membership, Authority, And Fencing Boundary

## Status

Status: **Proposed**

## Context

Distributed execution introduces stale membership, duplicate authority,
partition, lease, relocation, and split-brain risks.

## Decision

Guardian-controlled enrollment, certificate identity, membership epochs,
leases, placement, migration, and fencing determine active authority. A voter
identity has one effective control key and one authoritative activation.
Stale, partitioned, replayed, duplicated, or unfenced actors fail closed.

## Consequences

Distributed placement does not move governance into cognition or transport and
cannot silently create two active owners.

## Alternatives Considered

Peer self-admission, last-writer-wins ownership, and unfenced checkpoint copy
were rejected.

## Source Evidence

- `docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md`
- `adl-runtime/src/distributed/identity.rs`

## Validation Evidence

- `adl-runtime/tests/distributed_guardian.rs`
- `adl-runtime/tests/distributed_membership.rs`
- `adl-runtime/tests/distributed_fencing.rs`
- `adl-runtime/tests/distributed_migration.rs`

## Supersession Relationships

Refines ADR 0011, ADR 0013, and ADR 0054 without replacing Guardian authority.

## Non-Claims

Does not complete constitutional polis governance, global consensus, or
unbounded multi-region operation.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
