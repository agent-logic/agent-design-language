# Validation Planning Prompt

Template: 1.0.0

Issue: 47

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/47/design.md

Diagram: .csdlc/prepared/issues/47/diagram.mmd

## Selected Lanes

[
  {
    "lane": "rust-selector-contract",
    "proof_role": "The issue-owned focused integration target will prove exact, broad, and invalid selector classification plus actionable diagnostics after implementation creates it.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "validation_selectors"
    ],
    "parallel_group": "local",
    "defer_reason": "Deferred until issue #47 implementation creates the declared owned target csdlc-v2/tests/validation_selectors.rs; execution must fail closed if the target is still absent."
  },
  {
    "lane": "exact-schema-selection",
    "proof_role": "The existing target-bounded library command proves a nonzero schema unit-test set and excludes integration targets.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "schema::tests"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "selector-contract-lint",
    "proof_role": "Strict Clippy and active guidance checks prove the typed implementation and examples remain consistent.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 4000,
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test validation_selectors`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --lib schema::tests`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on ambiguous target intent, missing or conflicting target names, zero selected exact-lane tests, unexpected integration fan-out, stale typed state, or misleading active guidance.

## Handoff

Retain typed evidence before convergence.
