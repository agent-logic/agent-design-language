# Validation Planning Prompt

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/112/design.md

Diagram: .csdlc/prepared/issues/112/diagram.mmd

## Selected Lanes

[
  {
    "lane": "layer8-authority-core-tests",
    "proof_role": "Prove the focused Layer 8 authority integration test target selects and passes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "layer8_authority",
      "--",
      "--nocapture"
    ],
    "parallel_group": "112-core",
    "defer_reason": null
  },
  {
    "lane": "layer8-authority-core-fmt",
    "proof_role": "Prove changed runtime-kernel Rust sources are formatted.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "parallel_group": "112-core",
    "defer_reason": null
  },
  {
    "lane": "layer8-authority-core-clippy",
    "proof_role": "Prove the changed runtime-kernel crate is warning-clean under clippy.",
    "acceptance_ids": [
      "AC-3",
      "AC-6",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "112-core",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test layer8_authority -- --nocapture`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on serial-gate or ownership drift, unauthenticated or stale identity, capability mismatch, recipient substitution or widening, cross-Polis action, replay, revocation, expiry, policy uncertainty, audit discontinuity or write failure, forbidden-field leakage, zero-test selection, preparation-as-product-proof substitution, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
