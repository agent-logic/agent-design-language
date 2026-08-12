# Structured Output Record

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated fourth-review findings by binding imports and control inputs to exact identities and strictly parsing the bounded Git unified-diff form.

## Artifacts

- adl/config/mechanical_coverage_fallout.v1.json
- adl/tools/mechanical_coverage_fallout.py
- adl/tools/check_coverage_impact.sh
- adl/tools/test_mechanical_coverage_fallout.sh
- docs/tooling/COVERAGE_AUTHORITY_AND_RELEASE_PROOF.md
- .csdlc/evidence/295

## Execution

- Bind the governed token to the configured exact Rust module path and reject stable wrong-prefix imports.
- Load classifier and mapping from the clean exact-revision archive and verify both control digests after execution.
- Validate exact single-file diff headers, optional index header, hunk ranges and line counts, body prefixes, and EOF with no trailing content.
- Add wrong-module, dirty classifier/mapping, malformed header/count/body/trailing-content negatives while retaining the 80 percent and nightly/full gates.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "purpose": "Prove exact import identity, immutable controls, strict diff parsing, negatives, and threshold integration.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/295/mechanical-compile-fallout-classifier.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Prove unchanged 80 percent and authoritative coverage policy.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/295/coverage-impact-regression.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "purpose": "Prove PVF selector and non-authoritative PR routing.",
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
