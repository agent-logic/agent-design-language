# Structured Output Record

Template: 1.0.0

Issue: 5896

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and applied the pre-topology bound-record migration.

## Artifacts

- .csdlc/evidence/5896/apply-report.json
- .csdlc/evidence/5896/doctor-results.json
- .csdlc/evidence/5844/validate-article-series.rb

## Execution

- Added evidence-bound atomic migration for 58 pre-topology records
- Corrected bind ordering and topology collision checks
- Made WP-24 validation planning executable and execution-ready

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--test",
      "topology_migration"
    ],
    "purpose": "Focused atomicity, evidence, topology, and idempotence regressions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5896/apply-report.json"
  },
  {
    "command": [
      "cargo",
      "check",
      "--all-targets"
    ],
    "purpose": "Compile all v2 targets after migration API and bind changes.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2/tests/topology_migration.rs"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5896/validate-migration.rb"
    ],
    "purpose": "Validate the 58-record cohort, idempotence, corruption denominator, and WP-24 readiness.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5896/doctor-results.json"
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
