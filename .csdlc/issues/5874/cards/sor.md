# Structured Output Record

Template: 1.0.0

Issue: 5874

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented purpose-bound authenticated snapshot catalog entries and transfer manifests with current fenced lease authority, canonical protobuf encoding, redacted projections, bounded content verification, and durable replay denial.

## Artifacts

- adl-runtime/src/distributed/snapshot_catalog.rs
- adl-runtime/tests/distributed_snapshot_catalog.rs
- .csdlc/evidence/5874/execution-proof.json
- .csdlc/evidence/5874/operator-v1/negative-cases.json

## Execution

- Bind signed catalog entries and manifests to trust domain, lineage, source owner and guardian, current epoch and applied authority index, schema, content and chunk digests, byte length, lifetime, encryption context, and intended target.
- Require active fencing authorization and an authorized SnapshotSigning certificate before accepting complete content, while rejecting noncanonical, corrupt, incomplete, expired, wrong-target, stale-authority, and replayed transfers.
- Prove exact 13-test behavior, strict focused Clippy, sixteen machine-derived negative cases, and a digest-bound two-revision receipt.

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
      "distributed_snapshot_catalog",
      "--no-tests=fail"
    ],
    "purpose": "Run the exact issue-owned distributed snapshot catalog target.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5874/validate-proof-receipt.rb"
    ],
    "purpose": "Validate the issue 5874 two-revision proof receipt.",
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
      "distributed_snapshot_catalog",
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
