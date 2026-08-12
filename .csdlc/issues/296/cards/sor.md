# Structured Output Record

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated r4 with swap-aware issue-local authored verification, non-destructive failed-projection quarantine, and durable staged authored copies.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/issues/296

## Execution

- Keep validating retained source bytes and identity while accepting the expected copied inode only when the authored path is inside the swapped issue projection; verify copied canonical bytes and path safety after swap.
- On verifier failure, rename the rejected canonical projection to a fail-closed rollback-preserved quarantine before restoring the backup, preserving externally injected post-swap state without recursive deletion.
- Sync copied authored files, every staged destination directory through the staging root, and the issue parent after swap, quarantine, restore, and backup removal.

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
    "purpose": "Run full Gate 5 review, recovery, issue-local authored refresh and assignment regression proof.",
    "outcome": "passed",
    "evidence_ref": "local:r4-gate5-21"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run full library proof including expected issue-local copy and unrelated post-swap state preservation tests.",
    "outcome": "passed",
    "evidence_ref": "local:r4-lib-81"
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
    "evidence_ref": "local:r4-clippy"
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
