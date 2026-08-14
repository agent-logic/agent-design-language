# Validation Planning Prompt

Template: 1.0.0

Issue: 274

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/274/design.md

Diagram: .csdlc/prepared/issues/274/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Pre-bind packet identity only; not rerun post-bind.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/274/validate_preparation_bundle.py"
    ],
    "parallel_group": "274-01",
    "defer_reason": "Pre-bind-only evidence."
  },
  {
    "lane": "observatory-transition-unit",
    "proof_role": "Prove state guard and normalized final-state digest recomputation/tamper denial.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "observatory_serving_eligibility::tests",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "274-02a",
    "defer_reason": null
  },
  {
    "lane": "observatory-focused",
    "proof_role": "Prove authentic sealed lifecycle and meaningful uncovered branches through terminal #360 fixtures.",
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
    "budget_tokens": 16000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_serving_eligibility",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "274-02",
    "defer_reason": null
  },
  {
    "lane": "observatory-clippy",
    "proof_role": "Reject warnings in exact feature-bearing target.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
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
    "parallel_group": "274-03",
    "defer_reason": null
  },
  {
    "lane": "coverage-map-contract",
    "proof_role": "Prove the exact integration-plus-unit union mapping and unrelated-unmapped fail-closed behavior.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 540,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "parallel_group": "274-04",
    "defer_reason": null
  },
  {
    "lane": "coverage-runner-contract",
    "proof_role": "Prove the exact union activates bounded --lib plus integration targets with internal-test-fixtures and no other route drift.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_run_pr_fast_coverage_lane.sh"
    ],
    "parallel_group": "274-05",
    "defer_reason": null
  },
  {
    "lane": "focused-module-coverage",
    "proof_role": "Execute the exact mapped Observatory integration-plus-unit union and emit a nonzero focused summary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "bash",
      "adl/tools/run_pr_fast_coverage_lane.sh",
      "--filter-expression",
      "binary_id(adl-runtime::distributed_observatory_serving_eligibility) or (binary_id(adl-runtime) and test(/^distributed::observatory_serving_eligibility::tests::/))"
    ],
    "parallel_group": "274-06",
    "defer_reason": null
  },
  {
    "lane": "coverage-impact-preflight",
    "proof_role": "Require focused summary to satisfy unchanged 80 percent module denominator.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/check_coverage_impact.sh",
      "--changed-files",
      ".csdlc/evidence/274/coverage-impact-changed-files.txt",
      "--summary",
      "adl/target/coverage-impact-summary.json"
    ],
    "parallel_group": "274-07",
    "defer_reason": null
  },
  {
    "lane": "exact-scope",
    "proof_role": "Prove exact product paths, one mod.rs line, and only four coverage tooling paths.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/274/validate_scope.py"
    ],
    "parallel_group": "274-08",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors against immutable #363 merge base.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check",
      "4db4c2b9a5d622fb7af6ffa1346b4d5406d4a699...HEAD"
    ],
    "parallel_group": "274-09",
    "defer_reason": null
  },
  {
    "lane": "terminal-ancestry",
    "proof_role": "After finish prove canonical terminal cache and ancestry.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/274/validate_terminal.py"
    ],
    "parallel_group": "274-10",
    "defer_reason": "Deferred until typed finish."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/274/validate_preparation_bundle.py`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib observatory_serving_eligibility::tests --features internal-test-fixtures -- --test-threads=1`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_serving_eligibility --features internal-test-fixtures -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_serving_eligibility --features internal-test-fixtures -- -D warnings`
- `bash adl/tools/test_check_coverage_impact.sh`
- `bash adl/tools/test_run_pr_fast_coverage_lane.sh`
- `bash adl/tools/run_pr_fast_coverage_lane.sh --filter-expression binary_id(adl-runtime::distributed_observatory_serving_eligibility) or (binary_id(adl-runtime) and test(/^distributed::observatory_serving_eligibility::tests::/))`
- `bash adl/tools/check_coverage_impact.sh --changed-files .csdlc/evidence/274/coverage-impact-changed-files.txt --summary adl/target/coverage-impact-summary.json`
- `python3 .csdlc/prepared/issues/274/validate_scope.py`
- `git diff --check 4db4c2b9a5d622fb7af6ffa1346b4d5406d4a699...HEAD`
- `python3 .csdlc/prepared/issues/274/validate_terminal.py`

## Failure Semantics

Fail closed on stale or nonancestral authority, wrong quorum/lease/OwnerCommit/fence/generation/receipt, overlapping transfer, replay conflict, restart ambiguity, revoked or expired revival, redaction leak, scope drift, premature bind, review finding, CI failure, or noncanonical terminal ancestry.

## Handoff

Retain typed evidence before convergence.
