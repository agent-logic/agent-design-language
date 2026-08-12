# Structured Output Record

Template: 1.0.0

Issue: 297

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implement typed evidence-preserving recovery for rollback-preserved issue projections.

## Artifacts

- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs

## Execution

- Added tagged-CAS projection classification and candidate manifests
- Added typed recover and cleanup routes with private receipt namespaces
- Blocked ordinary commits while rollback-preserved evidence exists
- Added CLI/schema/public exports and focused gate5 tests
- Typed tagged-CAS classify/recover/cleanup contracts
- No-follow tree manifest and namespace ownership checks
- Commit blocking and immutable recovery/cleanup receipts
- CLI/schema exports and regression tests

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run complete library unit suite.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-lib.log"
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
    "purpose": "Run strict all-target Clippy.",
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
      "gate5",
      "preserved_projection_recovery"
    ],
    "purpose": "Run focused gate5 recovery tests.",
    "outcome": "passed",
    "evidence_ref": "preserved-projection-recovery.log"
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
