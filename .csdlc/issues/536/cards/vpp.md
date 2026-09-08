# Validation Planning Prompt

Template: 1.0.0

Issue: 536

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/536/design.md

Diagram: .csdlc/prepared/issues/536/diagram.mmd

## Selected Lanes

[
  {
    "lane": "sprint8-closeout",
    "proof_role": "Verify exact Sprint 8 membership, explicit no-PR dispositions, residual routing, and ancestry of every merged child before aggregate closeout.",
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
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/536/validate-sprint-closeout.rb"
    ],
    "parallel_group": "sprint8-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/536/validate-sprint-closeout.rb`

## Failure Semantics

Fail closed on missing child readiness, ownership overlap, unmet serial gates, operator-authority gaps, or unsupported completion claims.

## Handoff

Retain typed evidence before convergence.
