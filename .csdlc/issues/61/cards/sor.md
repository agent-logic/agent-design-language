# Structured Output Record

Template: 1.0.0

Issue: 61

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Correct bind topology scans so retained relative worktree records use canonical repository identity and unrelated historical artifacts are skipped before verification.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate2.rs
- .csdlc/prepared/issues/61/design.md
- .csdlc/prepared/issues/61/diagram.mmd

## Execution

- Resolve stored relative worktrees against the verified primary Git topology root.
- Carry same-issue, same-stored-branch, and same-canonical-worktree predicates through verification and reconciliation.
- Add issue/path context to surviving topology record, card, and artifact errors.
- Add an exact real-binary regression for the #5791-shaped success path and genuine collision controls.

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
    "purpose": "Prove all C-SDLC v2 targets remain warning-clean.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "bind_topology_scan_uses_canonical_record_identity",
      "--",
      "--exact"
    ],
    "purpose": "Prove canonical topology classification, contextual failures, and fail-closed collision behavior.",
    "outcome": "passed",
    "evidence_ref": "gate2-bind-topology-regression.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove issue diff hygiene before exact-head review.",
    "outcome": "passed",
    "evidence_ref": "issue-diff-hygiene.log"
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
