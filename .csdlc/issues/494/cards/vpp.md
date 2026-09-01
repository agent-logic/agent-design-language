# Validation Planning Prompt

Template: 1.0.0

Issue: 494

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/494/design.md

Diagram: .csdlc/prepared/issues/494/diagram.mmd

## Selected Lanes

[
  {
    "lane": "gcp-e-issue-validator",
    "proof_role": "Proves #494 issue-owned validation for split GCP-E support and instance roots, cost cap, SSH route, and cleanup selectors.",
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
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh",
      "--lane=all"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "gcp-e-terraform-fmt",
    "proof_role": "Formatting proof for the #494 split Terraform modules and roots.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "terraform",
      "fmt",
      "-check",
      "-recursive",
      "infra/gcp/workloads/modules/gpu-smoke-support",
      "infra/gcp/workloads/modules/gpu-smoke-instance",
      "infra/gcp/workloads/gpu-smoke-support",
      "infra/gcp/workloads/gpu-smoke-instance"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "gcp-e-support-terraform-validate",
    "proof_role": "Terraform schema validation for the stable #494 GCP-E support root.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "terraform",
      "-chdir=infra/gcp/workloads/gpu-smoke-support",
      "validate"
    ],
    "parallel_group": "terraform",
    "defer_reason": null
  },
  {
    "lane": "gcp-e-instance-terraform-validate",
    "proof_role": "Terraform schema validation for the disposable #494 GCP-E instance root.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "terraform",
      "-chdir=infra/gcp/workloads/gpu-smoke-instance",
      "validate"
    ],
    "parallel_group": "terraform",
    "defer_reason": null
  },
  {
    "lane": "gcp-e-diff-hygiene",
    "proof_role": "Diff hygiene proof for #494 before exact review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh --lane=all`
- `terraform fmt -check -recursive infra/gcp/workloads/modules/gpu-smoke-support infra/gcp/workloads/modules/gpu-smoke-instance infra/gcp/workloads/gpu-smoke-support infra/gcp/workloads/gpu-smoke-instance`
- `terraform -chdir=infra/gcp/workloads/gpu-smoke-support validate`
- `terraform -chdir=infra/gcp/workloads/gpu-smoke-instance validate`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed if paid authorization, quota/capacity, GPU inference, telemetry, cost ceiling, deadline, or zero-resource cleanup cannot be proven without credential disclosure.

## Handoff

Retain typed evidence before convergence.
