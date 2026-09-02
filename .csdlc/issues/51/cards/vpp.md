# Validation Planning Prompt

Template: 1.0.0

Issue: 51

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/51/design.md

Diagram: .csdlc/prepared/issues/51/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-51-parent-readiness",
    "proof_role": "Validate current parent closeout readiness, child state snapshot, #264 merge/acceptance blocker, and no-submission truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/51/validate-parent-closeout-readiness.rb"
    ],
    "parallel_group": "sprint8-issue-51",
    "defer_reason": "Run after the current #51 parent readiness packet exists."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/51/validate-parent-closeout-readiness.rb`

## Failure Semantics

Fail closed on child truth mismatch, missing #264 merge/acceptance gate, provider-action ambiguity, secret retention, unsupported public claim, or parent-child truth mismatch.

## Handoff

Retain typed evidence before convergence.
