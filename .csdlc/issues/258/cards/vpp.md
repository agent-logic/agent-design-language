# Validation Planning Prompt

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/258/design.md

Diagram: .csdlc/prepared/issues/258/diagram.mmd

## Selected Lanes

[
  {
    "lane": "authority-store-boundary",
    "proof_role": "Focused authority boundary proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_identity_lease_authority",
      "--",
      "--nocapture",
      "--test-threads=1"
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

- `cargo test --manifest-path adl-runtime/Cargo.toml --test distributed_identity_lease_authority -- --nocapture --test-threads=1`

## Failure Semantics

Fail closed on stale review, validation failure, or typed publication mismatch.

## Handoff

Retain typed evidence before convergence.
