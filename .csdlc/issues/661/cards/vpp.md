# Validation Planning Prompt

Template: 1.0.0

Issue: 661

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/661/design.md

Diagram: .csdlc/prepared/issues/661/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-shepherd-provider-reply",
    "proof_role": "Prove fresh Shepherd turns invoke the configured provider, return generated content with correct correlation, and expose provider failure without fallback.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "conversation"
    ],
    "parallel_group": "runtime-shepherd",
    "defer_reason": "Focused test names are finalized after tracing the existing execution surface."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib conversation`

## Failure Semantics

Fail explicitly when provider resolution or execution fails; never substitute a hardcoded acknowledgement.

## Handoff

Retain typed evidence before convergence.
