# Validation Planning Prompt

Template: 1.0.0

Issue: 517

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/517/design.md

Diagram: .csdlc/prepared/issues/517/diagram.mmd

## Selected Lanes

[
  {
    "lane": "quality-denominator",
    "proof_role": "Prove the complete required-lane denominator for the exact candidate.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/517/validate-quality-gate.rb"
    ],
    "parallel_group": "quality",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "zero-test-shield",
    "proof_role": "Demonstrate fail-closed handling for skipped, absent, zero-test, stale, and insufficient evidence.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/517/validate-quality-gate.rb",
      "--negative"
    ],
    "parallel_group": "quality",
    "defer_reason": "The issue-owned negative validator is an execution deliverable."
  },
  {
    "lane": "exact-scope",
    "proof_role": "Bind the decision, exceptions, and artifacts to the exact candidate.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/517/validate-quality-gate.rb`
- `ruby .csdlc/prepared/issues/517/validate-quality-gate.rb --negative`
- `git diff --check`

## Failure Semantics

Fail closed on an unmet predecessor, incomplete denominator, non-proving lane, candidate drift, or unowned exception.

## Handoff

Retain typed evidence before convergence.
