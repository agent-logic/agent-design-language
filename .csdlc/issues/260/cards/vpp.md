# Validation Planning Prompt

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/260/design.md

Diagram: .csdlc/prepared/issues/260/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-distributed-authority-adapter-callers-260",
    "proof_role": "Prove governed production caller boundaries and cfg(test)-only raw seams.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 360,
    "budget_tokens": 1800,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_authority_adapter_callers_260",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-migration",
    "proof_role": "Prove deterministic governed migration transitions and retry behavior.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_migration",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-recovery",
    "proof_role": "Prove deterministic governed recovery transitions and fail-closed retry behavior.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_recovery",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-lib-strict-clippy",
    "proof_role": "Reject production warning and lint regressions.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
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

- `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_authority_adapter_callers_260 -- --test-threads=1`
- `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_migration -- --test-threads=1`
- `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_recovery -- --test-threads=1`
- `cargo clippy --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings`

## Failure Semantics

Fail closed on #259 nonterminal gate, scope overlap with #258/#259/#203, stale main, validation failure, stale review, or typed publication mismatch.

## Handoff

Retain typed evidence before convergence.
