# Structured Task Prompt

Template: 1.0.0

Issue: 297

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #297 recovery subsystem and proving tests only; no #296/#294/#291 product mutation, merge, or closeout.

## Deliverables

- Typed read-only classification inventory with exact candidate identities, manifests, and disposition
- Typed immutable-ledger recovery that restores a verified canonical projection and archives rejected evidence
- Separate exact-identity cleanup operation with evidence-preserving receipts
- Crash/restart, collision, symlink, race, CAS, topology, corruption, ambiguity, idempotency, and subsequent-commit proof

## Acceptance

1. AC-1: Classify inventories canonical, backup, preserved, and active attempts without mutation and emits exact identities, manifests, validity, and disposition
2. AC-2: Recover restores exactly one verified prior projection while preserving rejected evidence and append-only recovery provenance
3. AC-3: Collisions, dangling symlinks, hardlink aliases, and post-preflight races cannot install or delete an unauthorized inode
4. AC-4: Recovery is restart-idempotent across every rename and parent-directory fsync boundary
5. AC-5: A complete recovery permits a later ordinary typed commit; incomplete or ambiguous attempts still fail closed
6. AC-6: Cleanup is separate, exact receipt/identity/manifest/CAS/topology guarded, and never removes unclassified or unrelated content
7. AC-7: Corruption, namespace mismatch, stale CAS, wrong worktree/branch, and multiple plausible candidates fail closed without partial mutation
8. AC-8: Existing initialized/ready recovery and issue #291 semantics remain unchanged
9. AC-9: Issue #296 remains blocked until #297 is terminal and ancestral

## Dependencies

- #296 blocked on terminal ancestral #297
- #294 remains blocked through #296

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/bin/csdlc-edit.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/tests/card_identity.rs
- docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md

## Non Goals

- Issue #296, #294, or #291 implementation/card changes
- Automatic evidence deletion or general recursive cleanup
- Lifecycle rollback, phase rewriting, or audit rewriting
- Raw filesystem/manual .csdlc repair
- Merge or closeout
