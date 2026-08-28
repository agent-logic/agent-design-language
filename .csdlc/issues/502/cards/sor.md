# Structured Output Record

Template: 1.0.0

Issue: 502

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the non-authoritative C-SDLC v3 lifecycle-kernel slice for #502, then restacked it onto origin/main after #501 merged, while preserving typed C-SDLC v2 as current lifecycle authority until explicit V3-F cutover.

## Artifacts

- csdlc-v3/AGENTS.md
- csdlc-v3/src/adapters/mod.rs
- csdlc-v3/src/lib.rs
- csdlc-v3/src/lifecycle/mod.rs
- csdlc-v3/src/storage/mod.rs
- csdlc-v3/tests/transactions.rs
- origin/main 1972aa47bd7047b8594a03bf770fb92f7fb63d51
- merge commit 83ee7d8ff189f44b7c4d4c6e82cb410272b97e62
- review-recovery commit 41c2481b15b1276ef95da2965d74d313f481b5e1
- worktree /Volumes/FastWork/adl-worktrees/adl-issue-502-v3-c-csdlc-v3-lifecycle-kernel
- branch codex/502-v3-c-csdlc-v3-lifecycle-kernel
- PR https://github.com/agent-logic/agent-design-language/pull/572

## Execution

- Restacked #502 onto origin/main at merged #501 commit 1972aa47bd7047b8594a03bf770fb92f7fb63d51 via merge commit 83ee7d8ff189f44b7c4d4c6e82cb410272b97e62.
- Recovered stale #502 review/publication truth after the main restack so the post-restack revision must receive fresh exact-head review before publication is trusted.
- Added csdlc-v3/src/lifecycle/mod.rs for explicit capability-checked lifecycle transition decisions, merge-readiness evidence, and projection invalidation semantics.
- Added csdlc-v3/src/storage/mod.rs for deterministic transaction staging, commit-time generation/digest CAS checks, projection-repair fail-closed behavior, recovery classification, audit-provenance preservation, content-bound record digests, and invalidation preservation.
- Added csdlc-v3/src/adapters/mod.rs for argv-only process/Git adapter boundaries, typed status/stdout/stderr outcomes, cancellation/timeout modeling, child credential scope, conservative credential redaction, and shell-executable rejection including path-qualified shell names.
- Added csdlc-v3/tests/transactions.rs covering retained requirements #168, #169, and #170 with transition, transaction, recovery, adapter, commit-CAS, repair-pending, digest-binding, invalidation, merge-readiness, and redaction regression tests.
- Added csdlc-v3/AGENTS.md to preserve the v2-authority boundary and three-minute issue-start expectation for future work in the crate.
- Updated csdlc-v3/src/lib.rs with V3-C module exports and the explicit [168, 169, 170] lifecycle-kernel predecessor denominator.
- Preserved dependency order by waiting for #501 to merge before moving #502's final PR base to main.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--check"
    ],
    "purpose": "format check for the C-SDLC v3 crate after restacking #502 on merged #501 main",
    "outcome": "passed",
    "evidence_ref": "exact-head:83ee7d8ff189f44b7c4d4c6e82cb410272b97e62:passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "purpose": "full local C-SDLC v3 crate test suite after restacking #502 on merged #501 main",
    "outcome": "passed",
    "evidence_ref": "exact-head:83ee7d8ff189f44b7c4d4c6e82cb410272b97e62:4-lib-11-foundation-15-transactions-passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "strict clippy for the C-SDLC v3 crate after main restack",
    "outcome": "passed",
    "evidence_ref": "exact-head:83ee7d8ff189f44b7c4d4c6e82cb410272b97e62:passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "diff hygiene after main restack",
    "outcome": "passed",
    "evidence_ref": "exact-head:83ee7d8ff189f44b7c4d4c6e82cb410272b97e62:passed"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-502-v3-c-csdlc-v3-lifecycle-kernel",
      "issue",
      "--issue",
      "502"
    ],
    "purpose": "typed C-SDLC issue validation after main restack before review recovery",
    "outcome": "blocked",
    "evidence_ref": "exact-head:83ee7d8ff189f44b7c4d4c6e82cb410272b97e62:review_publication_dead_end requiring recover_review"
  },
  {
    "command": [
      "csdlc-review",
      "recover"
    ],
    "purpose": "recover stale review/publication truth caused by the #501 main merge and #502 restack",
    "outcome": "passed",
    "evidence_ref": "generation 13 after recover_review; review_assignment/review/publication cleared before fresh review"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
