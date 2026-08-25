# Validation Planning Prompt

Template: 1.0.0

Issue: 476

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/476/design.md

Diagram: .csdlc/prepared/issues/476/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp27-remediation-regression",
    "proof_role": "Run the issue-owned remediation validator after the truth repair; typed review and finish separately enforce exact-head and terminal authority.",
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
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5848/validate-remediation-regressions.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and conflict artifacts.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/5848/validate-remediation-regressions.rb`
- `git diff --check`

## Failure Semantics

Fail closed on path widening, validation failure, stale review, red required checks, or terminal ambiguity.

## Handoff

Retain typed evidence before convergence.
