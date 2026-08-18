# Structured Output Record

Template: 1.0.0

Issue: 353

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Anchor canonical review equality to the exact live publication metadata head while retaining the original publication revision as the governed-drift lower bound.

## Artifacts

- csdlc-v2/src/finish.rs
- csdlc-v2/tests/gate_finish.rs

## Execution

- Read canonical review authority from expected_head_sha after live PR-head verification.
- Retain publication.revision for metadata-only drift classification and review.reviewed_revision for substantive ancestry.
- Exercise a republish-shaped historical publication anchor without review and a current metadata head containing the canonical completed review.

## Validation

[
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
    "purpose": "Prove strict lint cleanliness.",
    "outcome": "passed",
    "evidence_ref": "issue-353-clippy.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Prove Rust formatting.",
    "outcome": "passed",
    "evidence_ref": "issue-353-format.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "purpose": "Prove publication/review anchor semantics.",
    "outcome": "passed",
    "evidence_ref": "issue-353-gate-finish.log"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
