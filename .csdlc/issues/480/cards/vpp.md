# Validation Planning Prompt

Template: 1.0.0

Issue: 480

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/480/design.md

Diagram: .csdlc/prepared/issues/480/diagram.mmd

## Selected Lanes

[
  {
    "lane": "480-creation-plan",
    "proof_role": "Prove exact ordered denominator, specifications, titles, dependencies, and existing-issue routing.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/480/validate-wave-creation.rb",
      "plan"
    ],
    "parallel_group": "480-preflight",
    "defer_reason": "Validator is created after binding."
  },
  {
    "lane": "480-negative-recovery",
    "proof_role": "Reject duplicate, conflict, unresolved dependency, extra slot, replay, and malformed partial-recovery cases.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/480/test-validate-wave-creation.rb"
    ],
    "parallel_group": "480-preflight",
    "defer_reason": "Negative suite is created after binding."
  },
  {
    "lane": "480-live-readback",
    "proof_role": "Verify each created issue and the final 45-of-45 immutable live receipt.",
    "acceptance_ids": [
      "AC-3",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/480/validate-wave-creation.rb",
      "live"
    ],
    "parallel_group": "480-live",
    "defer_reason": "Requires created live issues."
  },
  {
    "lane": "480-diff-hygiene",
    "proof_role": "Reject whitespace and conflict artifacts in the bounded packet.",
    "acceptance_ids": [
      "AC-7"
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
    "parallel_group": "480-preflight",
    "defer_reason": "Runs after implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/480/validate-wave-creation.rb plan`
- `ruby .csdlc/prepared/issues/480/test-validate-wave-creation.rb`
- `ruby .csdlc/prepared/issues/480/validate-wave-creation.rb live`
- `git diff --check`

## Failure Semantics

Fail closed before mutation on denominator, identity, title, dependency, digest, or duplicate ambiguity; retain partial receipts and require reviewed forward recovery after any external creation.

## Handoff

Retain typed evidence before convergence.
