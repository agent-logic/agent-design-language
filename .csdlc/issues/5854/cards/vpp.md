# Validation Planning Prompt

Template: 1.0.0

Issue: 5854

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5854/design.md

Diagram: .csdlc/prepared/issues/5854/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v092-sprint5-readiness",
    "proof_role": "Prove current Sprint 5 membership, four-child operative closeout scope, WP-20 release-tail handoff, bind-safe child contracts, dependency gates, publication boundaries, umbrella ownership, and digest-bound live GitHub provenance.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5854/validate-sprint-readiness.rb"
    ],
    "parallel_group": "v092-docs",
    "defer_reason": "The contract checks are deterministic over retained inputs, but snapshot freshness depends on wall-clock time and observed GitHub state."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5854/validate-sprint-readiness.rb`

## Failure Semantics

Fail closed on overlapping ownership, unmet serial gates, missing child readiness, or unsupported completion claims.

## Handoff

Retain typed evidence before convergence.
