# Validation Planning Prompt

Template: 1.0.0

Issue: 486

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Before bind, run the issue-owned prebind packet validator only. After bind, run Terraform static validation and approved AWS readback lanes once their implementation-owned paths/resources exist.

## Lane Inputs

Design: .csdlc/prepared/issues/486/design.md

Diagram: .csdlc/prepared/issues/486/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-bootstrap-packet",
    "proof_role": "Proves #486 design packet readiness, #485 terminal dependency evidence, and lifecycle distinction between pre-bind packet proof and future implementation-owned Terraform/readback paths.",
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
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/486/validate-aws-c-bootstrap.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "terraform-bootstrap-static",
    "proof_role": "After bind, proves Terraform formatting, provider pins, backend-free initialization, and validate proof for the bootstrap root.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-c/run-terraform-bootstrap-readbacks.sh",
      "--lane",
      "terraform-static"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #486 is bound and the implementation creates infra/aws/bootstrap plus the issue-owned readback/validation script."
  },
  {
    "lane": "aws-bootstrap-readback",
    "proof_role": "After reviewed live apply, proves backend identity, lock/versioning, recovery rehearsal, provider pins, and state isolation through read-only AWS readbacks.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-c/run-terraform-bootstrap-readbacks.sh",
      "--lane",
      "aws-readback"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": "Deferred until #486 is bound, the bootstrap resources exist or the reviewed plan proves they are intentionally absent, and live AWS read-only proof is safe to run with the approved agent-logic-admin profile."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/486/validate-aws-c-bootstrap.sh`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-c/run-terraform-bootstrap-readbacks.sh --lane terraform-static`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-c/run-terraform-bootstrap-readbacks.sh --lane aws-readback`

## Failure Semantics

Fail closed if backend ownership is ambiguous, recovery cannot be rehearsed, reviewed plan differs before apply, or credentials would enter retained evidence.

## Handoff

Retain typed evidence before convergence.
