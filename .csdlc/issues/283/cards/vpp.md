# Validation Planning Prompt

Template: 1.0.0

Issue: 283

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/283/design.md

Diagram: .csdlc/prepared/issues/283/diagram.mmd

## Selected Lanes

[
  {
    "lane": "adr0065-evidence-reconciliation",
    "proof_role": "Execute the issue-owned validator to check terminal/live state, non-empty manifests, referenced artifacts, and issue-local reconciliation boundaries without editing shared ADR docs.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      ".csdlc/prepared/issues/283/validate-adr0065-evidence.sh"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `.csdlc/prepared/issues/283/validate-adr0065-evidence.sh`

## Failure Semantics

Fail closed on missing terminal linkage, stale typed terminal cache, empty manifests, missing referenced artifacts, or any attempted ADR acceptance/shared-doc mutation.

## Handoff

Retain typed evidence before convergence.
