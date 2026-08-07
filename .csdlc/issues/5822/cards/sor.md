# Structured Output Record

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

WP-05 now retains provenance-bearing advisory estimates and terminal actuals, recomputes calibration from verified sources, measures one genuine terminal cycle-time baseline, and explicitly leaves candidate and reduction fields absent until an equivalent terminal cohort exists.

## Artifacts

- .csdlc/evidence/5822/forecast.json
- .csdlc/evidence/5822/accepted-estimate.json
- .csdlc/evidence/5822/calibration-source-manifest.json
- .csdlc/evidence/5822/calibration-report.json
- .csdlc/evidence/5822/cycle-time-baseline.json
- .csdlc/evidence/5822/terminal-baseline-source-5778.json
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
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5822/csdlc-v2",
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "estimation_contracts"
    ],
    "purpose": "Prove all nine estimation, retained-source, terminal-outcome, calibration, security-negative, and terminal-baseline contracts with zero failures.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/cycle-time-baseline.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Reject whitespace errors at the exact review revision.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/cycle-time-evidence-boundary.json"
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
