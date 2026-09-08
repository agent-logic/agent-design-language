# Validation Planning Prompt

Template: 1.0.0

Issue: 725

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .adl/requests/725-design.md

Diagram: .adl/requests/725-diagram.md

## Selected Lanes

[
  {
    "lane": "csdlc-v3-focused",
    "proof_role": "focused implementation correctness",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "no-v2-fresh-worktree",
    "proof_role": "standalone build denominator",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 2400,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "proof_parity_install_commands"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 21600

Tokens: 100000

## Commands

- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml`
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands`

## Failure Semantics

Fail closed on stale selector receipt exact-head lifecycle digest or uncertain remote reconciliation.

## Handoff

Retain typed evidence before convergence.
