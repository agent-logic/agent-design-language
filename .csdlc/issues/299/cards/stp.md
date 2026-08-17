# Structured Task Prompt

Template: 1.0.0

Issue: 299

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #299 archived-projection cleanup only; no #298 recovery/classification implementation and no unrelated issue mutation.

## Deliverables

- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/tests/archived_projection_cleanup.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-edit.rs
- .csdlc/evidence/299
- Exact-head #119 review and publication-ready PR after gates

## Acceptance

1. AC-1: Cleanup cannot start without terminal+ancestral #298 recovery authority and an exact completed recovery/canonical/archive binding.
2. AC-2: No recursive deletion, symlink following, digest-only ownership, path-authoritative unlink, or removal of an unrecorded inode is possible.
3. AC-3: Regular files, empty directories, root placeholder, per-node tombstones, and type-matched disposal counterparts follow explicit pre/post exchange and identity-specific removal states.
4. AC-4: Restart adopts only exact receipt-owned identities and parent manifests across terminal gate, recovery receipt load, cleanup namespace creation, capture intent, exchange, capture receipt, removal intent, unlink/rmdir, parent fsync, placeholder disposal, final receipt, and completed-repeat boundaries; collisions, replacements, unsupported types, non-empty directories, topology/ownership drift, or third states preserve everything and fail closed.
5. AC-5: Partial cleanup resumes at the first incomplete recorded node; repeating a complete cleanup is idempotent.
6. AC-6: Immutable cleanup ledger and recovery evidence survive successful cleanup.
7. AC-7: Unrelated sentinels and replacement inodes survive every failure and race case.
8. AC-8: Exact-head #119 review has no unresolved actionable finding.

## Dependencies

- #298 terminal and ancestral to #299 execution base
- Completed #298 recovery receipt and verified canonical/archive binding
- Explicit release before touching projection_recovery.rs, store.rs, or gate5.rs

## Inputs

- Live GitHub issue #299 contract
- PR #305 metadata for completed #298 implementation
- .csdlc/prepared/issues/299/design.md
- .csdlc/prepared/issues/299/diagram.mmd

## Non Goals

- Implementing #298 anchored classification or recovery
- General filesystem cleanup
- Mutating #291, #294, #296, or parent #297
- Publication, merge, or closeout during preparation
- Touching #298 worktree or frozen files before explicit release
