# ADR 0066: Distributed Guardian Membership, Authority, And Fencing Boundary

## Status

Status: **Deferred**

## Context

The distributed-runtime library models stale membership, duplicate authority,
partition, lease, relocation, and split-brain risks. Production Guardian/kernel
integration remains the separate open issue #142 boundary.

## Decision

Defer the operational distributed-runtime architecture decision until issue
#142 proves real production Guardian/kernel launch and continuity. The landed
library contract models Guardian-controlled enrollment, certificate identity,
membership epochs, leases, placement, migration records, and fencing, but its
in-process guarantees must not be promoted into a live-polis claim.

## Consequences

The model remains useful bounded evidence, while production quorum, continuity,
partition, migration, recovery, Observatory movement, and shutdown claims stay
with #142.

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

Does not prove production Guardian/kernel launch, authenticated API/WSS
continuity, live partition, migration, recovery, Observatory movement,
continued mutation, bounded shutdown, constitutional polis governance, global
consensus, or unbounded multi-region operation. Those operational claims remain
blocked on open issue #142.

## Approval Boundary

Issue #142 must land and receive exact-head human architecture review before
this record can become Proposed.
