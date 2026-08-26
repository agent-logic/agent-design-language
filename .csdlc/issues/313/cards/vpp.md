# Validation Planning Prompt

Template: 1.0.0

Issue: 313

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/313/design.md

Diagram: .csdlc/prepared/issues/313/diagram.mmd

## Selected Lanes

[
  {
    "lane": "internal-review-packet",
    "proof_role": "Validate exact repository and SHA identity, dependency gates, nine lane completions, finding schema, digests, links, redaction, and claim boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5846/validate-internal-review.rb"
    ],
    "parallel_group": "packet",
    "defer_reason": "Runs after the exact target SHA and review packet exist."
  },
  {
    "lane": "review-quality-meta-review-receipt",
    "proof_role": "Validate the retained independent meta-review identity, exact packet and target digests, coverage and severity evaluation, limitations, and zero unresolved actionable review-quality findings.",
    "acceptance_ids": [
      "AC-6",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5846/validate-internal-review.rb",
      "--require-meta-review"
    ],
    "parallel_group": "meta-review-receipt",
    "defer_reason": "Runs only after an independent reviewer has produced the retained meta-review; the validator does not author or approve that review."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and patch hygiene errors in the review packet, validator, and milestone entrypoint.",
    "acceptance_ids": [
      "AC-7"
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

- `ruby .csdlc/prepared/issues/5846/validate-internal-review.rb`
- `ruby .csdlc/prepared/issues/5846/validate-internal-review.rb --require-meta-review`
- `git diff --check`

## Failure Semantics

Fail closed before review authority on dependency drift, wrong repository or SHA, incomplete lane coverage, invalid finding provenance, digest mismatch, evidence-link failure, private-data leakage, or unresolved meta-review findings.

## Handoff

Retain typed evidence before convergence.
