# Structured Output Record

Template: 1.0.0

Issue: 202

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and locally proved the authority-sealed one-learner transport, real Quinn/OpenRaft replication path, durable successor flip, and pending-voter exclusion without changing the exact three-voter cut; independent review, publication, merge, and closeout remain pending.

## Artifacts

- adl-runtime/src/distributed/learner_transport.rs
- adl-runtime/src/distributed/learner_transport/tests.rs
- adl-runtime/tests/distributed_authorized_learner_transport.rs
- .csdlc/prepared/issues/202/produce-proof-receipt.rb
- .csdlc/prepared/issues/202/validate-proof-receipt.rb
- .csdlc/evidence/202/v1/execution-proof.json

## Execution

- Added a crate-private adapter that consumes only durably published coarse #201 Membership operations and validates exact canonical EnrollNonVoting or RemoveVoter artifacts.
- Added token-bound learner AppendEntries and InstallSnapshot framing over authenticated Quinn, a real four-PolisRaft add_learner path, and fail-closed denial of vote, client, authority, mutation, renewal, Shepherd, and Observatory operations.
- Added durable single-admission publication, private successor staging and atomic restart-safe flip, exact retry, expiry, capacity, replay, and symlink-safe checkpoint behavior.
- Integrated one opaque pending-exclusion snapshot into #201 endorsement eligibility and ordinary voter session admission and revalidation without changing voter quorum or promotion semantics.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authorized_learner_transport",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy for the owned target.",
    "outcome": "passed",
    "evidence_ref": "authorized-learner-transport-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "learner_transport::tests",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Run the focused sealed-adapter and learner runtime lane.",
    "outcome": "passed",
    "evidence_ref": "authorized-learner-transport-private.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/202/produce-proof-receipt.rb"
    ],
    "purpose": "Run the issue-owned proof producer idempotently.",
    "outcome": "passed",
    "evidence_ref": "authorized-learner-transport-producer.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authorized_learner_transport",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run the public learner-artifact integration lane.",
    "outcome": "passed",
    "evidence_ref": "authorized-learner-transport-public.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/202/validate-proof-receipt.rb"
    ],
    "purpose": "Validate the retained portable proof receipt.",
    "outcome": "passed",
    "evidence_ref": "authorized-learner-transport-receipt.log"
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
