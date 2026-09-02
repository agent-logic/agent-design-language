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
    "proof_role": "Prove redacted key/batch contract, disabled S3 startup health, isolated 512 MiB drop-newest buffering, bounded retry telemetry, and continued Runtime/master-log/CloudWatch progress under S3 outage.",
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
    "proof_role": "Prove recursive Terraform formatting, backend-free initialization, static validity, lifecycle controls, and exact-prefix publisher policy.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/594/validate-terraform-log-archive.sh"
    ],
    "parallel_group": "terraform",
    "defer_reason": "The issue-owned wrapper exists; the log-archive Terraform module is an issue #594 implementation deliverable."
  },
  {
    "lane": "live-aws-archive",
    "proof_role": "Under explicit authorization, verify exact business account identity, bucket public block/versioning/encryption/lifecycle, encrypted proof-object metadata, retrieval into issue evidence, and bounded redaction inspection without printing content.",
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
    "defer_reason": "The issue-owned validator exists; paid live AWS proof remains blocked until implementation and separate operator authorization."
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
- `bash .csdlc/prepared/issues/594/validate-terraform-log-archive.sh`
- `bash .csdlc/prepared/issues/594/validate-live-aws.sh`
- `git diff --check`

## Failure Semantics

Fail closed on readiness coupling, unbounded resource use, sensitive archive output, overbroad IAM, zero-test validation, stale review, or unauthorized cloud action.

## Handoff

Retain typed evidence before convergence.
