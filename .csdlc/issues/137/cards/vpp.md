# Validation Planning Prompt

Template: 1.0.0

Issue: 137

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/137/design.md

Diagram: .csdlc/prepared/issues/137/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp04-native-workflow-contract",
    "proof_role": "Prove exact checkout, three-platform production, distinct artifacts, live hosted attestation, aggregate validation, pinned actions, bounded timeouts, fail-closed behavior, and repository path-policy compatibility.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 500,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/137/validate-workflow.rb"
    ],
    "parallel_group": "workflow",
    "defer_reason": "The issue-owned workflow and focused validator are created only after typed binding."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors and unintended tracked churn.",
    "acceptance_ids": [
      "AC-4",
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
    "parallel_group": "workflow",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `ruby .csdlc/prepared/issues/137/validate-workflow.rb`
- `git diff --check`

## Failure Semantics

Fail closed on invalid or mutable checkout input, missing matrix coverage, artifact collision, missing receipt fragments, aggregation failure, action pin drift, widened permissions, out-of-scope paths, failed validation, review drift, or hosted CI failure.

## Handoff

Retain typed evidence before convergence.
