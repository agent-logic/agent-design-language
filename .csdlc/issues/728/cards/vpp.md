# Validation Planning Prompt

Template: 1.0.0

Issue: 728

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/728/design.md

Diagram: .csdlc/prepared/issues/728/diagram.mmd

## Selected Lanes

[
  {
    "lane": "728-preparation-bundle",
    "proof_role": "Prove #728 has an executable issue-owned readiness denominator, concrete design, fail-closed live runner scaffold, AWS-F module inputs, and zero-residue/authorization gates before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/728/validate_preparation_bundle.sh"
    ],
    "parallel_group": "728-preflight",
    "defer_reason": null
  },
  {
    "lane": "aws-f-saved-plans",
    "proof_role": "Prove exact Terraform saved-plan digests and mutation envelope before any apply.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh"
    ],
    "parallel_group": "728-live",
    "defer_reason": "Requires exact operator-approved AWS inputs and mutable-envelope authorization before apply."
  },
  {
    "lane": "aws-f-live-disposable-proof",
    "proof_role": "Prove private target creation, ALB target health, external request receipt tied to exact instance, reverse destroy, and zero residue.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh"
    ],
    "parallel_group": "728-live",
    "defer_reason": "Requires explicit operator authorization packet naming exact account, region, route, Terraform inputs, cost ceiling, deadline, and destroy selectors."
  },
  {
    "lane": "quality-review-publication",
    "proof_role": "Prove lifecycle truth, exact-head review, diff hygiene, and publication readiness.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 1500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "728-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/728/validate_preparation_bundle.sh`
- `bash docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh`
- `bash docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh`
- `git diff --check`

## Failure Semantics

Fail closed on ambiguous account, state, route, target identity, authorization, cleanup, or residue; never claim live proof from static validation or ALB-only health.

## Handoff

Retain typed evidence before convergence.
