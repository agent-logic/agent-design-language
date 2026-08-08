# Structured Output Record

Template: 1.0.0

Issue: 17

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved all five exact-head pre-PR review findings without widening issue 17.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/src/git.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/src/doctor.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/tests/gate2.rs

## Execution

- Parse canonical GitHub origin identity and report deterministic repository drift.
- Return specific execution-readiness finding codes for owned paths, Rust module routes, validator availability, and denominator coverage.
- Add an issue-5795-shaped gate2 production fixture and repair the baseline fixture ownership contract.
- Require every declared validator deliverable to exist or have its own selected fail-closed deferred lane.
- Continue execution-readiness diagnosis after repository drift so one doctor report exposes the combined repair surface.
- Make the issue-5795-shaped fixture combine stale repository identity, unroutable module ownership, an unselected missing validator, and an unrelated existing test lane.
- Normalize fail-closed policy semantics and reject explicitly negated policies.
- Classify validator deliverables by test and validation semantics instead of treating every script as a validator.
- Anchor Rust module routing checks to Cargo crate roots and resolve nested parent modules correctly.
- Recognize Cargo manifest equals syntax and package-selected integration tests.
- Add focused positive and negative production-path fixtures for every review finding.

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
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove combined issue-5795 repository and execution findings plus per-required-validator enforcement after bounded review cleanup.",
    "outcome": "passed",
    "evidence_ref": "gate2: 1 passed; cargo fmt --check: passed; git diff --check: passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "purpose": "Prove the issue-5795-shaped false-ready rejection and focused edge cases for fail-closed deferral, product scripts, Cargo-root routing, nested modules, and Cargo target syntax.",
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
