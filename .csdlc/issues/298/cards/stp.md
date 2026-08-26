# Structured Task Prompt

Template: 1.0.0

Issue: 298

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #298 non-destructive classification and recovery engine only; no cleanup engine, cross-engine proof, sibling mutation, merge, or closeout.

## Deliverables

- Read-only anchored classification with exact candidate/attempt inventory and tagged-CAS disposition
- Immutable hash-linked recovery attempt ledger
- Recovery-owned canonical construction, atomic installation, exact audit binding, and deterministic restart
- Focused production failpoint, identity, collision, topology, idempotency, and later-commit proof

## Acceptance

1. AC-1: Classification is read-only, exact, idempotent, and reports clean, recoverable, already-recovered, ambiguous, or unsafe without trusting invalid embedded state
2. AC-2: Recovery preserves rejected evidence and restores only a verified prior projection through a complete recovery-owned candidate with exact audit provenance
3. AC-3: Tagged CAS, failed-operation lineage, registered topology, issue lock, and per-node mount, identity, ownership, permission, type, and link policy fail closed on drift
4. AC-4: Every recovery receipt, operation-owned temporary-node create and identity bind, content or metadata write, file fsync, parent fsync, no-replace node publish, archive, atomic install, displacement, verification, and final boundary has deterministic same-operation restart or exact fail-closed proof without deletion, replacement, or guessing
5. AC-5: Partial candidates never publish; same-operation replay is idempotent and conflicting operations/collisions preserve evidence and fail closed
6. AC-6: A completed matching recovery permits a later ordinary typed commit while retaining recovery, rejected, and displaced evidence
7. AC-7: Existing initialized and ready recovery plus issue #291 semantics remain unchanged
8. AC-8: Exact-head #119 review has no unresolved actionable finding

## Dependencies

- split_from and part_of #297
- prerequisite for #299 and #300
- #296 remains blocked until terminal parent #297

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md
- parent candidate 2d84616d5b309f0f4bd8d1a21dfc82bf907a8812 as unreviewed input only

## Non Goals

- Archived projection deletion or cleanup placeholders/tombstones
- Exhaustive integrated recovery-plus-cleanup qualification
- Issue #291, #294, #296, #297, #299, or #300 implementation/card changes
- Lifecycle rollback, manual .csdlc repair, merge, or closeout
