# Validation Planning Prompt

Template: 1.0.0

Issue: 311

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/311/design.md

Diagram: .csdlc/prepared/issues/311/diagram.mmd

## Selected Lanes

[
  {
    "lane": "semantic-quality-matrix",
    "proof_role": "Regenerate and validate the exact feature/critical-path denominator and independently verify GitHub, Git, review, merge, platform, and typed-terminal evidence for every row.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/311/validate-quality-gate.rb",
      "matrix"
    ],
    "parallel_group": "311-gate",
    "defer_reason": "Created in the bound worktree after #310 reaches terminal authority."
  },
  {
    "lane": "quality-negative-suite",
    "proof_role": "Pass forged and stale evidence classes through the production validator and require deterministic rejection without weakening positive semantics.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/311/test-validate-quality-gate.rb"
    ],
    "parallel_group": "311-negative",
    "defer_reason": "Created with the production validator in the bound worktree."
  },
  {
    "lane": "docs-schema-diff",
    "proof_role": "Validate JSON/YAML/Markdown packet structure, exact changed paths, links, and diff hygiene.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "311-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/311/validate-quality-gate.rb matrix`
- `ruby .csdlc/prepared/issues/311/test-validate-quality-gate.rb`
- `git diff --check`

## Failure Semantics

Fail closed on an open or unreconciled predecessor, incomplete denominator, ambiguous repository identity, unverifiable reviewed or merged revision, fabricated or self-attested evidence, failed negative, missing platform proof, unresolved blocker, stale typed state, or changed candidate after review.

## Handoff

Retain typed evidence before convergence.
