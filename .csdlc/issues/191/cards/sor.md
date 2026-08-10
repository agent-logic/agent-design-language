# Structured Output Record

Template: 1.0.0

Issue: 191

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a secure three-voter OpenRaft substrate with authority-derived topology, mutually authenticated QUIC transport, durable replay and state stores, external rollback checkpoints, bounded recovery, and exact restart behavior.

## Artifacts

- adl-runtime/Cargo.toml
- adl-runtime/Cargo.lock
- adl-runtime/src/distributed/transport.rs
- adl-runtime/src/distributed/polis_runtime.rs
- adl-runtime/tests/distributed_runtime_transport.rs
- .csdlc/prepared/issues/191/produce-proof-receipt.rb
- .csdlc/prepared/issues/191/validate-proof-receipt.rb

## Execution

- Added signed polis session establishment and authority-derived peer routing over the existing authenticated QUIC transport.
- Added crash-reconciled durable OpenRaft vote, log, state-machine, snapshot, and response-cache storage bound to an external monotonic checkpoint authority.
- Added focused three-voter quorum, isolation, restart, rotation, replay, rollback, canonical encoding, path-safety, bounded-I/O, and idle-timeout regressions.
- Added an issue-owned merge-safe receipt producer and validator that bind protected Git source, exact commands, machine cases, and immutable evidence history.

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
      "distributed_runtime_transport",
      "--no-tests=fail"
    ],
    "purpose": "Run the exact focused runtime integration target with a nonzero fourteen-test denominator.",
    "outcome": "passed",
    "evidence_ref": "focused-secure-raft-runtime.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_runtime_transport",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict static analysis over the same focused integration target.",
    "outcome": "passed",
    "evidence_ref": "strict-secure-raft-clippy.log"
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
