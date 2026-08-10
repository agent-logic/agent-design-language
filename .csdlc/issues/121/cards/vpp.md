# Validation Planning Prompt

Template: 1.0.0

Issue: 121

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/121/design.md

Diagram: .csdlc/prepared/issues/121/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Prove quorum fence/revoke, next-epoch, restart-floor, mutation denial, delayed activation, and atomic negatives.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_lease",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": null
  },
  {
    "lane": "strict-focused-clippy",
    "proof_role": "Prove warning-free bounded lease source and exact integration test target.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_lease",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "child",
    "defer_reason": null
  },
  {
    "lane": "exact-machine-receipt",
    "proof_role": "Require exact source, nonzero test count, output digests, and machine-derived negative denominator parity.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/121/validate-proof-receipt.rb"
    ],
    "parallel_group": "receipt",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_lease --no-tests=fail`
- `cargo clippy --manifest-path adl-runtime/Cargo.toml --test distributed_lease -- -D warnings`
- `ruby .csdlc/prepared/issues/121/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on base drift, path widening, weakened possession/quorum rules, unresolved recovery floors, self-attested evidence, zero tests, or unresolved review.

## Handoff

Retain typed evidence before convergence.
