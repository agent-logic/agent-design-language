# Validation Planning Prompt

Template: 1.0.0

Issue: 594

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/594/design.md

Diagram: .csdlc/prepared/issues/594/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-log-archive",
    "proof_role": "Prove bounded Vector S3 configuration, redaction, failure telemetry, and Runtime survival with nonzero focused tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/594/validate-runtime-log-archive.sh"
    ],
    "parallel_group": "runtime",
    "defer_reason": "The issue-owned wrapper exists; its named s3_archive cases are issue #594 implementation deliverables."
  },
  {
    "lane": "terraform-log-archive",
    "proof_role": "Prove Terraform formatting and static validity for the isolated archive module.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "terraform",
      "-chdir=infra/aws/runtime/log-archive",
      "validate"
    ],
    "parallel_group": "terraform",
    "defer_reason": "The log-archive Terraform module is an issue #594 implementation deliverable."
  },
  {
    "lane": "live-aws-archive",
    "proof_role": "Under explicit authorization, prove business-account identity, delivery, controls, retrieval, redaction, and cleanup receipts.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/594/validate-live-aws.sh"
    ],
    "parallel_group": "live-aws",
    "defer_reason": "The issue-owned preflight exists; paid live AWS proof remains blocked until implementation and separate operator authorization."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove exact branch diff whitespace hygiene.",
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
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/594/validate-runtime-log-archive.sh`
- `terraform -chdir=infra/aws/runtime/log-archive validate`
- `bash .csdlc/prepared/issues/594/validate-live-aws.sh`
- `git diff --check`

## Failure Semantics

Fail closed on readiness coupling, unbounded resource use, sensitive archive output, overbroad IAM, zero-test validation, stale review, or unauthorized cloud action.

## Handoff

Retain typed evidence before convergence.
