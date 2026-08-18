# Structured Output Record

Template: 1.0.0

Issue: 369

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Add exact CAS-guarded recovery for falsely recorded design approval on bound or implemented issues without granting replacement authority.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-edit.rs
- csdlc-v2/tests/gate2.rs

## Execution

- Add typed recover-design-review request, schema, library export, and csdlc-edit dispatch
- Require exact phase, generation, digest, prior reviewer/revision, and identical false reviewer
- Reject later lifecycle authority, wrong topology, empty provenance, and repeat recovery
- Preserve topology and audit history, append explicit correction, and set only current design review pending
- Prove bound, implemented, refusal, and current #275 gen21-shaped recovery with four literal exact tests

## Validation

[
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
    "purpose": "Run strict all-target Clippy.",
    "outcome": "passed",
    "evidence_ref": "clippy.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/369/run_exact_focused_matrix.py"
    ],
    "purpose": "Run the issue-owned exact focused wrapper.",
    "outcome": "passed",
    "evidence_ref": "focused-exact.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/369/validate_exact_scope.py"
    ],
    "purpose": "Run exact scope validator.",
    "outcome": "passed",
    "evidence_ref": "scope-exact.log"
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
