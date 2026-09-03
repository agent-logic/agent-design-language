# Validation Planning Prompt

Template: 1.0.0

Issue: 508

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/508/design.md

Diagram: .csdlc/prepared/issues/508/diagram.mmd

## Selected Lanes

[
  {
    "lane": "drt-c-readiness",
    "proof_role": "Prove #508 design/readiness names terminal #507 and the Runtime-authentic Observatory boundary before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/508/validate-readiness.rb"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "drt-c-focused",
    "proof_role": "Prove failure matrix, Runtime-authentic Observatory, bounded soak, exact revision, synthesis, and cleanup-zero.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 6000,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/508/validate-implementation.rb"
    ],
    "parallel_group": "qualification",
    "defer_reason": "Requires implementation/qualification evidence; any paid or external proof remains operator-gated."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/508/validate-readiness.rb`
- `ruby .csdlc/prepared/issues/508/validate-implementation.rb`

## Failure Semantics

Fail closed on stale dependency truth, synthetic Observatory evidence, failed negative behavior, unbounded soak, missing cleanup, or evidence that does not bind the exact Runtime revision.

## Handoff

Retain typed evidence before convergence.
