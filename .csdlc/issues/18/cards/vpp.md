# Validation Planning Prompt

Template: 1.0.0

Issue: 18

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/18/design.md

Diagram: .csdlc/prepared/issues/18/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-broken-pipe-focused",
    "proof_role": "Focused library and split-binary tests prove EPIPE normalization, JSON preservation, and clean stderr.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
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
      "--lib",
      "--test",
      "gate_github_actions"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-broken-pipe-clippy",
    "proof_role": "Warning-denied Clippy proves the shared writer and split binary integration remain clean.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "--bin",
      "csdlc-github-issue",
      "--bin",
      "csdlc-github-pr",
      "--test",
      "gate_github_actions",
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --lib --test gate_github_actions`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --lib --bin csdlc-github-issue --bin csdlc-github-pr --test gate_github_actions -- -D warnings`

## Failure Semantics

Fail closed on panic output, nonzero schema exit, non-JSON stdout, swallowed non-EPIPE failures, or split-binary drift.

## Handoff

Retain typed evidence before convergence.
