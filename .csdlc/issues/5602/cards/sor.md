# Structured Output Record

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Add --no-report to every partitioned cargo llvm-cov nextest command and retain the explicit combined ADL and Runtime report commands.

## Artifacts

- adl/tools/run_authoritative_coverage_lane.sh
- adl/tools/test_run_authoritative_coverage_lane.sh

## Execution

- Collect workspace coverage profiles without partition-local reports
- Collect companion Runtime profiles without partition-local reports
- Strengthen exact command-shape regression expectations

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_authoritative_coverage_lane.sh"
    ],
    "purpose": "Prove profile-only partition collection, explicit combined reporting, failure propagation, concurrency isolation, and unchanged gates; FastWork plan separately resolved under /Volumes/FastWork/adl-5602-coverage.",
    "outcome": "passed",
    "evidence_ref": "local:5602-authoritative-coverage-contract-pass"
  }
]

## Integration

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
