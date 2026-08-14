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
    "proof_role": "cfg(test)-only server-hook cleanup-race proof repeated thirty times with bounded scheduler pressure",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "conversation_sessions_tests::authenticated_selected_agent_conversation_uses_canonical_wss_ingress",
      "--",
      "--exact"
    ],
    "parallel_group": "runtime-focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-fast-tests",
    "proof_role": "required Runtime test surface matching CI",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 450,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml"
    ],
    "parallel_group": "runtime-required",
    "defer_reason": null
  },
  {
    "lane": "runtime-v3-fast-clippy",
    "proof_role": "required Runtime strict Clippy surface matching CI",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 250,
    "budget_tokens": 3500,
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

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib conversation_sessions_tests::authenticated_selected_agent_conversation_uses_canonical_wss_ingress -- --exact`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on authentication, capacity, cancellation, deadline, or ingress failure; never emit duplicate terminal results.

## Handoff

Retain typed evidence before convergence.
