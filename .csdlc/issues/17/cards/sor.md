# Structured Output Record

Template: 1.0.0

Issue: 17

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Doctor now fails closed on repository drift, unroutable Rust module plans, unavailable validator targets, and zero issue-specific validation denominators.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/src/git.rs
- csdlc-v2/tests/gate2.rs

## Execution

- Parse canonical GitHub origin identity and report deterministic repository drift.
- Return specific execution-readiness finding codes for owned paths, Rust module routes, validator availability, and denominator coverage.
- Add an issue-5795-shaped gate2 production fixture and repair the baseline fixture ownership contract.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove repository drift, unroutable module, missing validator, and nonzero issue-specific denominator doctor behavior through the focused gate2 integration target.",
    "outcome": "passed",
    "evidence_ref": "gate2: 1 passed; cargo fmt --check: passed; git diff --check: passed"
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
