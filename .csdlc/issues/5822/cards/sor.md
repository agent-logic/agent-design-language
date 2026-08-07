# Structured Output Record

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Deferred finish now retains available terminal observations; observation identity and validation ownership derive from canonical digest-verified issue records; operator wait and reconnect measurements are retained; calibration outcomes are recomputed from recursively digest-verified actual observation sources; and cycle identity, timing, and required gates derive from verified source bytes. AC-7 remains unsatisfied because no real equivalent terminal baseline and candidate cohort exists.

## Artifacts

- .csdlc/evidence/5822/atomic-readiness-repair.json
- .csdlc/evidence/5822/forecast.json
- .csdlc/evidence/5822/accepted-estimate.json
- .csdlc/evidence/5822/calibration-source-manifest.json
- .csdlc/evidence/5822/calibration-report.json
- .csdlc/evidence/5822/cycle-time-evidence-boundary.json
- .csdlc/evidence/5822/validation-summary.json

## Execution

- csdlc-v2/src/estimation.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/estimation_contracts.rs

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "estimation_contracts",
      "--",
      "--nocapture"
    ],
    "purpose": "Prove the focused estimation and finish contracts, including forged identity, favorable outcome, digest, traversal, deferred finish, operator timing, and gate-derived cycle negatives.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/validation-summary.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "estimation_contracts",
      "cycle_comparison_derives_basis_gates_and_totals_from_verified_artifacts"
    ],
    "purpose": "Require a real equivalent terminal baseline and candidate cohort under identical source-derived gates for AC-7.",
    "outcome": "blocked",
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
