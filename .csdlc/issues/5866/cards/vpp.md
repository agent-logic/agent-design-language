# Validation Planning Prompt

Template: 1.0.0

Issue: 5866

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5866/design.md

Diagram: .csdlc/prepared/issues/5866/diagram.mmd

## Selected Lanes

[
  {
    "lane": "exact-child-tests",
    "proof_role": "Exact nonzero target proves candidate and seed identity plus transport-certificate generation binding across await, canonical bounded Prost messages, durable replay rejection across restart, cross-seed request replay denial, bounded live-window capacity, expiry recovery, and real Quinn/rustls positive behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_discovery",
      "--no-tests=fail"
    ],
    "parallel_group": "child",
    "defer_reason": null
  },
  {
    "lane": "exact-revision-proof-receipt",
    "proof_role": "Validate the final two-revision generation-bound protobuf and durable-replay source, command, nonzero test, negative-case, runner, and artifact bindings.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5866/validate-proof-receipt.rb",
      ".csdlc/evidence/5866/generation-protobuf-durable/execution-proof.json"
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

- `cargo nextest run --manifest-path adl-runtime/Cargo.toml --test distributed_discovery --no-tests=fail`
- `ruby .csdlc/prepared/issues/5866/validate-proof-receipt.rb .csdlc/evidence/5866/generation-protobuf-durable/execution-proof.json`

## Failure Semantics

Fail closed on stale dependencies, path overlap, zero tests, invalid evidence, insecure fallback, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
