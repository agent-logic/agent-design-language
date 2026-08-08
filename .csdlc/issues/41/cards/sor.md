# Structured Output Record

Template: 1.0.0

Issue: 41

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added explicit IssueRead-only structured Octocrab failure classification with stable exit codes, bounded diagnostics, and real-binary loopback regression proof while preserving shared non-read behavior.

## Artifacts

- .csdlc/prepared/issues/41/design.md
- .csdlc/prepared/issues/41/diagram.mmd

## Execution

- csdlc-v2/src/error.rs
- csdlc-v2/src/github.rs
- csdlc-v2/tests/gate_github_actions.rs

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "purpose": "Prove typed issue-read failure taxonomy, stable JSON and exits, redaction, successful reads, and unchanged non-read readback classification",
    "outcome": "passed",
    "evidence_ref": "local issue worktree: gate_github_actions 5 passed, 0 failed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the complete C-SDLC v2 target set remains warning-free after the classifier change",
    "outcome": "passed",
    "evidence_ref": "local issue worktree: strict all-target Clippy, cargo fmt --check, and git diff --check passed"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
