# Validation Planning Prompt

Template: 1.0.0

Issue: 5855

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5855/design.md

Diagram: .csdlc/prepared/issues/5855/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v092-sprint2-readiness",
    "proof_role": "Prove Sprint 2 membership, dependency order, canonical TLS ancestry, current typed child readiness, FastWork binding policy, and asynchronous closeout separation.",
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
      "ruby",
      ".csdlc/prepared/issues/5855/validate-sprint-readiness.rb"
    ],
    "parallel_group": "v092-sprint2",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5855/validate-sprint-readiness.rb`

## Failure Semantics

Fail closed on overlapping ownership, unmet serial gates, missing child readiness, or unsupported completion claims.

## Handoff

Retain typed evidence before convergence.
