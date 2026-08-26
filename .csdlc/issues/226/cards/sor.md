# Structured Output Record

Template: 1.0.0

Issue: 226

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Map the issue 111 and 113 Observatory and diagram paths to existing focused lanes while preserving unknown-path fail-closed routing.

## Artifacts

- .csdlc/evidence/226/selector-contract.log
- .csdlc/evidence/226/ci-path-policy-contract.log

## Execution

- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/test_select_validation_lanes.sh
- adl/tools/test_ci_path_policy.sh

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove no slow proof or authoritative full coverage for the issue 111 and 113 path set.",
    "outcome": "passed",
    "evidence_ref": "ci-path-policy-contract.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "purpose": "Prove exact issue 111 and 113 path routing and unknown-path fail-closed behavior.",
    "outcome": "passed",
    "evidence_ref": "selector-contract.log"
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
