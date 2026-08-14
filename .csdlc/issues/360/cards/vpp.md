# Validation Planning Prompt

Template: 1.0.0

Issue: 360

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/360/design.md

Diagram: .csdlc/prepared/issues/360/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused",
    "proof_role": "Prove authentic distinct Observatory fixture and mismatch denial.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_authority_projection",
      "--features",
      "internal-test-fixtures",
      "--",
      "--test-threads=1"
    ],
    "parallel_group": "360-serial-01",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "clippy",
    "proof_role": "Reject warnings in exact target.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_observatory_authority_projection",
      "--features",
      "internal-test-fixtures",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "360-serial-02",
    "defer_reason": "Deferred until implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_authority_projection --features internal-test-fixtures -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_authority_projection --features internal-test-fixtures -- -D warnings`

## Failure Semantics

Fail closed on production exposure scope drift mismatch acceptance review finding CI failure or nonterminal ancestry.

## Handoff

Retain typed evidence before convergence.
