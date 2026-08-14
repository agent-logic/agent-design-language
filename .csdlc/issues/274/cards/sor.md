# Structured Output Record

Template: 1.0.0

Issue: 274

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediate PR362 coverage-impact failure with an exact Observatory integration-plus-unit mapping and unchanged 80 percent threshold.

## Artifacts

- adl-runtime/src/distributed/observatory_serving_eligibility.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_observatory_serving_eligibility.rs
- adl/tools/check_coverage_impact.sh
- adl/tools/test_check_coverage_impact.sh
- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh

## Execution

- Map only observatory_serving_eligibility.rs to the exact integration-plus-unit nextest union
- Route only that exact union through internal-test-fixtures and bounded --lib plus integration-test Cargo targets
- Preserve unrelated-unmapped fail-closed behavior and all existing routes
- Refresh immutable scope and diff base to terminal #363 merge

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Exact mapping and unrelated-unmapped fail-closed contract.",
    "outcome": "passed",
    "evidence_ref": "PASS at source 02ed85c37"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "purpose": "Exact union runner target and no-route-drift contract.",
    "outcome": "passed",
    "evidence_ref": "PASS at source 02ed85c37"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_pr_fast_coverage_lane.sh",
      "--filter-expression",
      "binary_id(adl-runtime::distributed_observatory_serving_eligibility) or (binary_id(adl-runtime) and test(/^distributed::observatory_serving_eligibility::tests::/))"
    ],
    "purpose": "Feature-bearing focused union coverage.",
    "outcome": "passed",
    "evidence_ref": "6 tests across 2 binaries; module 350/360 lines 97.22 percent at source 02ed85c37"
  },
  {
    "command": [
      "bash",
      "adl/tools/check_coverage_impact.sh",
      "--changed-files",
      ".csdlc/evidence/274/coverage-impact-changed-files.txt",
      "--summary",
      "adl/target/coverage-impact-summary.json"
    ],
    "purpose": "Unchanged 80 percent module preflight.",
    "outcome": "passed",
    "evidence_ref": "350/360 97.22 percent exceeds unchanged 80 percent"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "observatory_serving_eligibility::tests",
      "--features",
      "internal-test-fixtures"
    ],
    "purpose": "Meaningful state and normalized-digest unit behavior.",
    "outcome": "passed",
    "evidence_ref": "2 passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_serving_eligibility",
      "--features",
      "internal-test-fixtures"
    ],
    "purpose": "Authentic sealed lifecycle behavior.",
    "outcome": "passed",
    "evidence_ref": "4 passed"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict focused Clippy.",
    "outcome": "passed",
    "evidence_ref": "PASS"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/274/validate_scope.py"
    ],
    "purpose": "Exact seven-path and one-line registration scope.",
    "outcome": "passed",
    "evidence_ref": "PASS at source 02ed85c37"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "4db4c2b9a5d622fb7af6ffa1346b4d5406d4a699...HEAD"
    ],
    "purpose": "Immutable #363-base diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "no output at source 02ed85c37"
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
