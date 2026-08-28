# Validation Planning Prompt

Template: 1.0.0

Issue: 563

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/563/design.md

Diagram: .csdlc/prepared/issues/563/diagram.mmd

## Selected Lanes

[
  {
    "lane": "primary-checkout-owner-integration",
    "proof_role": "Prove current, stale, partial, primary, linked, isolated, and preserved-residue owner behavior",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "primary_checkout_bootstrap_guard"
    ],
    "parallel_group": "owner",
    "defer_reason": null
  },
  {
    "lane": "install-resolve-contract",
    "proof_role": "Prove complete atomic generation and install/resolve receipt behavior",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "operator"
    ],
    "parallel_group": "owner",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Verify exact patch and whitespace hygiene",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-control",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test primary_checkout_bootstrap_guard`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --lib operator`
- `git diff --check`

## Failure Semantics

Fail closed on missing provenance, incomplete owner denominator, any mutation before verification, target checkout drift, mixed generation publication, unsafe residue handling, failing focused proof, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
