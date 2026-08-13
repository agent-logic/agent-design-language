# Validation Planning Prompt

Template: 1.0.0

Issue: 273

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/273/design.md

Diagram: .csdlc/prepared/issues/273/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Prove typed identity, exact predecessor terminal ancestry, disjoint ownership, serial registration order, and declared post-bind targets.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/273/validate_preparation_bundle.py"
    ],
    "parallel_group": "273-serial-01",
    "defer_reason": "Preparation-only lane; initialized phase required and no post-bind claim."
  },
  {
    "lane": "shepherd-focused",
    "proof_role": "Prove acquire, replace, revoke, expiry, exact historical retry, restart, full binding rejection, capacity, receipt, and redaction under the explicit test-fixture feature.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
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
    "parallel_group": "273-serial-02",
    "defer_reason": null
  },
  {
    "lane": "shepherd-clippy",
    "proof_role": "Reject warnings and API misuse in the exact feature-gated Shepherd integration target.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 6000,
    "argv": [
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
    "parallel_group": "273-serial-03",
    "defer_reason": null
  },
  {
    "lane": "ordinary-build",
    "proof_role": "Prove ordinary builds do not expose the internal fixture constructor.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "check",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "parallel_group": "273-serial-04",
    "defer_reason": null
  },
  {
    "lane": "coverage-map-contract",
    "proof_role": "Prove the new Shepherd module maps exactly once to its dedicated coverage token and expression.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "parallel_group": "273-serial-05",
    "defer_reason": null
  },
  {
    "lane": "coverage-runner-contract",
    "proof_role": "Prove the dedicated filter routes only to the runtime companion with internal-test-fixtures enabled.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "parallel_group": "273-serial-06",
    "defer_reason": null
  },
  {
    "lane": "focused-module-coverage",
    "proof_role": "Execute the exact mapped Shepherd integration target and emit the production module coverage summary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "bash",
      "adl/tools/run_pr_fast_coverage_lane.sh",
      "--filter-expression",
      "binary_id(adl-runtime::distributed_shepherd_serving_eligibility)"
    ],
    "parallel_group": "273-serial-07",
    "defer_reason": null
  },
  {
    "lane": "coverage-impact-preflight",
    "proof_role": "Require the focused summary to satisfy the changed production module impact denominator before publication.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/check_coverage_impact.sh",
      "--changed-files",
      ".csdlc/evidence/273/coverage-impact-changed-files.txt",
      "--summary",
      "adl/target/coverage-impact-summary.json"
    ],
    "parallel_group": "273-serial-08",
    "defer_reason": null
  },
  {
    "lane": "shepherd-scope",
    "proof_role": "Require the exact four product paths, four coverage-policy paths, and #273-local records while rejecting unrelated, parent, or #274 paths.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/273/validate_scope.py"
    ],
    "parallel_group": "273-serial-09",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace, conflict marker, and patch hygiene defects.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "273-serial-10",
    "defer_reason": null
  },
  {
    "lane": "terminal-authority",
    "proof_role": "Require canonical merged #273 terminal cache and ancestry before #274 shared registration.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/273/validate_terminal.py"
    ],
    "parallel_group": "273-serial-11",
    "defer_reason": "Deferred until required CI is green and typed finish creates terminal authority; no optional or paid runner authorized."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/273/validate_preparation_bundle.py`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --features internal-test-fixtures --test distributed_shepherd_serving_eligibility -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --features internal-test-fixtures --test distributed_shepherd_serving_eligibility -- -D warnings`
- `cargo check --locked --manifest-path adl-runtime/Cargo.toml`
- `bash adl/tools/test_check_coverage_impact.sh`
- `bash adl/tools/test_run_pr_fast_coverage_lane.sh`
- `bash adl/tools/run_pr_fast_coverage_lane.sh --filter-expression binary_id(adl-runtime::distributed_shepherd_serving_eligibility)`
- `bash adl/tools/check_coverage_impact.sh --changed-files .csdlc/evidence/273/coverage-impact-changed-files.txt --summary adl/target/coverage-impact-summary.json`
- `python3 .csdlc/prepared/issues/273/validate_scope.py`
- `git diff --check origin/main...HEAD`
- `python3 .csdlc/prepared/issues/273/validate_terminal.py`

## Failure Semantics

Fail closed on stale authority, wrong foundation binding, dual eligibility, replay, partial mutation, redaction leak, scope collision, lifecycle drift, review finding, CI failure, or nonancestral terminal state.

## Handoff

Retain typed evidence before convergence.
