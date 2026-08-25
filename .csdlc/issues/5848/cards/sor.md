# Structured Output Record

Template: 1.0.0

Issue: 5848

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated WP-27 review findings by accounting for all retained internal and external review rows, preserving #314/#471/#316 routing boundaries, and fixing the production birthday activation cleanup, digest, and filesystem-assumption findings.

## Artifacts

- docs/reviews/v0.92/remediation-5848/disposition-register.json
- docs/reviews/v0.92/remediation-5848/README.md
- .csdlc/evidence/5848/production-birthday-focused-test.json
- .csdlc/prepared/issues/5848/validate-remediation-regressions.rb
- adl-runtime-kernel/src/production_birthday.rs
- adl-runtime-kernel/tests/production_birthday.rs

## Execution

- Added a remediation disposition register and README under docs/reviews/v0.92/remediation-5848.
- Retained all 30 source finding rows while grouping duplicate external birthday-code findings by shared fix.
- Routed internal review rows to the merged #467 corrective hydration authority rather than reopening settled release-credit semantics.
- Changed production birthday post-commit cleanup failures to return CommittedWithCleanupPending with the committed receipt.
- Made receipt input digest construction fallible instead of defaulting to an empty string.
- Documented same-host local-filesystem and advisory-lock assumptions on ProductionBirthdayStore.
- Added focused regression tests for cleanup-pending and partial-cleanup recovery.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5848/validate-remediation-regressions.rb"
    ],
    "purpose": "Validate complete internal/external finding universe, authoritative dispositions, evidence digests, external-review intake regression, and current-head production birthday focused regression.",
    "outcome": "passed",
    "evidence_ref": "local:issue-315-remediation-validator:passed"
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
