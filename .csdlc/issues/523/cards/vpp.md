# Validation Planning Prompt

Template: 1.0.0

Issue: 523

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/523/design.md

Diagram: .csdlc/prepared/issues/523/diagram.mmd

## Selected Lanes

[
  {
    "lane": "tail07-denominator",
    "proof_role": "Prove the complete successor planning package and dispositions.",
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
      ".csdlc/prepared/issues/523/validate-tail07.rb"
    ],
    "parallel_group": "docs",
    "defer_reason": "The issue-owned validator is an execution deliverable."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed planning diffs.",
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

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/523/validate-tail07.rb`
- `git diff --check`

## Failure Semantics

Fail closed on missing dispositions, duplicate ownership, successor issue creation, or planning claims unsupported by current milestone truth.

## Handoff

Retain typed evidence before convergence.
