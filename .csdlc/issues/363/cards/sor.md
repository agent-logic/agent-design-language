# Structured Output Record

Template: 1.0.0

Issue: 363

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Permit one sequenced Implemented SPP summary correction within a bounded review-recovery epoch.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs

## Execution

- Explicit parsed allowlist for typed intervening repair operations
- Second correction unknown operation and current review/publication fail closed
- Correction audit binds recovery sequence and generation

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
    "purpose": "Strict csdlc-v2 Clippy.",
    "outcome": "passed",
    "evidence_ref": "clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "recovered_issue_can_correct_only_the_spp_plan_summary"
    ],
    "purpose": "Run existing gate5 regression.",
    "outcome": "passed",
    "evidence_ref": "focused-existing.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_plan_summary_recovery_survives_allowed_intervening_repairs"
    ],
    "purpose": "Run new gate5 regression.",
    "outcome": "passed",
    "evidence_ref": "focused-new.log"
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
