# Validation Planning Prompt

Template: 1.0.0

Issue: 5411

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/reviews/v0.91.7/runtime-v3-5411/DESIGN.md

Diagram: docs/reviews/v0.91.7/runtime-v3-5411/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-focused",
    "proof_role": "Prove guardian containment, pressure continuity, and release evidence semantics",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "runtime-v3",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-quality",
    "proof_role": "Prove formatting, warnings, and implementation budget truth",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
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
    "parallel_group": "runtime-v3-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --all-targets`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed: preserve the running process when continuity cannot commit, report the terminal error, and do not promote non-executed evidence.

## Handoff

Retain typed evidence before convergence.
