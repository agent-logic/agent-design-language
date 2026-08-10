# Structured Output Record

Template: 1.0.0

Issue: 5909

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Corrected lease mutation authority, bounded lineage state atomically, separated grant and activation transitions, and retained machine-derived proof.

## Artifacts

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs
- .csdlc/evidence/5909/execution-proof.json
- .csdlc/evidence/5909/operator-repair-v1/negative-cases.json

## Execution

- Require mutation authorization to match the ledger-owned applied log index.
- Reject lineage-count and prospective snapshot capacity overflow before state mutation.
- Enforce distinct initial LeaseGrant and successor Activate semantics.
- Derive nine exact negative cases from executed Rust test output.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_lease",
      "--no-tests=fail"
    ],
    "purpose": "Run the exact distributed lease target.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5909/validate-proof-receipt.rb"
    ],
    "purpose": "Validate the retained WP-04.07 two-revision receipt.",
    "outcome": "passed",
    "evidence_ref": "exact-revision-proof-receipt.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_lease",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy for the exact distributed lease test surface.",
    "outcome": "passed",
    "evidence_ref": "strict-focused-clippy.log"
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
