# Structured Output Record

Template: 1.0.0

Issue: 301

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented durable title-only issue-update provenance with partial-operation recovery and deterministic remote-drift proof.

## Artifacts

- csdlc-v2/src/github.rs
- csdlc-v2/tests/gate_github_actions.rs

## Execution

- Bind each operation key to a canonical mutation fingerprint in a durable comment receipt.
- Preserve the issue body byte-for-byte through one governed title PATCH.
- Recover a successful PATCH with missing receipt without repeating PATCH.
- Fail closed at before-PATCH, during-PATCH, before-receipt, and after-receipt body drift boundaries.
- Preserve body-bearing marker compatibility and isolate status classification from retrying transport fixtures.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Focused GitHub issue owner proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/301/title-only-github-issue-update.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict warning-free owner and fixture proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/301/strict-clippy-github-owner.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Full library regression proof.",
    "outcome": "passed",
    "evidence_ref": "local output: 75 passed"
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
