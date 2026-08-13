# Structured Output Record

Template: 1.0.0

Issue: 358

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Seal Observatory transition action/predecessor and full canonical time.

## Artifacts

- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/tests/distributed_observatory_authority_projection.rs

## Execution

- Canonical action/predecessor artifact binding
- Private sealed projection accessors for action and full time
- Focused action/time/mismatch/redaction proof

## Validation

[
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
  },
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
    "purpose": "Run projection target.",
    "outcome": "passed",
    "evidence_ref": "focused.log"
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
