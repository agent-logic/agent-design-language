# Validation Planning Prompt

Template: 1.0.0

Issue: 316

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute focused deterministic documentation validation for both milestone packages.

## Lane Inputs

Design: .csdlc/prepared/issues/316/design.md

Diagram: .csdlc/prepared/issues/316/diagram.mmd

## Selected Lanes

[
  {
    "lane": "planning-package",
    "proof_role": "Validate the complete canonical package, planned IDs, dependencies, dispositions, handoffs, and no-.adl boundary.",
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
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/316/validate-v0921-plan.rb"
    ],
    "parallel_group": "316-1-plan",
    "defer_reason": "Validator is authored in the bound issue worktree."
  },
  {
    "lane": "codefriend-beta1-package",
    "proof_role": "Validate the complete v0.92.2 standard package, Beta 1 exit-bar coverage, planned IDs, dependencies, deferrals, release tail, and no-.adl or Drive runtime dependency.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/316/validate-v0922-codefriend-plan.rb"
    ],
    "parallel_group": "316-1-beta1",
    "defer_reason": "Validator is authored in the bound issue worktree."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Validate exact planning diff hygiene.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "316-2-diff",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/316/validate-v0921-plan.rb`
- `ruby .csdlc/prepared/issues/316/validate-v0922-codefriend-plan.rb`
- `git diff --check`

## Failure Semantics

Fail closed on issue creation, tracked .adl or Drive dependencies, missing or duplicate planned IDs, unresolved dependencies, unsupported claims, scope drift, stale review, or ambiguous candidate disposition.

## Handoff

Retain typed evidence before convergence.
