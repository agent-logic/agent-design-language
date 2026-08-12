# Validation Planning Prompt

Template: 1.0.0

Issue: 248

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/248/design.md

Diagram: .csdlc/prepared/issues/248/diagram.mmd

## Selected Lanes

[
  {
    "lane": "process-backend-precedence-repeat",
    "proof_role": "Repeated deterministic precedence and cleanup proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "parity",
      "process_backend_timeout_and_oversized_file_leave_no_artifacts",
      "--",
      "--exact"
    ],
    "parallel_group": "runtime-focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-fast-required",
    "proof_role": "Required Runtime focused test surface matching CI.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "runtime-required",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test parity process_backend_timeout_and_oversized_file_leave_no_artifacts -- --exact`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`

## Failure Semantics

Fail closed, terminate the owned process tree, remove output artifacts, and return exactly one deterministic terminal classification.

## Handoff

Retain typed evidence before convergence.
