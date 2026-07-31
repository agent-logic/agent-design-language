# Validation Planning Prompt

Template: 1.0.0

Issue: 5748

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5748/design.md

Diagram: .csdlc/prepared/issues/5748/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-doctor-and-receipt-parity",
    "proof_role": "Prove closed_out claim-free projections and exact retained receipt equality",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 6000,
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

Seconds: 7200

Tokens: 50000

## Commands

- `git diff --check`

## Failure Semantics

Fail closed on missing receipt, stale identity, unsupported disposition correction, dirty-worktree conflict, doctor failure, receipt mismatch, or any forbidden route.

## Handoff

Retain typed evidence before convergence.
