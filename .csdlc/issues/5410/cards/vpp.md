# Validation Planning Prompt

Template: 1.0.0

Issue: 5410

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/reviews/v0.91.7/runtime-v3-5410/DESIGN.md

Diagram: docs/reviews/v0.91.7/runtime-v3-5410/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-focused",
    "proof_role": "Prove live assembly, continuity, qualified time, and binary refusal paths",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-strict",
    "proof_role": "Prove formatting, warning-free all-target compilation, and review-ready integration",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 1200,
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
    "parallel_group": "rust",
    "defer_reason": null
  },
  {
    "lane": "inventory-reproducibility",
    "proof_role": "Prove current counts regenerate deterministically and historical labels remain explicit",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on missing bindings, invalid signed continuity, unqualified time, stale generated inventory, budget breach, or review findings.

## Handoff

Retain typed evidence before convergence.
