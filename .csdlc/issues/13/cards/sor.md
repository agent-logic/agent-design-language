# Structured Output Record

Template: 1.0.0

Issue: 13

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added canonical Runtime, fast-workspace, and full-workspace producer selectors; mapped the v0.92 Observatory validator; routed bounded Runtime-owner profiles to Runtime-only coverage; moved producer selection to job-level guards; and extracted a fail-closed hosted/Spot result verifier with a directly executable route matrix.

## Artifacts

- .csdlc/prepared/issues/13/design.md
- .csdlc/prepared/issues/13/diagram.mmd

## Execution

- .github/workflows/ci.yaml
- adl/config/validation_lane_selector.v0.91.6.json
- adl/tools/ci_path_policy.sh
- adl/tools/verify_coverage_producer_results.sh
- adl/tools/test_verify_coverage_producer_results.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/test_ci_runtime_contracts.sh

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Prove Runtime-only, fast-workspace, authoritative-full, no-coverage, validation-manager mapping, and canonical workflow routing contracts.",
    "outcome": "passed",
    "evidence_ref": "Focused path-policy suite passed, including the PR #9 Runtime + Observatory regression fixture with runtime=true, workspace-fast=false, workspace-full=false."
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Prove job-level producer guards, canonical path-policy consumption, all valid hosted/Spot producer combinations, invalid-combination rejection, result mismatches, provenance routing, and stable required aggregation.",
    "outcome": "passed",
    "evidence_ref": "CI runtime contracts and the directly invoked test_verify_coverage_producer_results matrix passed. YAML parsing and git diff hygiene also passed."
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
