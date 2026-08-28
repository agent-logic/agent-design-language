# Validation Planning Prompt

Template: 1.0.0

Issue: 488

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/488/design.md

Diagram: .csdlc/prepared/issues/488/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-aws-adoption-packet",
    "proof_role": "Proves #488 design packet readiness, #487 terminal dependency gate, owned-path boundaries, one-owner adoption-register invariant, cleanup/deletion stop conditions, and lifecycle distinction between pre-bind packet proof and future implementation/live readback proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/488/validate-aws-e-adoption-register.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "prebind-aws-adoption-readback-static",
    "proof_role": "Proves the AWS-E readback entrypoint has a static non-credentialed mode and reports cloud-mutation and credential-retention posture without requiring AWS credentials.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/488/run-aws-e-readback.sh",
      "--lane=static"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "prebind-review-readiness",
    "proof_role": "Proves #488 has issue-owned executable packet validators before design approval; this does not claim final implementation exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/488/validate-aws-e-adoption-register.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "aws-e-register-static",
    "proof_role": "After bind, verifies the implemented adoption register, required dispositions, one-owner invariant, non-goal boundaries, retained evidence, and redaction posture.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/488/validate-aws-e-adoption-register.sh"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #488 is bound and the implementation creates the complete adoption-register and evidence surfaces."
  },
  {
    "lane": "aws-e-live-readback",
    "proof_role": "After reviewed implementation and approved AWS read-only context, reconciles live AWS inventory against the adopted register with redacted retained evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/488/run-aws-e-readback.sh",
      "--lane=inventory-readonly"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": "Deferred until #488 is bound, the reviewed adoption register exists, and the approved agent-logic-admin AWS read-only context is selected without exposing credential contents."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/488/validate-aws-e-adoption-register.sh`
- `bash .csdlc/prepared/issues/488/run-aws-e-readback.sh --lane=static`
- `bash .csdlc/prepared/issues/488/validate-aws-e-adoption-register.sh`
- `bash .csdlc/prepared/issues/488/validate-aws-e-adoption-register.sh`
- `bash .csdlc/prepared/issues/488/run-aws-e-readback.sh --lane=inventory-readonly`

## Failure Semantics

Fail closed if a resource may belong to website or retained evidence, dual management is possible, deletion authority is missing, live and declared state cannot be reconciled, evidence would expose credentials or sensitive values, or downstream runtime/CloudFormation work would be absorbed.

## Handoff

Retain typed evidence before convergence.
