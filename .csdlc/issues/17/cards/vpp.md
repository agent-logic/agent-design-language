# Validation Planning Prompt

Template: 1.0.0

Issue: 17

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/17/design.md

Diagram: .csdlc/prepared/issues/17/diagram.mmd

## Selected Lanes

[
  {
    "lane": "doctor-readiness-regression",
    "proof_role": "Prove issue-specific doctor readiness and valid workflow behavior through the focused integration target.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2`

## Failure Semantics

Fail closed on repository drift, unroutable owned modules, unavailable undeclared validators, or a zero issue-specific proof denominator.

## Handoff

Retain typed evidence before convergence.
