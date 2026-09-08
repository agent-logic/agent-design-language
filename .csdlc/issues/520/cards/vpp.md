# Validation Planning Prompt

Template: 1.0.0

Issue: 520

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/520/design.md

Diagram: .csdlc/prepared/issues/520/diagram.mmd

## Selected Lanes

[
  {
    "lane": "review-denominator",
    "proof_role": "Prove every changed path, milestone issue/PR, acceptance surface, and mandatory lane is inventoried and dispositioned.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/520/validate-internal-review.rb",
      "denominator"
    ],
    "parallel_group": "packet",
    "defer_reason": "Validator and packet are #520 execution deliverables."
  },
  {
    "lane": "finding-schema",
    "proof_role": "Prove raw findings, canonical register, synthesis, and acceptance matrix agree exactly.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/520/validate-internal-review.rb",
      "findings"
    ],
    "parallel_group": "packet",
    "defer_reason": "Validator and packet are #520 execution deliverables."
  },
  {
    "lane": "exact-revision-integrity",
    "proof_role": "Prove candidate identity, ancestry, artifact digests, portability, and redaction.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/520/validate-internal-review.rb",
      "integrity"
    ],
    "parallel_group": "packet",
    "defer_reason": "Validator and packet are #520 execution deliverables."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed tracked changes.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 300,
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

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/520/validate-internal-review.rb denominator`
- `ruby .csdlc/prepared/issues/520/validate-internal-review.rb findings`
- `ruby .csdlc/prepared/issues/520/validate-internal-review.rb integrity`
- `git diff --check`

## Failure Semantics

Fail closed on dependency, candidate, denominator, evidence, redaction, schema, review, or exact-revision drift; retain limitations and never convert missing proof into a pass.

## Handoff

Retain typed evidence before convergence.
