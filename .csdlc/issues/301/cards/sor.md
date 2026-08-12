# Structured Output Record

Template: 1.0.0

Issue: 301

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Bind durable provenance to title-only GitHub issue updates without rewriting issue bodies.

## Artifacts

- csdlc-v2/src/github.rs
- csdlc-v2/tests/gate_github_actions.rs

## Execution

- Canonical operation-key-to-request fingerprint receipts
- Single title-only PATCH with byte-identical body verification
- Idempotent same-fingerprint retry and conflicting same-key rejection
- Fail-closed pre-receipt and final-readback body drift detection
- Body-bearing update compatibility

## Validation

[
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
    "purpose": "Run strict all-target and all-feature Clippy.",
    "outcome": "passed",
    "evidence_ref": "strict-clippy-github-owner.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Run the focused scripted GitHub owner tests.",
    "outcome": "passed",
    "evidence_ref": "title-only-github-issue-update.log"
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
