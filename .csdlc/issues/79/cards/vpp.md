# Validation Planning Prompt

Template: 1.0.0

Issue: 79

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/79/design.md

Diagram: .csdlc/prepared/issues/79/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-gate2-bind-safe-deferred-targets",
    "proof_role": "Focused Gate 2 tests prove child-shaped pre-bind admission and preserve all false-readiness rejection paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-gate2-strict-clippy",
    "proof_role": "Warning-denied Clippy proves the changed readiness logic and focused Gate 2 test target remain clean.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "--test",
      "gate2",
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

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --lib --test gate2 -- -D warnings`

## Failure Semantics

Fail closed on undeclared or unroutable targets, placeholder deferrals, permissive lanes, zero selected tests, missing implemented artifacts, absent proof, or prose-to-path misclassification.

## Handoff

Retain typed evidence before convergence.
