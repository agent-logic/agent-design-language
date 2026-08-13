# Structured Output Record

Template: 1.0.0

Issue: 274

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implement sealed Observatory serving eligibility using terminal #360 authentic transition fixtures.

## Artifacts

- adl-runtime/src/distributed/observatory_serving_eligibility.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_observatory_serving_eligibility.rs

## Execution

- Authentic sealed Acquire/Renew/Transfer/Revoke lifecycle with fail-closed overlap, stale, superseded, revival, replay and substitution behavior
- Named normalized final-state digest over the committed receipt state with recomputation and tamper sensitivity
- Exact three-path implementation against terminal #360 merge base

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "observatory_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove state-machine guard plus normalized final-state digest recomputation and tamper sensitivity; closes R2 digest-convention finding.",
    "outcome": "passed",
    "evidence_ref": "2 passed at source 9a27e5987"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Prove authentic sealed A/R/T/R, restart/replay, overlap, stale/superseded predecessor, revoked revival, nanos expiry, A/B mismatch and redaction through terminal #360 fixtures; closes R2 mapping-boundary finding.",
    "outcome": "passed",
    "evidence_ref": "4 passed at source 9a27e5987"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict feature-bearing Clippy.",
    "outcome": "passed",
    "evidence_ref": "clippy PASS at source 9a27e5987"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/274/validate_scope.py"
    ],
    "purpose": "Prove exact issue scope and one additive mod.rs registration against immutable #360 merge.",
    "outcome": "passed",
    "evidence_ref": "scope PASS base dae957c435b73d87af1f36d4e15fb088f6fd055b"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "dae957c435b73d87af1f36d4e15fb088f6fd055b...HEAD"
    ],
    "purpose": "Reject diff hygiene errors.",
    "outcome": "passed",
    "evidence_ref": "no output at source 9a27e5987"
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
