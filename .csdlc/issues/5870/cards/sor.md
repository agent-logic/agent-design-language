# Structured Output Record

Template: 1.0.0

Issue: 5870

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented durable quorum fencing with fresh membership verification, strict epochs, restart-safe floors, and fail-closed mutation admission.

## Artifacts

- adl-runtime/src/distributed/fencing.rs
- adl-runtime/tests/distributed_fencing.rs
- .csdlc/evidence/5870/execution-proof.json
- .csdlc/evidence/5870/operator-v2/negative-cases.json

## Execution

- Add an exact Fence/Revoke authority gate that requires a current majority-endorsed certificate and committed membership index.
- Persist bounded fencing floors and replay receipts atomically with component-wise symlink-safe state paths and restart/rollback denial.
- Prove three focused tests, strict Clippy, and sixteen machine-derived negative cases through a digest-bound two-revision receipt.

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
      "distributed_fencing",
      "--no-tests=fail"
    ],
    "purpose": "Run the exact issue-owned distributed fencing target.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5870/validate-proof-receipt.rb",
      ".csdlc/evidence/5870/execution-proof.json"
    ],
    "purpose": "Validate the issue 5870 two-revision proof receipt.",
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
      "distributed_fencing",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict focused Clippy.",
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
