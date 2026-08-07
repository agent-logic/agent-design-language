# Structured Output Record

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Replaced caller-supplied timing adapters with byte-verified retained-source loading; made calibration loader-only and reproducible from digest-linked sources; made finish collect available terminal actuals; made cycle comparison load, hash, and derive cohorts and gates from retained artifacts; removed synthetic calibration and nonterminal candidate evidence; and revised AC-7 truthfully because no equivalent terminal candidate cohort exists.

## Artifacts

- .csdlc/evidence/5822/atomic-readiness-repair.json
- .csdlc/evidence/5822/forecast.json
- .csdlc/evidence/5822/accepted-estimate.json
- .csdlc/evidence/5822/calibration-source-manifest.json
- .csdlc/evidence/5822/calibration-report.json
- .csdlc/evidence/5822/cycle-time-baseline.json
- .csdlc/evidence/5822/cycle-time-evidence-boundary.json
- .csdlc/evidence/5822/validation-summary.json

## Execution

- csdlc-v2/src/estimation.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/estimation_contracts.rs
- .csdlc/prepared/issues/5822/design.md

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
    "purpose": "Prove byte-verified adapters, identity negatives, unique cohorts, calibration loader integrity and fallback, finish actual collection, cycle artifact verification, missing/tampered artifact rejection, traversal rejection, and advisory semantics.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/validation-summary.json"
  },
  {
    "command": [
      "ruby",
      "-rjson",
      "-e",
      "validate retained terminal baseline and pending candidate boundary"
    ],
    "purpose": "Prove a real measured baseline is retained while candidate evidence and reduction claims remain absent.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5822/cycle-time-evidence-boundary.json"
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
