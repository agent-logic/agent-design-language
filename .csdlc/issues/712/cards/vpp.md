# Validation Planning Prompt

Template: 1.0.0

Issue: 712

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/712/design.md

Diagram: .csdlc/prepared/issues/712/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-startup-simplification",
    "proof_role": "Prove focused CSM, Guardian, Kernel startup and reload behavior plus range diff hygiene.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/712/validate-runtime-startup-simplification.sh"
    ],
    "parallel_group": "runtime-startup",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/712/validate-runtime-startup-simplification.sh`

## Failure Semantics

Fail closed on invalid config or real security-boundary regression; preserve the current live generation for rollback.

## Handoff

Retain typed evidence before convergence.
