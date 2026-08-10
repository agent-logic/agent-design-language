# Validation Planning Prompt

Template: 1.0.0

Issue: 149

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/149/design.md

Diagram: .csdlc/prepared/issues/149/diagram.mmd

## Selected Lanes

[
  {
    "lane": "corp-u-coordination",
    "proof_role": "Validate exact child denominator, readiness, dependency, terminal, and no-product-ownership truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/149/validate-outcome.rb"
    ],
    "parallel_group": "corp-u-coordination",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/149/validate-outcome.rb`

## Failure Semantics

Fail closed on dependency drift, path collision, authority ambiguity, missing producer evidence, validation failure, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
