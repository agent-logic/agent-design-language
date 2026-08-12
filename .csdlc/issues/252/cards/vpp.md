# Validation Planning Prompt

Template: 1.0.0

Issue: 252

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/252/design.md

Diagram: .csdlc/prepared/issues/252/diagram.mmd

## Selected Lanes

[
  {
    "lane": "guardian-parity-focused",
    "proof_role": "Prove the exact parity_b_live_kernel target containing both observed Guardian regressions and genuine SpawnFailed coverage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "parity_b_live_kernel"
    ],
    "parallel_group": "runtime-local",
    "defer_reason": null
  },
  {
    "lane": "runtime-required-local",
    "proof_role": "Run the same complete Runtime-kernel Cargo test boundary used by the hosted Runtime v3 focused-tests step.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "runtime-local",
    "defer_reason": null
  },
  {
    "lane": "runtime-strict-clippy",
    "proof_role": "Prove warning-free Runtime kernel production and test targets before exact-head review.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test parity_b_live_kernel`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml`
- `cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on unresolved executable identity, genuine spawn failure, flaky repeated proof, required-lane failure, Clippy warning, review finding, or scope expansion.

## Handoff

Retain typed evidence before convergence.
