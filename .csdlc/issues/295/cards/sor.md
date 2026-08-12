# Structured Output Record

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated the second exact-review findings by executing governed proof commands inside the classifier and structurally binding the added token argument to the mapped callee invocation.

## Artifacts

- adl/config/mechanical_coverage_fallout.v1.json
- adl/tools/mechanical_coverage_fallout.py
- adl/tools/check_coverage_impact.sh
- adl/tools/test_mechanical_coverage_fallout.sh
- docs/tooling/COVERAGE_AUTHORITY_AND_RELEASE_PROOF.md
- .csdlc/evidence/295

## Execution

- Execute tracked compile and per-owner behavioral commands directly without accepting caller-authored pass results.
- Record runner producer, argv, exit code, result digest, evidence digest, base, head, diff, mapping, and changed hunk identities.
- Require the sole added token argument to immediately follow the mapped governed callee invocation; reject unrelated calls even within the same hunk.
- Preserve the 80 percent whole-file gate, nightly/full authority policy, and read-only issue 258 fixture boundary.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Prove trusted command execution, mapped-callee binding, exact grammar, negative cases, and below-threshold integration.",
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
