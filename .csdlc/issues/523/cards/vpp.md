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
    "proof_role": "Prove the complete successor planning denominator, dependency integrity, deferrals, feature routing, and non-creation boundary.",
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
    "defer_reason": "Runs during execution against the post-#522 planning candidate."
  },
  {
    "lane": "tail07-negative",
    "proof_role": "Reject duplicate, missing, stale, machine-local, and silently promoted planning mutations.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/523/validate-tail07.rb",
      "--negative"
    ],
    "parallel_group": "docs",
    "defer_reason": null
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
- `ruby .csdlc/prepared/issues/523/validate-tail07.rb --negative`
- `git diff --check`

## Failure Semantics

Fail closed on missing dispositions, duplicate ownership, successor issue creation, or planning claims unsupported by current milestone truth.

## Handoff

Retain typed evidence before convergence.
