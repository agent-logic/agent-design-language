# Structured Output Record

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added the recovery-only typed authored-design refresh, full tuple approval and assignment guards, canonical linked-worktree locking, artifact alias/drift rejection, and append-only provenance.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/review.rs
- csdlc-v2/tests/card_identity.rs
- .csdlc/prepared/issues/296/design.md
- .csdlc/prepared/issues/296/diagram.mmd

## Execution

- Add refresh_authored_design_after_recovery as an implemented SPP operation authorized only by current recover_review provenance and cleared downstream truth.
- Atomically refresh SPP/VPP design and diagram digests, invalidate approval, and record prior approval plus old/new tuple provenance.
- Require canonical fresh-session design approval and reject approval rewrites after review or publication authority exists.
- Require assignment to observe the exact approved current design and diagram tuple before and under commit lock.
- Reject pre-existing hardlink aliases and wrong registered-worktree invocation.

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
    "purpose": "Run strict all-target Clippy.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run the complete C-SDLC v2 library unit suite.",
    "outcome": "passed",
    "evidence_ref": "implemented-authored-design-refresh.log"
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
    "purpose": "Run card identity integration proof.",
    "outcome": "passed",
    "evidence_ref": "implemented-authored-design-refresh-linked-worktree.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
