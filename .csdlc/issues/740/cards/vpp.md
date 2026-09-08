# Validation Planning Prompt

Template: 1.0.0

Issue: 740

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/740/design.md

Diagram: .csdlc/prepared/issues/740/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-gcp-b1-corrective-contract",
    "proof_role": "Focused local validator for credential hygiene, authorization binding, Terraform-backend canary evidence shape, and post-merge truth separation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/740/validate_gcp_b1_corrective.sh"
    ],
    "parallel_group": "serial",
    "defer_reason": null
  },
  {
    "lane": "live-gcp-b1-corrective-proof",
    "proof_role": "Bounded live GCP proof for legacy-key disposition, Terraform-backend canary write/recovery, and bucket posture readback.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 2400,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh"
    ],
    "parallel_group": "serial",
    "defer_reason": "Live GCP proof runs after script repair and authenticated approval evidence are in place."
  },
  {
    "lane": "typed-issue-validation",
    "proof_role": "Typed lifecycle/card validation before review and publication.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/740/validate_typed_issue.sh"
    ],
    "parallel_group": "serial",
    "defer_reason": "Run after implementation evidence and cards are current."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/740/validate_gcp_b1_corrective.sh`
- `bash .csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh`
- `bash .csdlc/prepared/issues/740/validate_typed_issue.sh`

## Failure Semantics

Fail closed on any retained credential material, self-issued authorization, missing legacy-key disposition, non-Terraform canary write, shared persistent auth/cache usage, or live readback contradiction.

## Handoff

Retain typed evidence before convergence.
