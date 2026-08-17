# Structured Output Record

Template: 1.0.0

Issue: 296

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated C-SDLC standalone CI fixture drift after publication by aligning tests with current canonical design-review and target-directory contracts.

## Artifacts

- csdlc-v2/tests/estimation_contracts.rs
- csdlc-v2/tests/gate10a.rs
- csdlc-v2/tests/gate2.rs
- .csdlc/issues/296

## Execution

- Allow estimation contract fixtures to fall back to the crate target directory when CARGO_TARGET_DIR is absent, matching cargo test's default standalone behavior.
- Update Gate 10A installed-editor fixture to use a canonical fresh-session design reviewer for implemented design reapproval.
- Update Gate 2 design approval fixtures and audit expectations for canonical fresh-session reviewer identity, structured approve_design audit records, prior_design_approval null provenance, and the current single-link authored artifact guard message.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "estimation_contracts"
    ],
    "purpose": "Reproduce and prove the missing CARGO_TARGET_DIR fixture fallback in the failing standalone integration target.",
    "outcome": "passed",
    "evidence_ref": "local:r7-estimation-contracts-11"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "purpose": "Prove installed owner-binary and canonical fresh-session design reapproval fixtures.",
    "outcome": "passed",
    "evidence_ref": "local:r7-gate10a-20"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove initialized/ready design approval and pre-bind contract repair fixtures match current typed audit semantics.",
    "outcome": "passed",
    "evidence_ref": "local:r7-gate2-11"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--",
      "--check"
    ],
    "purpose": "Prove C-SDLC v2 formatting.",
    "outcome": "passed",
    "evidence_ref": "local:r7-fmt"
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
    "evidence_ref": "local:r7-clippy"
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
