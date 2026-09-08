# Validation Planning Prompt

Template: 1.0.0

Issue: 721

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/milestones/v0.92.1/evidence/csdlc-v3/issue-721/design.md

Diagram: docs/milestones/v0.92.1/evidence/csdlc-v3/issue-721/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-v3-remote",
    "proof_role": "issue-create mutation parity",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--lib",
      "commands::remote::tests"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "focused-v3-terminal",
    "proof_role": "terminal authority reporting",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 90,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "terminal_cleanup_cutover_commands"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "format-clippy",
    "proof_role": "Rust formatting and lint safety",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "docs-and-backlog",
    "proof_role": "operator documentation and adjacent-defect issue recording",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
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

- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::tests`
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`
- `cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on stale authority, stale review SHA, duplicate/uncertain issue-create reconciliation, or failed focused validation.

## Handoff

Retain typed evidence before convergence.
