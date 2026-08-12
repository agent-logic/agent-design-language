# Structured Output Record

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the typed authored-design refresh and fixed r1 final hardlink/ctime revalidation, Git-common serialized registered-worktree review assignment, and nonzero declared focused tests.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/review.rs
- csdlc-v2/tests/card_identity.rs

## Execution

- Reject final-open and final-read artifacts unless nlink remains one and ctime/identity remain stable.
- Serialize review assignment with the Git-common binding lock and reject noncanonical registered worktrees.
- Add the exact declared implemented_authored_design_refresh and linked_worktree filtered integration tests.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity",
      "implemented_authored_design_refresh"
    ],
    "purpose": "Run the exact declared typed operation filter and prove it executes nonzero tests.",
    "outcome": "passed",
    "evidence_ref": "local:r1-remediation"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "card_identity",
      "implemented_authored_design_refresh_linked_worktree"
    ],
    "purpose": "Run the exact declared linked-worktree/schema filter and prove it executes nonzero tests.",
    "outcome": "passed",
    "evidence_ref": "local:r1-remediation"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run the complete library suite including late hardlink and phase/card authority negatives.",
    "outcome": "passed",
    "evidence_ref": "local:r1-remediation"
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
    "evidence_ref": "local:r1-remediation"
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
