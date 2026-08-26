# Validation Planning Prompt

Template: 1.0.0

Issue: 323

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/323/design.md

Diagram: .csdlc/prepared/issues/323/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-issue-migration-tests",
    "proof_role": "Focused Rust regression coverage for bound issue identity migration and finish identity invariants",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "topology_migration"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-clippy",
    "proof_role": "Strict relevant Rust lint for C-SDLC v2 owner changes",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test topology_migration`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml -- -D warnings`

## Failure Semantics

Fail closed if the operation cannot prove source identity, target issue provenance, clean topology, nonterminal phase, and safe publication/review handling.

## Handoff

Retain typed evidence before convergence.
