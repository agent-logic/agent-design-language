# Structured Output Record

Template: 1.0.0

Issue: 201

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the deterministic committed authority protocol with strict current and joint quorum, opaque voter endorsements, canonical time, durable node-local fresh-CAS publication, exact retry, sealed continuity binding, and fail-closed legacy commands.

## Artifacts

- adl-runtime/src/distributed/authority_protocol.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/tests/distributed_authority_protocol.rs
- .csdlc/prepared/issues/201/produce-proof-receipt.rb
- .csdlc/prepared/issues/201/validate-proof-receipt.rb
- .csdlc/evidence/201/v2/execution-proof.json

## Execution

- Added canonical prepare, endorsement, finalization, artifact, membership, time, and quorum verification for committed authority operations.
- Added symlink-safe locked durable result and retry publication through independent node-local checkpoint objects with restart reconciliation and rollback denial.
- Added sealed continuity-transfer binding validation and retired legacy direct authority commands at replicated apply.
- Added and retained the exact ordered 47-case contract, strict Clippy proof, and merge-safe proof validator.

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
    "purpose": "Run the exact focused authority protocol target.",
    "outcome": "passed",
    "evidence_ref": "committed-authority-protocol.log"
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
    "purpose": "Strict lint the bounded authority protocol surface.",
    "outcome": "passed",
    "evidence_ref": "committed-authority-protocol-clippy.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/201/validate-proof-receipt.rb"
    ],
    "purpose": "Run the canonical merge-safe retained proof validator.",
    "outcome": "passed",
    "evidence_ref": "committed-authority-protocol-receipt.log"
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
