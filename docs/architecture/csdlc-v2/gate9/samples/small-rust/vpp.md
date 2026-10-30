# Validation Planning Prompt

Template: 1.0.0

Issue: 9002

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: small-rust/design.md

Diagram: small-rust/diagram.mmd

## Selected Lanes

[
  {
    "lane": "sample-focused",
    "proof_role": "small-rust",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--test",
      "gate9"
    ],
    "parallel_group": "gate9-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --test gate9`

## Failure Semantics

Fail closed, retain evidence, repair, and retry idempotently.

## Handoff

Retain typed evidence before convergence.
