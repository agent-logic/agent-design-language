# Structured Output Record

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the deterministic committed authority protocol with strict current and joint quorum, opaque voter endorsements, canonical committed time, monotonic prepare/finalize log ordering, durable node-local fresh-CAS publication, exact retry, sealed continuity binding, and fail-closed legacy commands.

## Artifacts

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/tests/distributed_authority_protocol.rs
- .csdlc/prepared/issues/201/produce-proof-receipt.rb
- .csdlc/prepared/issues/201/validate-proof-receipt.rb
- .csdlc/evidence/201/v5/execution-proof.json

## Execution

- Added canonical prepare, endorsement, finalization, artifact, membership, time, and strict current/joint quorum verification for committed authority operations.
- Added symlink-safe locked durable result and retry publication through independent node-local checkpoint objects with restart reconciliation, rollback denial, and cache-first exact retry.
- Bound distinct operations to strictly increasing committed prepare/finalize log indices while permitting multiple operations under one unchanged membership cut and rejecting reordered finalization.
- Added sealed continuity-transfer binding validation and retired legacy direct authority commands at replicated apply.
- Retained one final exact ordered 47-case proof packet with strict Clippy, merge-safe validation, protected-source drift denial, and clean diff hygiene.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_protocol",
      "--no-tests=fail"
    ],
    "purpose": "Prove the exact nonzero 47-case authority protocol denominator including sequential same-membership operations and reordered-finalize denial.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/201/v5/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_protocol",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove warning-free bounded protocol, persistence, projection, and test surfaces.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/201/v5/execution-proof.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/201/validate-proof-receipt.rb"
    ],
    "purpose": "Prove exact source, command, stream, case-marker, immutable-introduction, protected-drift, and merge-topology bindings.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/201/v5/execution-proof.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Prove final branch diff hygiene after consolidating superseded proof packets.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/201/v5/execution-proof.json"
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
