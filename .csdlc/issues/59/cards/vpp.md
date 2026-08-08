# Validation Planning Prompt

Template: 1.0.0

Issue: 59

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/59/design.md

Diagram: .csdlc/prepared/issues/59/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-59-routing-diff-hygiene",
    "proof_role": "Reject malformed tracked routing artifacts while keeping validation bounded to the authority package.",
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
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `git diff --check`

## Failure Semantics

Fail closed if source authority is ambiguous, historical blocked-goal truth would be rewritten, ADL policy would be weakened, or a repository implementation is proposed without an owning executable seam.

## Handoff

Retain typed evidence before convergence.
