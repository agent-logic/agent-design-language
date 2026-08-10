# Structured Task Prompt

Template: 1.0.0

Issue: 121

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair only quorum fence/revoke possession, next-epoch fencing, and portable restart-floor semantics in the existing lease authority implementation.

## Deliverables

- Operation-sensitive activation possession
- Majority Fence/Revoke without old holder key
- Committed next-epoch durable fence
- Snapshot/restore preserving portable recovery floor
- Focused positive and machine-derived negative tests
- Green reviewed stacked PR

## Acceptance

1. AC-1: Majority Fence and Revoke do not require the old holder activation key while holder-authorized operations still do.
2. AC-2: Fence commits exactly the next epoch and newer applied index; stale, same, and skipped epochs fail atomically.
3. AC-3: Fenced mutation is denied and snapshot/restore at current membership index preserves the fence and portable floor.
4. AC-4: Replacement activation before the portable safety deadline is denied after restart and succeeds only at the safe deadline with a valid new key.
5. AC-5: Exact nonzero tests, strict focused Clippy, machine-derived receipt validation, independent review, and CI pass.

## Dependencies

- agent-logic/agent-design-language PR #120 exact head 91c47ed3ab5ec060cf2ba790d107b1598aa6ba6f
- Legacy WP-04.08 issue #5870 remains unbound until this defect merges

## Inputs

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs
- docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md
- .csdlc/prepared/issues/5870/design.md

## Non Goals

- fencing.rs or distributed_fencing implementation
- Module registration, lib.rs, mod.rs, Cargo, manifest, or lockfile changes
- Sibling distributed child implementation
- Merge authorization
