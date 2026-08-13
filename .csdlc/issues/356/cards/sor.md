# Structured Output Record

Template: 1.0.0

Issue: 356

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Pre-execution output record.

## Artifacts

- none

## Execution

- none

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_authority_projection",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove all sealed projection accessors, A/B mismatch denial, canonical binding, durable quorum restore rejection, integer bounds, and redaction.",
    "outcome": "passed",
    "evidence_ref": "8 tests passed; 0 failed; 0 ignored at source commit 8ab811777"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_authority_projection",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings in exact accessor source and focused target.",
    "outcome": "passed",
    "evidence_ref": "strict Clippy completed successfully at source commit 8ab811777"
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
