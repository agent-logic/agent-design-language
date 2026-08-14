# Validation Planning Prompt

Template: 1.0.0

Issue: 275

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/275/design.md

Diagram: .csdlc/prepared/issues/275/diagram.mmd

## Selected Lanes

[
  {
    "lane": "integrated-unit",
    "proof_role": "Exact private prefix and receipt normalization tamper proof.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--features",
      "internal-test-fixtures",
      "distributed::integrated_serving_authority_snapshot::tests::normalized_receipt_rejects_tamper",
      "--",
      "--exact"
    ],
    "parallel_group": "275-serial-01",
    "defer_reason": null
  },
  {
    "lane": "integrated-focused-eight-case-matrix",
    "proof_role": "Require exactly eight named tests and zero failures: authentic_pair_snapshot_retry_restart_and_redaction (AC-1/2/5), immutable_multi_operation_prefix_and_four_outcomes (AC-2/4/5), capacity_and_invalid_operation_fail_closed (AC-3), checkpoint_cas_failure_preserves_last_commit (AC-3), corrupt_truncated_and_unknown_state_fail_closed (AC-3), terminal_child_combinations_remain_evidence_only (AC-4), authentic_ab_substitution_is_denied_before_commit (AC-1/4), independent_prefix_receipt_and_checkpoint_tamper_is_denied (AC-2/3/5).",
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
    "budget_tokens": 14000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/275/run_exact_focused_matrix.py"
    ],
    "parallel_group": "275-serial-02",
    "defer_reason": null
  },
  {
    "lane": "integrated-api-denial",
    "proof_role": "Require exactly three selected compile-fail examples for pair construction/by-value use, separate children, and raw lineage/eligibility input.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--doc",
      "integrated_serving_authority_snapshot"
    ],
    "parallel_group": "275-serial-03",
    "defer_reason": null
  },
  {
    "lane": "integrated-lib-clippy",
    "proof_role": "Reject warnings and hidden API drift in the feature-bearing library.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 6500,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "275-serial-04",
    "defer_reason": null
  },
  {
    "lane": "integrated-test-clippy",
    "proof_role": "Reject warnings in the exact focused integration target.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 6500,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_integrated_serving_authority",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "275-serial-05",
    "defer_reason": null
  },
  {
    "lane": "exact-scope-diff",
    "proof_role": "Reject whitespace and product drift outside exact three product paths against terminal #367 base.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/275/validate_exact_scope.py"
    ],
    "parallel_group": "275-serial-06",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --lib --features internal-test-fixtures distributed::integrated_serving_authority_snapshot::tests::normalized_receipt_rejects_tamper -- --exact`
- `python3 .csdlc/prepared/issues/275/run_exact_focused_matrix.py`
- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --doc integrated_serving_authority_snapshot`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib --features internal-test-fixtures -- -D warnings`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_integrated_serving_authority --features internal-test-fixtures -- -D warnings`
- `python3 .csdlc/prepared/issues/275/validate_exact_scope.py`

## Failure Semantics

Fail closed on stale authority, noncanonical ancestry, raw authority input, replay conflict, ambiguous digest, uncommitted publication, corruption, capacity overflow, redaction leak, scope drift, review finding, CI failure, or terminal mismatch.

## Handoff

Retain typed evidence before convergence.
