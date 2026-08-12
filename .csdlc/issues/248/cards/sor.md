# Structured Output Record

Template: 1.0.0

Issue: 248

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented deterministic output-limit precedence at the server-owned post-termination arbitration boundary while preserving timeout and cleanup semantics.

## Artifacts

- single focused precedence-and-cleanup pass retained by typed validation; separate 20/20 repetitions passed in the implementation and independent review sessions
- full Runtime kernel suite
- strict all-target Clippy
- Observatory integration proof

## Execution

- adl-runtime-kernel/src/parity.rs
- adl-runtime-kernel/src/bin/adl-runtime-shadow-fixture.rs
- adl-runtime-kernel/tests/parity.rs

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across all Runtime targets.",
    "outcome": "passed",
    "evidence_ref": "clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "parity",
      "process_backend_timeout_and_oversized_file_leave_no_artifacts",
      "--",
      "--exact"
    ],
    "purpose": "Prove ordinary hang yields timeout while a file at the enforced limit followed by a hang yields output_limit and both remove artifacts.",
    "outcome": "passed",
    "evidence_ref": "focused.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observatory"
    ],
    "purpose": "Run the Runtime Observatory integration target.",
    "outcome": "passed",
    "evidence_ref": "observatory.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "purpose": "Run the complete Runtime kernel test suite.",
    "outcome": "passed",
    "evidence_ref": "runtime.log"
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
