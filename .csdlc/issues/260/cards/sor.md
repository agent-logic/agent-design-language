# Structured Output Record

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Migrate non-transport distributed Runtime callers to governed authority-store adapters

## Artifacts

- adl-runtime/src/distributed/capability_advertisement.rs
- adl-runtime/src/distributed/migration.rs
- adl-runtime/src/distributed/placement.rs
- adl-runtime/src/distributed/projection.rs
- adl-runtime/src/distributed/recovery.rs
- adl-runtime/src/distributed/resource_weather.rs
- adl-runtime/src/distributed/snapshot_catalog.rs
- adl-runtime/tests/distributed_authority_adapter_callers_260.rs

## Execution

- Require governed certificate adapters for capability-advertisement and snapshot-catalog production verification.
- Require governed certificate authority for resource-weather and placement production reads.
- Require governed certificate, lease, and fencing adapters for production projection reads.
- Route migration and recovery lease/fencing transitions through fail-closed governed adapter helpers while keeping raw seams cfg(test)-only.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_adapter_callers_260",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run the issue-owned exact caller boundary guard.",
    "outcome": "passed",
    "evidence_ref": "authority-adapter-callers-260.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_migration",
      "--test",
      "distributed_recovery",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run focused migration and recovery integration tests.",
    "outcome": "passed",
    "evidence_ref": "migration-recovery.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict library Clippy.",
    "outcome": "passed",
    "evidence_ref": "runtime-lib-clippy.log"
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
