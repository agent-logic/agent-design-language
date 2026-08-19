# Validation Planning Prompt

Template: 1.0.0

Issue: 256

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/256/design.md

Diagram: .csdlc/prepared/issues/256/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue256-preparation-gate",
    "proof_role": "Validate #256 successor authority, packet input-only truth, Observatory/#345 gates, terminal canonical ancestral #414, exact habitable Runtime denominator, excluded predecessor implementation, and #341/#343 serialization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/evidence/256/validate_preparation_gate.py"
    ],
    "parallel_group": "local",
    "defer_reason": "#414 terminal evidence and final birthday-demo integration remain deferred; this lane currently proves coordination truth only."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `python3 .csdlc/evidence/256/validate_preparation_gate.py`

## Failure Semantics

Fail closed before bind/implementation when Observatory, #345, or scope-exclusion gates are unsatisfied.

## Handoff

Retain typed evidence before convergence.
