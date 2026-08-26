# Validation Planning Prompt

Template: 1.0.0

Issue: 117

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/117/design.md

Diagram: .csdlc/prepared/issues/117/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-terminal-cache-validator",
    "proof_role": "Verify #117 preparation consumes canonical terminal #271/#114/#115/#116/#279/#280/#281/#282 evidence and remains parent-only.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/117/validate_preparation_bundle.py"
    ],
    "parallel_group": "117-parent-proof",
    "defer_reason": null
  },
  {
    "lane": "parent-closeout-validator",
    "proof_role": "Verify #117 parent closeout evidence, terminal child/prerequisite caches, exact #282 integrated qualification truth, residual risks, non-claims, and #110 handoff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/evidence/117/validate_parent_closeout.py"
    ],
    "parallel_group": "117-parent-proof",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace or patch artifacts before exact-head review and publication.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "117-parent-proof",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `python3 .csdlc/prepared/issues/117/validate_preparation_bundle.py`
- `python3 .csdlc/evidence/117/validate_parent_closeout.py`
- `git diff --check`

## Failure Semantics

Fail closed on stale child terminal cache, non-canonical child receipt, missing merged disposition, missing issue closure, umbrella overclaim, or any product/runtime/cloud/provider scope expansion.

## Handoff

Retain typed evidence before convergence.
