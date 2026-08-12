# Structured Output Record

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated all three exact-review P1 findings with strict import-addition grammar, verified result/evidence artifact bindings, and replay-resistant exact revision/diff/mapping/result/hunk receipts.

## Artifacts

- adl/tools/mechanical_coverage_fallout.py
- adl/tools/check_coverage_impact.sh
- adl/tools/test_mechanical_coverage_fallout.sh
- docs/tooling/COVERAGE_AUTHORITY_AND_RELEASE_PROOF.md
- .csdlc/evidence/295

## Execution

- Reject semantic import removal, reorder, alias, path, and multiplicity changes.
- Require digest-verified compile and behavioral result artifacts plus their evidence logs instead of caller-declared passed strings.
- Bind base, head, diff, mapping, proof manifest, result, evidence, and hunk content digests in each receipt.
- Preserve the 80 percent threshold, nightly/full policy, and read-only #258 fixture boundary.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Prove exact grammar, artifact integrity, replay resistance, required negatives, and below-threshold integration.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/295/mechanical-compile-fallout-classifier.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Prove the existing 80 percent and authoritative coverage routing contract remains intact.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/295/coverage-impact-regression.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "purpose": "Prove PVF selector inventory and non-authoritative PR evidence routing.",
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
