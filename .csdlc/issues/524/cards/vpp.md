# Validation Planning Prompt

Template: 1.0.0

Issue: 524

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/524/design.md

Diagram: .csdlc/prepared/issues/524/diagram.mmd

## Selected Lanes

[
  {
    "lane": "tail08-denominator",
    "proof_role": "Prove denominator parity, tail order, operator gates, and asynchronous bookkeeping.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/524/validate-tail08.rb"
    ],
    "parallel_group": "docs",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed documentation diffs.",
    "acceptance_ids": [
      "AC-6"
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
    "parallel_group": "docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/524/validate-tail08.rb`
- `git diff --check`

## Failure Semantics

Fail closed on denominator disagreement, reordered or omitted tail work, implicit release authority, or closeout-dependent execution.

## Handoff

Retain typed evidence before convergence.
