# Validation Planning Prompt

Template: 1.0.0

Issue: 317

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Not run before implementation; live acquisition and deterministic validation remain deferred.

## Lane Inputs

Design: .csdlc/prepared/issues/317/design.md

Diagram: .csdlc/prepared/issues/317/diagram.mmd

## Selected Lanes

[
  {
    "lane": "317-live-observation",
    "proof_role": "Acquire and retain GitHub issue, PR, head, merge, checks, reviews, observation time, repository identity, and response digests for the canonical denominator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/317/validate-closeout-plan.rb",
      "observe"
    ],
    "parallel_group": "317-observe",
    "defer_reason": "Runs after implementation and requires live read-only GitHub access."
  },
  {
    "lane": "317-snapshot-universe",
    "proof_role": "Deterministically validate the retained observation envelope, canonical-to-legacy mapping, exact row denominator, immutable Git identities, and typed-state bindings.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/317/validate-closeout-plan.rb",
      "universe"
    ],
    "parallel_group": "317-local",
    "defer_reason": "Runs after the observation envelope exists."
  },
  {
    "lane": "317-closeout-dag",
    "proof_role": "Validate the complete acyclic merge-gated graph and asynchronous finish/cleanup routing.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/317/validate-closeout-plan.rb",
      "dag"
    ],
    "parallel_group": "317-local",
    "defer_reason": "Runs after implementation."
  },
  {
    "lane": "317-negative-cases",
    "proof_role": "Exercise stale, red, missing-review, non-ancestral, duplicate, ambiguous, unmapped, unknown, cyclic, unowned, self-declared, and closeout-gate cases.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/317/validate-closeout-plan.rb",
      "negative"
    ],
    "parallel_group": "317-local",
    "defer_reason": "Runs after implementation."
  },
  {
    "lane": "317-diff-hygiene",
    "proof_role": "Reject whitespace and conflict artifacts in the bounded documentation packet.",
    "acceptance_ids": [
      "AC-5"
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
    "parallel_group": "317-local",
    "defer_reason": "Runs after implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/317/validate-closeout-plan.rb observe`
- `ruby .csdlc/prepared/issues/317/validate-closeout-plan.rb universe`
- `ruby .csdlc/prepared/issues/317/validate-closeout-plan.rb dag`
- `ruby .csdlc/prepared/issues/317/validate-closeout-plan.rb negative`
- `git diff --check`

## Failure Semantics

Fail closed on incomplete or contradictory authority, missing or duplicate rows, stale or non-ancestral merges, cycles, unowned actions, closeout serialization, or review findings.

## Handoff

Retain typed evidence before convergence.
