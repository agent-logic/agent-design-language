# Validation Planning Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/602/design.md

Diagram: .csdlc/prepared/issues/602/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-agent-admission",
    "proof_role": "Prove the complete csmctl and Runtime agent lifecycle, authenticated transport, validation, idempotence, atomic persistence, freeze-dry integrity, rehydration, roster projection, OpenAPI parity, and diff hygiene.",
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
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/602/validate-focused.sh"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "live-wuji-agent-admission",
    "proof_role": "Prove add, duplicate, freeze-dry migration, rehydration, roster health, Shepherd preservation, and restart recovery against installed Wuji gemma4:e4b-mlx.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/602/validate-live-wuji.sh"
    ],
    "parallel_group": "live",
    "defer_reason": "Runs after focused proof and exact candidate deployment using the existing installed model without paid cloud resources."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/602/validate-focused.sh`
- `bash .csdlc/prepared/issues/602/validate-live-wuji.sh`

## Failure Semantics

Fail closed on unauthorized or invalid admission unavailable models persistence/live divergence Shepherd mutation secret exposure exact-head drift or any proof that requires init-file editing or Runtime restart for first admission.

## Handoff

Retain typed evidence before convergence.
