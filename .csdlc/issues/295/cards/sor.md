# Structured Output Record

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added fail-closed exact mechanical compile-fallout classification with complete receipts while preserving changed-source coverage authority.

## Artifacts

- adl/tools/mechanical_coverage_fallout.py
- adl/config/mechanical_coverage_fallout.v1.json
- adl/tools/test_mechanical_coverage_fallout.sh
- adl/tools/check_coverage_impact.sh
- adl/config/validation_lane_selector.v0.91.6.json
- docs/tooling/COVERAGE_AUTHORITY_AND_RELEASE_PROOF.md

## Execution

- Parse exact unified diffs and accept only governed import or argument pass-through changes.
- Require passed compile commands for every hunk and mapped behavioral tests for every owner path.
- Emit file, hunk, token, owner, tests, rationale, and non-authority in a machine receipt.
- Reject semantic, predicate, branch, state, error, unmapped, and incomplete-proof cases.
- Integrate below-threshold classification without path allowlists, threshold changes, or nightly exclusions.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Prove unchanged 80 percent gate and no nightly/full exclusion",
    "outcome": "passed",
    "evidence_ref": "coverage-impact-regression.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Prove AC-1 through AC-10 including #258-shaped ownership mapping",
    "outcome": "passed",
    "evidence_ref": "mechanical-compile-fallout-classifier.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "purpose": "Prove lane class, proof role, determinism, resource, and release gate routing",
    "outcome": "passed",
    "evidence_ref": "validation-selector-pvf.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
