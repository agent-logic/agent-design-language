# Validation Planning Prompt

Template: 1.0.0

Issue: 358

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/358/design.md

Diagram: .csdlc/prepared/issues/358/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused",
    "proof_role": "Prove action shapes, mutations, A/B denial, time boundaries, restart and redaction.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 16000,
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
    "parallel_group": "358-serial-1",
    "defer_reason": "Deferred until bound implementation."
  },
  {
    "lane": "clippy",
    "proof_role": "Reject warnings.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
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
    "parallel_group": "358-serial-2",
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

Fail closed on ambiguous action/predecessor, lossy time, caller authority, redaction leak, scope drift, proof/review/CI failure.

## Handoff

Retain typed evidence before convergence.
