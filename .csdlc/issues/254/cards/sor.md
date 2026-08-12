# Structured Output Record

Template: 1.0.0

Issue: 254

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Eliminated repeated hosted workspace coverage compilation by making the full workspace producer emit the authoritative summary/provenance artifact and converting adl-coverage-hosted into a light verification/merge gate on ubuntu-latest.

## Artifacts

- .github/workflows/ci.yaml
- adl/tools/test_ci_runtime_contracts.sh
- adl/tools/test_ci_path_policy.sh
- adl/tools/validate_ci_workflow_policy.rb
- .csdlc/prepared/issues/254/design.md
- .csdlc/prepared/issues/254/diagram.mmd

## Execution

- .github/workflows/ci.yaml: removed workspace shard matrix/profraw aggregate reporting and aggregate Rust toolchain/cache/install steps
- .github/workflows/ci.yaml: full workspace producer now runs run-and-report once and uploads workspace summary/provenance/log evidence
- .github/workflows/ci.yaml: adl-coverage-hosted now downloads producer summaries, verifies provenance, copies the workspace summary, merges summaries, and runs existing gates on ubuntu-latest
- adl/tools/test_ci_runtime_contracts.sh: updated contract checks to require the light aggregate topology and forbid aggregate Rust coverage reruns
- adl/tools/test_ci_path_policy.sh: updated path-policy workflow assertions for summary artifact topology
- adl/tools/validate_ci_workflow_policy.rb: classified adl_coverage_hosted as a light aggregate and forbids run_authoritative_coverage_lane.sh in that job

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "purpose": "Focused runtime/coverage workflow contract validation",
    "outcome": "passed",
    "evidence_ref": "local stdout: PASS test_verify_coverage_producer_results; PASS test_ci_runtime_contracts; PASS test_run_pr_fast_coverage_lane"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "purpose": "Focused path-policy PR-fast/full-coverage contract validation",
    "outcome": "passed",
    "evidence_ref": "local stdout: PASS: ci_path_policy PR-fast/full-coverage contract"
  },
  {
    "command": [
      "ruby",
      "adl/tools/validate_ci_workflow_policy.rb"
    ],
    "purpose": "Machine-readable workflow policy validation",
    "outcome": "passed",
    "evidence_ref": "local JSON: status pass"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Whitespace/conflict-marker validation",
    "outcome": "passed",
    "evidence_ref": "local stdout empty; exit 0"
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
