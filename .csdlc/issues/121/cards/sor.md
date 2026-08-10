# Structured Output Record

Template: 1.0.0

Issue: 121

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented majority-authorized next-epoch fencing with operation-sensitive possession and restart-safe portable recovery floors.

## Artifacts

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs
- .csdlc/evidence/121/execution-proof.json
- .csdlc/evidence/121/operator-v1/negative-cases.json

## Execution

- Allow majority-endorsed Fence and Revoke operations without cooperation from an unavailable old holder while retaining activation possession for holder-authorized operations.
- Commit Fence at exactly the next epoch and newer applied index, retain portable recovery safety through snapshot/restore, and remove the floor only after delayed valid activation.
- Prove exact 20-test behavior, strict focused Clippy, and eleven machine-derived negative cases through a digest-bound two-revision receipt.

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
    "purpose": "Run the exact issue-owned distributed lease target.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/121/validate-proof-receipt.rb",
      ".csdlc/evidence/121/execution-proof.json"
    ],
    "purpose": "Validate the issue 121 two-revision proof receipt.",
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
