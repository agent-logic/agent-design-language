# Validation Planning Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/244/design.md

Diagram: .csdlc/prepared/issues/244/diagram.mmd

## Selected Lanes

[
  {
    "lane": "conversation-cleanup-race-focused",
    "proof_role": "focused cleanup-race integration proof, repeated by the session before finalize",
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
      "conversation_sessions"
    ],
    "parallel_group": "runtime-focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-fast-required",
    "proof_role": "required Runtime focused test surface matching CI",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
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

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_sessions`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`

## Failure Semantics

Fail closed on authentication, capacity, cancellation, deadline, or ingress failure; never emit duplicate terminal results.

## Handoff

Retain typed evidence before convergence.
