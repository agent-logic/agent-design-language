# Structured Output Record

Template: 1.0.0

Issue: 5876

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded durable quorum recovery for interrupted relocation with authoritative committed-prefix selection, pre-fence rollback, ambiguity fencing, safe activation, owner commit, and fail-closed restart reconciliation.

## Artifacts

- adl-runtime/src/distributed/recovery.rs
- adl-runtime/tests/distributed_recovery.rs
- .csdlc/evidence/5876/execution-proof.json

## Execution

- Added a canonical bounded recovery store with journaled checkpoint CAS, rollback detection, and bounded durable reads.
- Implemented authority-verified recovery routing for pre-fence rollback, incomplete-target cleanup, ambiguity fencing, activation, and owner commit.
- Persisted pending intent before external side effects and reconciled exact retries within a bounded recovery window.
- Added focused divergence, restart, crash, timeout, capacity, symlink, oversized-input, post-activation fencing, and authority regressions.

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
      "distributed_recovery",
      "--no-tests=fail"
    ],
    "purpose": "Prove authoritative recovery routing, restart reconciliation, fencing, activation, bounds, and fail-closed negative paths",
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
      "distributed_recovery",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings in the exact recovery test and implementation surface",
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
