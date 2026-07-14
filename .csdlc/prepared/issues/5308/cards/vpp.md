# Validation Planning Prompt

Template: 1.0.0

Issue: 5308

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: docs/architecture/csdlc-v2/gate10d3/DESIGN.md

Diagram: docs/architecture/csdlc-v2/gate10d3/DIAGRAM.mmd

## Selected Lanes

[
  {
    "lane": "rollback-sunset-decision",
    "proof_role": "Evaluate trusted current time, exact approval, current v2 health, and protected rollback scope from typed evidence",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
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
      "csdlc-v2/operator/gate10d3-sunset-request.json"
    ],
    "parallel_group": "authority",
    "defer_reason": null
  },
  {
    "lane": "rollback-sunset",
    "proof_role": "After a passing typed sunset decision, prove the exact rollback removal and intact importer",
    "acceptance_ids": [
      "AC-4",
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

- `csdlc-sunset evaluate --repo . --request csdlc-v2/operator/gate10d3-sunset-request.json`
- `cargo test --manifest-path csdlc-v2/Cargo.toml`

## Failure Semantics

Fail closed with zero rollback mutation when any date, approval, scope, or health input is missing or ambiguous.

## Handoff

Retain typed evidence before convergence.
