# Structured Output Record

Template: 1.0.0

Issue: 225

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added two field-specific semantic editor operations with exact lifecycle, provenance, topology, migration, drift, input, audit, and atomicity guards.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/evidence/225/validation-summary.json

## Execution

- Correct recovered implemented SPP plan_summary only after corresponding typed review recovery.
- Correct initialized or ready unbound SIP operator_constraints without absorbing authored design drift.
- Make retained merge-ready review recovery executable and add complete Gate 2 and Gate 5 regression proof.

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
      "gate2"
    ],
    "purpose": "Prove the complete initialized, ready, bind, and pre-bind correction surface.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/225/validation-summary.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "purpose": "Prove review recovery, provenance, summary correction, audit, and rejection boundaries.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/225/validation-summary.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "--bin",
      "csdlc-edit",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject type, exhaustiveness, and warning regressions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/225/validation-summary.json"
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
    "purpose": "Require canonical Rust formatting.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/225/validation-summary.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch defects.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/225/validation-summary.json"
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
