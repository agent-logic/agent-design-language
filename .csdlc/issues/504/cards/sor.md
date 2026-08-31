# Structured Output Record

Template: 1.0.0

Issue: 504

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired PR #588 remote-delivery findings and the follow-up cleanup-gate review finding while preserving V3-E as non-authoritative construction work.

## Artifacts

- csdlc-v3/src/commands/remote/mod.rs
- csdlc-v3/src/publication/mod.rs
- csdlc-v3/src/review/mod.rs
- csdlc-v3/tests/remote_commands/remote_delivery.rs
- .csdlc/issues/504

## Execution

- Changed remote delivery input so PVF evidence, review records, publication intents, GitHub readbacks, and cleanup candidates must be wrapped in typed verified observations with source-specific receipts before deliver() can treat them as authority inputs.
- Normalized principal comparison before case-insensitive equality so whitespace variants of the same implementer/reviewer cannot self-authorize publication.
- Allowed Part-Of publication to complete through deliver() as a checkpoint result without terminal cleanup authority.
- Added cleanup removal execution that performs filesystem removal after preview receipt and terminal gates, and distinguishes removed, already-removed, unregistered, path-mismatch, dirty, and live-worktree states.
- Fixed the already-removed cleanup path so it still requires terminal state, terminal receipt, preview receipt, clean worktree, non-live worktree, canonical absolute paths, and removal mode before returning AlreadyRemoved.
- Kept the V3-E implementation construction-only: no v3 live GitHub mutation, v2 retirement, merge, finish, or terminal cleanup authority is claimed.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "remote_commands"
    ],
    "purpose": "Focused V3-E remote workflow regression tests including already-removed cleanup gate refusal.",
    "outcome": "passed",
    "evidence_ref": "local stdout: 15 passed"
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
