# Structured Output Record

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented concrete lifecycle, GitHub, validation, session, and operator observation adapters; unique-issue cohort consolidation; calibration-gated fallback; digest-verified terminal finish reconciliation; bounded artifact references; reproducible calibration inputs; and measured operator evidence that refuses an unsupported cycle-time claim.

## Artifacts

- .csdlc/evidence/5822/atomic-readiness-repair.json
- .csdlc/evidence/5822/forecast-inputs.json
- .csdlc/evidence/5822/forecast.json
- .csdlc/evidence/5822/accepted-estimate.json
- .csdlc/evidence/5822/calibration-backtest-inputs.json
- .csdlc/evidence/5822/calibration-backtest.json
- .csdlc/evidence/5822/estimation-outcome.sample.json
- .csdlc/evidence/5822/operator-cycle-inputs.json
- .csdlc/evidence/5822/cycle-time-comparison.json
- .csdlc/evidence/5822/cycle-time-evidence-boundary.json
- .csdlc/evidence/5822/validation-summary.json

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
    "purpose": "Prove concrete adapters, unique cohorts, calibration fallback, digest tampering, terminal finish reconciliation, traversal rejection, and advisory-only behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/validation-summary.json"
  },
  {
    "command": [
      "ruby",
      "-rjson",
      "-e",
      "validate measured provenance and non-comparable zero-reduction boundary"
    ],
    "purpose": "Prove measured provenance is retained and unsupported cycle-time reduction is refused.",
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
