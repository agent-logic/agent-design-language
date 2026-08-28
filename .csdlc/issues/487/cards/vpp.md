# Validation Planning Prompt

Template: 1.0.0

Issue: 487

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/487/design.md

Diagram: .csdlc/prepared/issues/487/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-audit-security-packet",
    "proof_role": "Proves #487 design packet readiness, #486 dependency gate, owned-path boundaries, redaction posture, and lifecycle distinction between pre-bind packet proof and future implementation-owned Terraform/readback paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/487/validate-aws-d-baseline.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "prebind-review-readiness",
    "proof_role": "Proves #487 has an issue-owned executable packet validator before design review; this does not claim final implementation exact-head review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/487/validate-aws-d-baseline.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "aws-d-static-contract",
    "proof_role": "After bind, verifies #487 account-foundation Terraform, runbook, retention, encryption, alert routing, and redaction static contract.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/487/validate-aws-d-baseline.sh"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #487 is bound and the implementation creates the complete account-foundation audit/security surfaces."
  },
  {
    "lane": "aws-d-readback",
    "proof_role": "After reviewed live apply or approved readback context, proves CloudTrail, configuration, detection, access-analysis, retention, encryption, alert, and redaction posture.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh",
      "--lane=aws-readonly"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": "Deferred until #486 is terminal, #487 is bound, account-foundation controls exist or a reviewed plan proves intentional absence, and live AWS read-only proof is safe with the approved agent-logic-admin profile."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/487/validate-aws-d-baseline.sh`
- `bash .csdlc/prepared/issues/487/validate-aws-d-baseline.sh`
- `bash .csdlc/prepared/issues/487/validate-aws-d-baseline.sh`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=aws-readonly`

## Failure Semantics

Fail closed if audit gaps remain unexplained, findings lack ownership, retention/encryption/cost posture is implicit, retained proof would expose secrets, or #486 is not terminal and current before binding.

## Handoff

Retain typed evidence before convergence.
