# Validation Planning Prompt

Template: 1.0.0

Issue: 537

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/537/design.md

Diagram: .csdlc/prepared/issues/537/diagram.mmd

## Selected Lanes

[
  {
    "lane": "sprint9-readiness",
    "proof_role": "Exact membership, dependency graph, child card readiness, release-tail fail-closed rules, and coordination-only authority",
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
    "budget_seconds": 300,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/537/validate-sprint-readiness.rb"
    ],
    "parallel_group": "sprint9-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/537/validate-sprint-readiness.rb`

## Failure Semantics

Fail closed on missing child readiness, unmet predecessor gates, incomplete root admission, stale or non-proving evidence, or unsupported completion claims.

## Handoff

Retain typed evidence before convergence.
