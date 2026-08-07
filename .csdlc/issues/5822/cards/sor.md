# Structured Output Record

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

WP-05 retains provenance-bearing advisory estimates and terminal actuals, measures one genuine terminal cycle-time baseline without claiming a candidate reduction, validates qualified cross-repository closing linkage against verified publication authority, and emits explicit schema drift for unsupported session evidence.

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
    "purpose": "Prove all eleven estimation, retained-source, publication-authority, schema-drift, terminal-outcome, calibration, security-negative, and terminal-baseline contracts with zero failures.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/validation-summary.json"
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
    "purpose": "Reject Rust formatting drift in the remediated adapter and focused tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/validation-summary.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace errors before the exact review revision is committed.",
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
