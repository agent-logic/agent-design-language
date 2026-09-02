# Validation Planning Prompt

Template: 1.0.0

Issue: 342

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/342/design.md

Diagram: .csdlc/prepared/issues/342/diagram.mmd

## Selected Lanes

[
  {
    "lane": "wp24a-preparation-readiness",
    "proof_role": "Prove AC-6 preparation only: canonical identity, current dependency boundaries, pre-bind collision gates, initialized/unbound lifecycle truth, and the honest zero-complete/one-candidate/nine-absent denominator. This lane cannot satisfy AC-1 through AC-5.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/342/validate-readiness.rb"
    ],
    "parallel_group": "preparation",
    "defer_reason": null
  },
  {
    "lane": "episode-package-contract",
    "proof_role": "After authorized bind and implementation, prove AC-1, AC-2, and AC-4 across the exact ten-package denominator; no preparation result satisfies this lane.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/342/validate-episode-packages.rb"
    ],
    "parallel_group": "product",
    "defer_reason": "The issue-owned ten-package validator and Episodes 002 through 010 are created only after separately authorized typed execution binding; current preparation claims zero complete product packages."
  },
  {
    "lane": "integrated-podcast-proof",
    "proof_role": "After all ten packages exist, prove AC-3 and AC-5 through integrated local parity, exact-head review evidence, and publication-negative assertions; no preparation result satisfies this lane.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/342/validate-integrated-podcast-proof.rb"
    ],
    "parallel_group": "product",
    "defer_reason": "The issue-owned integrated validator is created only after separately authorized typed execution binding and cannot run until the complete ten-package denominator and exact-head review inputs exist."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/342/validate-readiness.rb`
- `ruby .csdlc/prepared/issues/342/validate-episode-packages.rb`
- `ruby .csdlc/prepared/issues/342/validate-integrated-podcast-proof.rb`

## Failure Semantics

Fail closed on identity drift, unresolved path ownership, incomplete package denominator, missing rights/redaction/provenance, validation failure, stale review, or any external/publication action requirement.

## Handoff

Retain typed evidence before convergence.
