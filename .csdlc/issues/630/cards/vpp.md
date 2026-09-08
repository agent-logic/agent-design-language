# Validation Planning Prompt

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/630/design.md

Diagram: .csdlc/prepared/issues/630/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v3-terminal-clean-cutover-tests",
    "proof_role": "Run focused Rust tests for v3 finish, clean, and cutover positive and denial behavior, including caller-forged finish denial, sealed typed terminal closeout, part_of terminal denial, Git-registration-derived cleanup states, and cutover rollback refusal.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2400,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "terminal_cleanup_cutover_commands"
    ],
    "parallel_group": "rust-focused",
    "defer_reason": null
  },
  {
    "lane": "v3-full-regression",
    "proof_role": "Run the complete v3 test suite after route implementation to prove the command manifest, foundation, local, remote, terminal, and transaction suites remain coherent.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "parallel_group": "rust-full",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml`

## Failure Semantics

Fail closed on caller-forged terminal authority, cleanup without Git registration proof, collapsed cleanup outcomes, cutover authority before #505, v2 source changes, stale/empty evidence, or missing real-issue canary coverage.

## Handoff

Retain typed evidence before convergence.
