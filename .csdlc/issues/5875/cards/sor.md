# Structured Output Record

Template: 1.0.0

Issue: 5875

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded durable distributed migration state machine with authority-verified placement, snapshot transfer, fencing, activation, owner commit, crash reconciliation, and fail-closed restart behavior.

## Artifacts

- adl-runtime/src/distributed/migration.rs
- adl-runtime/tests/distributed_migration.rs
- .csdlc/evidence/5875/execution-proof.json

## Execution

- Added a bounded canonical migration store with external generation-and-digest checkpoint authority and rollback detection.
- Implemented strict prepare, quiesce, checkpoint, transfer, validate, fence, activate, and owner-commit transitions with exact retry semantics.
- Bound placement, snapshot, lease, fencing, quiescence, and isolated-restore evidence to authoritative producers rather than caller assertions.
- Added focused restart, replay, corruption, capacity, abort, fencing, and activation-key regressions.

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
      "distributed_migration",
      "--no-tests=fail"
    ],
    "purpose": "Prove ordered and idempotent migration, restart recovery, authority boundaries, and fail-closed negative paths",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_migration",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings in the exact migration test and implementation surface",
    "outcome": "passed",
    "evidence_ref": "strict-focused-clippy.log"
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
