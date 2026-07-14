# Validation Planning Prompt

Template: 1.0.0

Issue: 5307

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/architecture/csdlc-v2/gate10d4/DESIGN.md

Diagram: docs/architecture/csdlc-v2/gate10d4/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "importer-sunset-decision",
    "proof_role": "Evaluate trusted current time, exact approval, migration completeness, active contracts, and current v2 health from typed evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "csdlc-sunset",
      "evaluate",
      "--repo",
      ".",
      "--request",
      "csdlc-v2/operator/gate10d4-sunset-request.json"
    ],
    "parallel_group": "authority",
    "defer_reason": null
  },
  {
    "lane": "importer-sunset",
    "proof_role": "After a passing typed sunset decision, prove durable migration evidence and green v2 after importer removal",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 1140,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `csdlc-sunset evaluate --repo . --request csdlc-v2/operator/gate10d4-sunset-request.json`
- `cargo test --manifest-path csdlc-v2/Cargo.toml`

## Failure Semantics

Fail closed with zero importer mutation when any date, approval, migration, contract, or health input is missing or ambiguous.

## Handoff

Retain typed evidence before convergence.
