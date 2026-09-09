# Validation Planning Prompt

Template: 1.0.0

Issue: 762

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/762/design.md

Diagram: .csdlc/prepared/issues/762/diagram.mmd

## Selected Lanes

[
  {
    "lane": "proof-worktree",
    "proof_role": "Required deterministic linked-worktree identity and negative confinement proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "proof_worktree_binding"
    ],
    "parallel_group": "csdlc",
    "defer_reason": null
  },
  {
    "lane": "csdlc-owner",
    "proof_role": "Required C-SDLC v3 CLI and lifecycle integration regression coverage for changed public dispatcher.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "parallel_group": "csdlc",
    "defer_reason": null
  },
  {
    "lane": "clippy",
    "proof_role": "Required Rust static validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "csdlc",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_worktree_binding`
- `cargo test --manifest-path csdlc-v3/Cargo.toml`
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on any missing identity, path or mutation-isolation proof; hosted CI remains required for integration.

## Handoff

Retain typed evidence before convergence.
