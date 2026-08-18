# Structured Output Record

Template: 1.0.0

Issue: 358

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Seal Observatory transition action/predecessor and full canonical time.

## Artifacts

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/tests/distributed_observatory_authority_projection.rs

## Execution

- Canonical action/predecessor artifact binding
- Private sealed projection accessors for action and full time
- Focused action/time/mismatch/redaction and durable reopen proof

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
    "purpose": "Prove action shapes, exact nanos ordering, actual durable state reopen with full time retention, mismatch denial and redaction.",
    "outcome": "passed",
    "evidence_ref": "9 passed at source e758b46c3"
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
    "purpose": "Strict Clippy.",
    "outcome": "passed",
    "evidence_ref": "clippy.log"
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
