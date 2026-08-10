# Structured Output Record

Template: 1.0.0

Issue: 133

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded revisioned redacted snapshots for seven distributed authority stores, with complete deterministic enumeration, content-bound revisions, stable opaque references, durable restart parity for checkpointed authorities, and fail-closed placement unavailability until reconstruction.

## Artifacts

- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/failure_detection.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/fencing.rs
- adl-runtime/src/distributed/placement.rs
- adl-runtime/src/distributed/migration.rs
- adl-runtime/src/distributed/recovery.rs
- adl-runtime/src/distributed/membership.rs
- adl-runtime/tests/distributed_authority_snapshots.rs

## Execution

- Added opaque certificate, failure, lease, fencing, placement, migration, and recovery snapshot APIs owned by their authoritative stores.
- Added deterministic domain-separated references and coarse health, evidence, freshness, and capacity projections without raw identifiers or operational detail.
- Placement retains current decisions in process and returns AuthorityUnavailable after restart until authoritative decisions are reconstructed; migration, recovery, fencing, and lease revisions retain their documented durable behavior.
- Added focused proof for complete enumeration, rotation overlap, explicit unavailable rows, mutation drift, N/N+1 no-partial bounds, replacement/removal, stable reference parity, wrong-domain rejection, and restart behavior.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--locked",
      "--test",
      "distributed_authority_snapshots",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict focused static analysis for the exact integration target.",
    "outcome": "passed",
    "evidence_ref": "authority-snapshot-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--locked",
      "--test",
      "distributed_authority_snapshots"
    ],
    "purpose": "Run the exact focused integration target with a nonzero passing denominator.",
    "outcome": "passed",
    "evidence_ref": "authority-snapshot-tests.log"
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
