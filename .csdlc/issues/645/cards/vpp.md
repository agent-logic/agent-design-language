# Validation Planning Prompt

Template: 1.0.0

Issue: 645

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/645/design.md

Diagram: .csdlc/prepared/issues/645/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-stacked-closing-relation-regression",
    "proof_role": "Prove closing-mode publication rejects a non-default-base PR whose body has a closing keyword but whose GitHub closing relation readback is absent.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/645/validate-stacked-closing-relation.sh"
    ],
    "parallel_group": "csdlc-v2-publication",
    "defer_reason": "The issue-owned wrapper and exact regression test are #645 implementation deliverables."
  },
  {
    "lane": "csdlc-v2-publication-format-hygiene",
    "proof_role": "Confirm Rust formatting and diff hygiene for the changed publication surfaces.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "csdlc-v2-publication",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/645/validate-stacked-closing-relation.sh`
- `git diff --check`

## Failure Semantics

Fail closed on missing live GitHub closing relation, ambiguous linkage source, non-default-base terminal overclaim, raw gh lifecycle workaround, stale PR-state truth, or zero-test validation.

## Handoff

Retain typed evidence before convergence.
