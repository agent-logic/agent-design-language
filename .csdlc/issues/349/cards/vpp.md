# Validation Planning Prompt

Template: 1.0.0

Issue: 349

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/349/design.md

Diagram: .csdlc/prepared/issues/349/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-349-preparation",
    "proof_role": "Prove exact generation/digest and six-card integrity, authored design bindings, and absence of local #342 paths or status mutations.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/349/validate_preparation.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "gate2-deferred-ready-sequence",
    "proof_role": "Exercise initialized doctor PASS, advertised advance_ready, unchanged ready doctor and bind, bound missing-target failure, and materialized-target PASS.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "initialized_deferred_distributed_targets_bind_only_through_exact_path_harnesses"
    ],
    "parallel_group": "local",
    "defer_reason": "Runs after typed bind and the bounded lifecycle/test correction exists."
  },
  {
    "lane": "gate2-deferred-negative-predicate-matrix",
    "proof_role": "Prove each missing #79 admission predicate fails closed: exact ownership, validator deliverable, bounded non-placeholder deferral, proving route, and failure policy remain mandatory.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "deferred_rust_path_harness_admission_fails_closed_for_each_missing_predicate"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-format-check",
    "proof_role": "Prove the touched C-SDLC v2 Rust source and regression remain canonically formatted.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-strict-clippy",
    "proof_role": "Reject warnings and lifecycle implementation defects in the touched C-SDLC v2 crate.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/349/validate_preparation.rb`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 initialized_deferred_distributed_targets_bind_only_through_exact_path_harnesses`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 deferred_rust_path_harness_admission_fails_closed_for_each_missing_predicate`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on stale topology, scope collision, #342 mutation, widened deferred admission, weakened #79 predicates, post-bind admission, validation failure, or stale review.

## Handoff

Retain typed evidence before convergence.
