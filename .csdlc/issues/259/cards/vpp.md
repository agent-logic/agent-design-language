# Validation Planning Prompt

Template: 1.0.0

Issue: 259

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/259/design.md

Diagram: .csdlc/prepared/issues/259/diagram.mmd

## Selected Lanes

[
  {
    "lane": "governed-transport-authority",
    "proof_role": "Prove positive transport authorization, negative identity/purpose/revocation paths, and the cfg(test) crate-private raw-store constructor contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_transport",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "dependent-runtime-transport-authority",
    "proof_role": "Prove the dependent integration crate cannot use the private raw-store constructor and exercises transport only through a real published #258 adapter handle.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_runtime_transport",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "transport-coupled-discovery",
    "proof_role": "Prove the directly coupled discovery transport harness still compiles and preserves its fail-closed authorization behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_discovery",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-transport-strict-clippy",
    "proof_role": "Reject warnings in the production library and every changed transport-coupled test target without absorbing unrelated repository-wide all-target lint debt.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--features",
      "internal-test-fixtures",
      "--test",
      "distributed_transport",
      "--test",
      "distributed_discovery",
      "--test",
      "distributed_runtime_transport",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "exact-diff-hygiene",
    "proof_role": "Reject whitespace and conflict-marker drift across the exact issue worktree diff.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
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

- `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_transport -- --nocapture --test-threads=1`
- `cargo test --manifest-path adl-runtime/Cargo.toml --features internal-test-fixtures --test distributed_runtime_transport -- --test-threads=1`
- `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_discovery -- --test-threads=1`
- `cargo clippy --manifest-path adl-runtime/Cargo.toml --features internal-test-fixtures --test distributed_transport --test distributed_discovery --test distributed_runtime_transport -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on stale dependency ancestry, scope collision with #260/#203, validation failure, stale review, or typed publication mismatch.

## Handoff

Retain typed evidence before convergence.
