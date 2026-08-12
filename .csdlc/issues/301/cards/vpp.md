# Validation Planning Prompt

Template: 1.0.0

Issue: 301

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/301/design.md

Diagram: .csdlc/prepared/issues/301/diagram.mmd

## Selected Lanes

[
  {
    "lane": "title-only-github-issue-update",
    "proof_role": "Focused owner regression for title-only body preservation, durable operation provenance, same-key retry, conflicting reuse, concurrent body drift, and body-bearing compatibility.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "strict-clippy-github-owner",
    "proof_role": "Strict warning-free proof for the csdlc-v2 GitHub issue owner crate after the title-only provenance change.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions -- -D warnings`

## Failure Semantics

Fail closed on read, update, preservation, marker, retry, or conflict mismatch without reporting reconciliation.

## Handoff

Retain typed evidence before convergence.
