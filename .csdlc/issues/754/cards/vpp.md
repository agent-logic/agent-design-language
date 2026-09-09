# Validation Planning Prompt

Template: 1.0.0

Issue: 754

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/754/design.md

Diagram: .csdlc/prepared/issues/754/diagram.mmd

## Selected Lanes

[
  {
    "lane": "tests",
    "proof_role": "Required deterministic local standalone tests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1000,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "serial",
    "defer_reason": null
  },
  {
    "lane": "format",
    "proof_role": "Required deterministic local standalone format",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1000,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "parallel_group": "serial",
    "defer_reason": null
  },
  {
    "lane": "clippy",
    "proof_role": "Required deterministic local standalone clippy",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1000,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "serial",
    "defer_reason": null
  },
  {
    "lane": "registry-focused",
    "proof_role": "Required deterministic local standalone tests",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate9"
    ],
    "parallel_group": "serial",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate9`

## Failure Semantics

Fail closed on failed required tests, provenance, or current review.

## Handoff

Retain typed evidence before convergence.
