# Validation Planning Prompt

Template: 1.0.0

Issue: 723

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/milestones/v0.92.1/evidence/csdlc-v3/issue-723/design.md

Diagram: docs/milestones/v0.92.1/evidence/csdlc-v3/issue-723/diagram.mmd

## Selected Lanes

[
  {
    "lane": "proof-suite",
    "proof_role": "Exact proof shadow install behavior",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
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
  },
  {
    "lane": "all-targets",
    "proof_role": "Full v3 regression denominator",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets"
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

- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands`
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`

## Failure Semantics

Fail closed on malformed typed output, authority mismatch, fixture residue, or any red test.

## Handoff

Retain typed evidence before convergence.
