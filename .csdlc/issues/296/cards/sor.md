# Structured Output Record

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Reconciled the typed implemented authored-design refresh with terminal #297 projection recovery and current main.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/issues/296

## Execution

- Restore implemented authored-design refresh preparation/application helpers after the current-main merge while preserving the public rollback-preserved path and fail-closed classification guard.
- Remove an orphaned Gate 5 merge artifact that left a duplicate fixture helper and restore the intended issue-local authored artifact proof.
- Allow post-recovery plan-summary correction to recognize current structured approve_design audit records and the matching recovery transition after approved intervening repairs.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "purpose": "Run complete Gate 5 regression suite including projection recovery, exact review recovery, and issue-local authored refresh.",
    "outcome": "passed",
    "evidence_ref": "local:r6-gate5-59"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity"
    ],
    "purpose": "Run card identity integration suite including implemented authored-design refresh and linked-worktree coverage.",
    "outcome": "passed",
    "evidence_ref": "local:r6-card-identity-5"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run full library suite including retained authored artifact, rollback-preserved, GitHub, finish, and schema invariants.",
    "outcome": "passed",
    "evidence_ref": "local:r6-lib-86"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove strict all-target lint cleanliness.",
    "outcome": "passed",
    "evidence_ref": "local:r6-clippy"
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
