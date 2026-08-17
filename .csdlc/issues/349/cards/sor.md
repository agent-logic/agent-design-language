# Structured Output Record

Template: 1.0.0

Issue: 349

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Keep exact deferred validator admission coherent across the doctor-advertised ready transition while preserving fail-closed bound execution proof.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/tests/gate2.rs
- .csdlc/evidence/349

## Execution

- Admit the existing exact #79 deferred-target predicate in initialized and ready pre-bind phases only.
- Exercise initialized doctor PASS, advertised advance_ready, unchanged ready doctor PASS and bind, bound missing-target failure, and materialized-target PASS.
- Preserve the existing negative predicate matrix and leave #342 untouched.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict all-target Clippy.",
    "outcome": "passed",
    "evidence_ref": "issue349-clippy.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Run exact diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "issue349-diff.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Run exact C-SDLC v2 format check.",
    "outcome": "passed",
    "evidence_ref": "issue349-format.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "deferred_rust_path_harness_admission_fails_closed_for_each_missing_predicate"
    ],
    "purpose": "Run the exact negative Gate 2 predicate matrix.",
    "outcome": "passed",
    "evidence_ref": "issue349-negative-matrix.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "initialized_deferred_distributed_targets_bind_only_through_exact_path_harnesses"
    ],
    "purpose": "Run the exact positive Gate 2 protocol regression.",
    "outcome": "passed",
    "evidence_ref": "issue349-ready-sequence.log"
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
