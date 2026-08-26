# Structured Output Record

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated all post-allocation mechanical-fallout failure paths so stale or invalid revision resolution cannot retain temporary proof artifacts or prior receipt/results evidence.

## Artifacts

- adl/tools/check_coverage_impact.sh
- adl/tools/test_mechanical_coverage_fallout.sh
- .csdlc/evidence/295

## Execution

- Centralize removal of the proof archive root, per-path result directory, temporary diff files, and receipt in one cleanup helper.
- Route initial base rev-parse, merge-base, head rev-parse, archive, patch, control discovery, integrity, post-revision, post-diff, and classifier rejection failures through the same cleanup helper.
- Add a stale-base regression with an isolated temporary directory proving no archive root, diff file, stale receipt, or path result survives revision-resolution failure.
- Preserve the prior success-then-semantic-failure regression, 80 percent threshold, PR-fast non-authority, and read-only issue 258 boundary.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Prove centralized cleanup for revision-resolution and classifier failure paths plus prior parser, mapping, governed execution, and threshold behavior.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/295/mechanical-compile-fallout-classifier.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Prove the existing 80 percent gate and authoritative coverage routing remain intact.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/295/coverage-impact-regression.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "purpose": "Prove PVF selector inventory and PR-fast non-authoritative routing.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/295/validation-selector-pvf.log"
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
