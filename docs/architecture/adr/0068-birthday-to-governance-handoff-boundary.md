# ADR 0068: Birthday-To-Governance Handoff Boundary

## Status

Status: **Deferred**

## Context

v0.92 birthday evidence is intended to feed later governance, but the demo,
review packet, and governance handoff are not yet terminal proof.

## Decision

Defer the handoff architecture until the integrated birthday proof and its
review packet land. The future handoff must preserve evidence lineage and keep
governance authority separate from birthday detection.

## Consequences

Birthday implementation cannot imply citizenship or constitutional standing.

## Alternatives Considered

Treating birth as automatic governance admission was rejected.

## Source Evidence

- `docs/milestones/v0.92/features/FIRST_BIRTHDAY_DEMO_AND_GOVERNANCE_HANDOFF_v0.92.md`

## Validation Evidence

- `adl-runtime-kernel/tests/birthday.rs`

## Supersession Relationships

May later refine ADR 0013, ADR 0014, and ADR 0016.

## Non-Claims

No completed v0.93 governance, citizenship, constitutional admission, or
governance decision is claimed.

## Approval Boundary

Integrated WP-18/WP-19 proof and human review are required before Proposed.
