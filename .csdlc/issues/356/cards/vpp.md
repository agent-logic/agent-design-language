# Validation Planning Prompt

Template: 1.0.0

Issue: 356

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/356/design.md

Diagram: .csdlc/prepared/issues/356/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-accessors",
    "proof_role": "Prove exact accessors, A/B denial, and redaction in the existing projection target.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
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
    "parallel_group": "356-serial-1",
    "defer_reason": "Deferred until bound implementation exists."
  },
  {
    "lane": "strict-clippy",
    "proof_role": "Reject warnings in the changed runtime and test target.",
    "acceptance_ids": [
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
    "parallel_group": "356-serial-2",
    "defer_reason": "Deferred until implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_authority_projection --features internal-test-fixtures -- --test-threads=1`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test distributed_observatory_authority_projection --features internal-test-fixtures -- -D warnings`

## Failure Semantics

Fail closed on authority exposure, constructor/mutation, mismatch acceptance, redaction leak, scope drift, failed proof/review/CI, or stale terminal ancestry.

## Handoff

Retain typed evidence before convergence.
