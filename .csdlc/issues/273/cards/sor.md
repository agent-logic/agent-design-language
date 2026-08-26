# Structured Output Record

Template: 1.0.0

Issue: 273

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded Shepherd serving-eligibility authority over a non-forgeable verified #272 foundation cut and added its exact required coverage-impact mapping after CI exposed an unmapped denominator.

## Artifacts

- adl-runtime/src/distributed/serving_authority.rs
- adl-runtime/src/distributed/shepherd_serving_eligibility.rs
- adl-runtime/src/distributed/mod.rs
- adl-runtime/tests/distributed_shepherd_serving_eligibility.rs
- adl/tools/check_coverage_impact.sh
- adl/tools/run_pr_fast_coverage_lane.sh
- adl/tools/test_check_coverage_impact.sh
- adl/tools/test_run_pr_fast_coverage_lane.sh
- .csdlc/evidence/273

## Execution

- Added a non-policy opaque VerifiedServingAuthorityCut returned only after existing sealed receipt/binding verification and durable Published commit.
- Added the Shepherd-only checkpointed eligibility store with fenced acquire, replace, revoke, expiry, retry, restart, and capacity behavior.
- Registered only the Shepherd module and added six focused deterministic tests; #274 and #205 remain untouched.
- Mapped the new production module to its exact runtime-only focused coverage target and added classifier and runner contract regressions.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_shepherd_serving_eligibility",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy for the exact feature-enabled focused target.",
    "outcome": "passed",
    "evidence_ref": "issue273-clippy.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Run exact candidate diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "issue273-diff.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_shepherd_serving_eligibility",
      "--",
      "--test-threads=1"
    ],
    "purpose": "Run the exact feature-enabled six-test integration target.",
    "outcome": "passed",
    "evidence_ref": "issue273-focused.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/273/validate_scope.py"
    ],
    "purpose": "Run fail-closed issue scope proof.",
    "outcome": "passed",
    "evidence_ref": "issue273-scope.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "purpose": "Prove exact production-path classifier and filter-expression mapping.",
    "outcome": "passed",
    "evidence_ref": "coverage-map-contract.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "purpose": "Prove runtime-only routing and internal-test-fixtures activation for the dedicated filter.",
    "outcome": "passed",
    "evidence_ref": "coverage-runner-contract.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_pr_fast_coverage_lane.sh",
      "--filter-expression",
      "binary_id(adl-runtime::distributed_shepherd_serving_eligibility)"
    ],
    "purpose": "Execute the mapped six-test target and prove the new production module denominator.",
    "outcome": "passed",
    "evidence_ref": "focused-module-coverage.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/check_coverage_impact.sh",
      "--changed-files",
      ".csdlc/evidence/273/coverage-impact-changed-files.txt",
      "--summary",
      "adl/target/coverage-impact-summary.json"
    ],
    "purpose": "Prove the changed production module satisfies its required coverage-impact denominator.",
    "outcome": "passed",
    "evidence_ref": "coverage-impact-preflight.log"
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
