# Structured Output Record

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented provenance-bearing advisory estimation, deterministic cohort selection and fallback, typed SPP disposition, terminal comparison validation, calibration/backtesting, and gate-preserving cycle-time comparison.

## Artifacts

- .csdlc/evidence/5822/atomic-readiness-repair.json
- .csdlc/evidence/5822/forecast.json
- .csdlc/evidence/5822/accepted-estimate.json
- .csdlc/evidence/5822/estimation-outcome.sample.json
- .csdlc/evidence/5822/calibration-backtest.json
- .csdlc/evidence/5822/cycle-time-comparison.json
- .csdlc/evidence/5822/cycle-time-evidence-boundary.json

## Execution

- csdlc-v2/src/estimation.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/estimation_contracts.rs

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--no-tests=fail",
      "--test",
      "estimation_contracts"
    ],
    "purpose": "Prove schema, privacy, leakage rejection, deterministic cohorts, fallback, advisory-only editing, terminal comparison, calibration, and backtesting.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/validation-summary.json"
  },
  {
    "command": [
      "ruby",
      "-rjson",
      "-e",
      "validate cycle-time-comparison.json equivalent cohorts and preserved gates"
    ],
    "purpose": "Prove controlled component accounting and reject incomparable or gate-weakened cycle-time claims.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/cycle-time-comparison.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace errors and preserve reviewable exact-revision output.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/validation-summary.json"
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
