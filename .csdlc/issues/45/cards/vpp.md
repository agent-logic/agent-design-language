# Validation Planning Prompt

Template: 1.0.0

Issue: 45

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/45/design.md

Diagram: .csdlc/prepared/issues/45/diagram.mmd

## Selected Lanes

[
  {
    "lane": "doctor-repository-identity",
    "proof_role": "Focused Rust fixtures prove same-repository acceptance, explicit valid split acceptance, and accidental drift rejection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
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
    "lane": "doctor-contract-and-lint",
    "proof_role": "Typed validation, active guidance scans, and strict Clippy prove schema and operator-contract consistency.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on missing or ambiguous repository identity, unexpected effective remote drift, stale typed state, failed three-case proof, or obsolete active guidance.

## Handoff

Retain typed evidence before convergence.
