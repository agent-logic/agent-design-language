# Validation Planning Prompt

Template: 1.0.0

Issue: 495

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/495/design.md

Diagram: .csdlc/prepared/issues/495/diagram.mmd

## Selected Lanes

[
  {
    "lane": "xcl-01-governed-validator",
    "proof_role": "After bind, proves #495 denominator inventory, portable contract, AWS/GCP Terraform surfaces, provider-specific identity/network/IAM differences, CloudFormation rollback retention, redacted proof packet, and explicit paid/live proof gating. During initialized readiness, this lane is an issue-owned deferred validator target paired with the generated prebind validator already executed before design approval.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh",
      "--lane=all"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #495 is bound and implementation creates the issue-owned governed validator plus complete cross-cloud conversion surfaces."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash docs/milestones/v0.92.1/evidence/cloud/xcl-01/validate-xcl-01-cross-cloud-runtime-terraform.sh --lane=all`

## Failure Semantics

Fail closed if a template behavior is unmapped, provider security is hidden, CloudFormation rollback is weakened before AWS-G, cleanup proof is incomplete, live paid proof lacks explicit authorization, or #494/#496/DRT-D scope is absorbed.

## Handoff

Retain typed evidence before convergence.
