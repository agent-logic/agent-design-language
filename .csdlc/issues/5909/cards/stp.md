# Structured Task Prompt

Template: 1.0.0

Issue: 5909

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Repair only the four merged WP-04.07 defects in the existing lease authority implementation and its issue-owned tests/proof.

## Deliverables

- Exact ledger-owned applied-index authorization
- Hard-bounded atomic lineage and snapshot state
- Machine-derived exact negative-case evidence
- Distinct LeaseGrant and Activate state transitions
- Fresh exact-head review and green ready corrective PR

## Acceptance

1. AC-1: Future and stale caller indexes fail closed without replay-state mutation.
2. AC-2: Lineage-count and serialized-size capacity failures are deterministic and atomic.
3. AC-3: Executed Rust negative cases have exact denominator, name, result, and digest parity.
4. AC-4: LeaseGrant refuses existing authority and Activate requires prior authority.
5. AC-5: Focused nextest, strict Clippy, exact receipt validation, review, and CI pass.

## Dependencies

- Merged PR agent-logic/agent-design-language#107
- Current origin/main at or after 081988dfe4632e27062f3acc72b7c5d226cd0802

## Inputs

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs
- .csdlc/prepared/issues/5869/design.md
- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md

## Non Goals

- Distributed module registration
- Manifest or lockfile changes
- Umbrella receipt changes
- Unrelated lease redesign or sibling work
