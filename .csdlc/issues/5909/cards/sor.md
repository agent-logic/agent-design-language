# Structured Output Record

Template: 1.0.0

Issue: 5909

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Corrected lease mutation authority, bounded lineage state atomically, separated grant and activation transitions, and rebuilt machine-derived proof with exact nine-case deep validation.

## Artifacts

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs
- .csdlc/evidence/5909/review-repair-v3/execution-proof.json
- .csdlc/evidence/5909/operator-repair-v3/negative-cases.json

## Execution

- Require mutation authorization to match the ledger-owned applied log index.
- Reject lineage-count and prospective snapshot capacity overflow before state mutation.
- Enforce distinct initial LeaseGrant and successor Activate semantics.
- Derive and deeply validate nine exact negative cases against executed Rust marker output.

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
    "evidence_ref": ".csdlc/evidence/5909/operator-repair-v3/exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5909/validate-proof-receipt.rb"
    ],
    "purpose": "Deeply bind the exact nine machine-derived cases, producer and stream digests, source artifacts, and two-revision topology.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5909/review-repair-v3/execution-proof.json"
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
    "evidence_ref": ".csdlc/evidence/5909/operator-repair-v3/strict-focused-clippy.log"
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
