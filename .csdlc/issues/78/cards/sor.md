# Structured Output Record

Template: 1.0.0

Issue: 78

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added a typed, recovery-provenance-gated operation that corrects only STP deliverables after review recovery, rejects malformed or stale requests, and retains exact previous and replacement values in atomic audit evidence.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/prepared/issues/78/design.md
- .csdlc/prepared/issues/78/diagram.mmd

## Execution

- Added correct_stp_deliverables_after_recovery to the typed semantic operation schema and STP projection application path.
- Required the latest review-control audit event to be recover_review and all review, publication, readiness, and terminal authority to be cleared.
- Rejected empty, blank, and trim-normalized duplicate deliverables while preserving every unrelated STP field.
- Recorded exact previous and replacement collections in structured audit evidence.
- Added positive and negative integration tests for recovery provenance, phase and card authorization, stale CAS, malformed input, atomic field preservation, structured audit evidence, and projection drift.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--target-dir",
      "../adl-builds/78/csdlc-v2-target",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy across all C-SDLC v2 targets.",
    "outcome": "passed",
    "evidence_ref": "issue-78-clippy.log"
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
    "purpose": "Run the canonical Rust formatter check.",
    "outcome": "passed",
    "evidence_ref": "issue-78-fmt.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--target-dir",
      "../adl-builds/78/csdlc-v2-target",
      "--test",
      "gate5"
    ],
    "purpose": "Run the complete C-SDLC v2 review and recovery integration gate containing issue #78 proof.",
    "outcome": "passed",
    "evidence_ref": "issue-78-gate5.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "gate5"
    ],
    "purpose": "Run the complete review-recovery gate after adding stale-digest and unchanged-record proof.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/78/review-remediation-gate5.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove warning-free production and test targets after review remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/78/review-remediation-clippy.log"
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
    "purpose": "Prove canonical Rust formatting after review remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/78/review-remediation-fmt.log"
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
