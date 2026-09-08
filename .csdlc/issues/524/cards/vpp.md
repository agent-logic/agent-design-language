# Validation Planning Prompt

Template: 1.0.0

Issue: 524

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/524/design.md

Diagram: .csdlc/prepared/issues/524/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-524-tail08",
    "proof_role": "Prove release-plan/checklist denominator parity, exact tail order, operator gates, asynchronous bookkeeping, and diff hygiene.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/524/validate-tail08.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash .csdlc/prepared/issues/524/validate-tail08.sh`

## Failure Semantics

Fail closed on denominator disagreement, reordered or omitted tail work, implicit release authority, or closeout-dependent execution.

## Handoff

Retain typed evidence before convergence.
