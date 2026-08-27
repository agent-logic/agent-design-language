# Validation Planning Prompt

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/544/design.md

Diagram: .csdlc/prepared/issues/544/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-lifecycle-primary-checkout-guard",
    "proof_role": "Focused Rust tests prove primary rejection, zero residue, isolated success, fail-closed ambiguous topology, idempotent isolated reconciliation, and unchanged bind policy.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "primary_checkout_bootstrap_guard"
    ],
    "parallel_group": "csdlc-v2-focused",
    "defer_reason": "Deferred only until the bound issue worktree creates csdlc-v2/tests/primary_checkout_bootstrap_guard.rs; fail-closed policy keeps publication blocked until this owned validator exists and passes."
  },
  {
    "lane": "csdlc-v2-lifecycle-unit",
    "proof_role": "Focused lifecycle unit coverage for the touched module.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "lifecycle"
    ],
    "parallel_group": "csdlc-v2-focused",
    "defer_reason": null
  },
  {
    "lane": "primary-checkout-bootstrap-docs-contract",
    "proof_role": "Focused documentation contract confirms primary checkout is inspection-only and bootstrap uses an isolated staging checkout.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "primary_checkout_bootstrap_guard",
      "operator_docs"
    ],
    "parallel_group": "docs-contract",
    "defer_reason": "Deferred only until the bound issue worktree creates csdlc-v2/tests/primary_checkout_bootstrap_guard.rs with the operator documentation contract; fail-closed policy keeps publication blocked until this owned validator exists and passes."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test primary_checkout_bootstrap_guard`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --lib lifecycle`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test primary_checkout_bootstrap_guard operator_docs`

## Failure Semantics

Fail closed on missing or ambiguous Git topology, primary-checkout invocation, initialization residue, bind-policy drift, failed focused validation, stale review truth, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
