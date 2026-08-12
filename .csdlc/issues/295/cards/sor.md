# Structured Output Record

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated stale mechanical-fallout evidence retention so rejected or failed reruns cannot leave a prior receipt or result set available as current proof.

## Artifacts

- adl/tools/check_coverage_impact.sh
- adl/tools/test_mechanical_coverage_fallout.sh
- .csdlc/evidence/295

## Execution

- Namespace governed proof results by the exact mapped source path while preserving unrelated paths' evidence.
- Remove the path's prior receipt and results before each classification attempt and on every classifier or post-execution integrity failure.
- Add a success-then-semantic-failure regression in the same receipt directory proving stale receipt and result artifacts are absent.
- Preserve the 80 percent coverage threshold, non-authoritative PR evidence boundary, and read-only issue 258 fixture contract.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Prove fail-closed stale receipt/results cleanup, parser, mapping, governed execution, negatives, and threshold integration.",
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

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
