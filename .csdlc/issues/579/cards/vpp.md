# Validation Planning Prompt

Template: 1.0.0

Issue: 579

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/579/design.md

Diagram: .csdlc/prepared/issues/579/diagram.mmd

## Selected Lanes

[
  {
    "lane": "579-terraform-static",
    "proof_role": "Focused Terraform formatting/static checks for touched AWS-F Terraform roots/modules without backend or cloud mutation, including no AWS-F Route53/ACM executable ownership.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/579/validate-aws-f-corrective.sh",
      "--lane=terraform-static"
    ],
    "parallel_group": "579-local",
    "defer_reason": null
  },
  {
    "lane": "579-security-validator-regression",
    "proof_role": "Proves forbidden world-open Runtime ingress is rejected and egress-only blocks do not mask ingress.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/579/validate-aws-f-corrective.sh",
      "--lane=security-validator-regression"
    ],
    "parallel_group": "579-local",
    "defer_reason": null
  },
  {
    "lane": "579-proof-truth",
    "proof_role": "Checks AWS-F proof/runbook wording for public-edge ownership, local-vs-live proof truth, state isolation, cleanup, rollback, observability, artifact wiring, and Spot resilience boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1200,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/579/validate-aws-f-corrective.sh",
      "--lane=proof-truth"
    ],
    "parallel_group": "579-local",
    "defer_reason": null
  },
  {
    "lane": "579-review-gate-planning",
    "proof_role": "Checks the #579 SRP requires fresh exact-head review before publication without claiming review execution as validation.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/579/validate-aws-f-corrective.sh",
      "--lane=review-gate-planning"
    ],
    "parallel_group": "579-local",
    "defer_reason": null
  },
  {
    "lane": "579-diff-hygiene",
    "proof_role": "Rejects conflict artifacts and whitespace errors across the corrective diff.",
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
    "parallel_group": "579-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/579/validate-aws-f-corrective.sh --lane=terraform-static`
- `bash .csdlc/prepared/issues/579/validate-aws-f-corrective.sh --lane=security-validator-regression`
- `bash .csdlc/prepared/issues/579/validate-aws-f-corrective.sh --lane=proof-truth`
- `bash .csdlc/prepared/issues/579/validate-aws-f-corrective.sh --lane=review-gate-planning`
- `git diff --check`

## Failure Semantics

Fail closed on public-edge ownership regression, validator false pass, overstated proof, advisory-only state isolation, production-resilience overclaim, credential disclosure, paid cloud mutation without approval, or terminal #489 mutation.

## Handoff

Retain typed evidence before convergence.
