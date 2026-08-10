# Validation Planning Prompt

Template: 1.0.0

Issue: 5909

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5869/design.md

Diagram: .csdlc/prepared/issues/5869/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Prove all four corrective behaviors with nonzero issue-owned Rust tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
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
    "proof_role": "Prove warning-free focused Rust implementation and tests.",
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
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Validate source, test count, machine negative cases, and artifact digests.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5909/validate-proof-receipt.rb"
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
- `ruby .csdlc/prepared/issues/5909/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on scope drift, non-atomic capacity handling, self-attested evidence, stale source binding, zero tests, or unresolved review.

## Handoff

Retain typed evidence before convergence.
