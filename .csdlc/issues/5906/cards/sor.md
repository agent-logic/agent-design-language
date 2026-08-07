# Structured Output Record

Template: 1.0.0

Issue: 5906

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Retained GitHub mergedAt evidence for closing PR candidates and added fail-closed unique-latest RFC3339 instant selection while preserving single-candidate and exact identity behavior.

## Artifacts

- .csdlc/prepared/issues/5906/design.md
- .csdlc/prepared/issues/5906/diagram.mmd

## Execution

- csdlc-v2/src/github.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/gate_finish.rs

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
      "gate_finish"
    ],
    "purpose": "Prove single-candidate compatibility, unique-latest acceptance, wrong candidate rejection, missing and malformed timestamp rejection, tied-instant rejection, and unchanged finish behavior.",
    "outcome": "passed",
    "evidence_ref": "17 focused gate_finish tests passed; strict all-target Clippy passed with -D warnings; cargo fmt and git diff checks passed."
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
