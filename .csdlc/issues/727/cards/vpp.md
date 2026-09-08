# Validation Planning Prompt

Template: 1.0.0

Issue: 727

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .adl/requests/727/design.md

Diagram: .adl/requests/727/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-727-local-readiness",
    "proof_role": "Run the #727-owned local readiness wrapper, which checks issue-local C-SDLC surfaces, delegates to the reviewed #487 AWS-D static contract validator, confirms the live-apply authorization gate is recorded, and rejects credential-like retained material without making AWS calls.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/727/validate-issue-727-readiness.sh",
      "."
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue-727-authorization-envelope",
    "proof_role": "Validate the operator-supplied #727 live mutation envelope names the exact approved region, Terraform root, workspace, state key, saved-plan digest, permitted resources, mutation deadline, cost ceiling, and rollback/destroy disposition before any apply; preserve this lane as the approval and exact-head review/publication gate.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/727/validate-issue-727-authorization-envelope.sh",
      "."
    ],
    "parallel_group": "approval",
    "defer_reason": "Runs after the operator supplies .adl/requests/727/operator-authorization.json; fail closed before any Terraform apply while this envelope is absent or incomplete."
  },
  {
    "lane": "issue-727-aws-readonly-readback",
    "proof_role": "Run the governed redacted AWS readback after the authorized saved-plan apply to prove CloudTrail, encrypted/versioned audit bucket retention, AWS Config delivery, IAM Access Analyzer, SNS/EventBridge findings route, and owner/destination tags without printing sensitive identifiers.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh",
      "--lane=aws-readonly"
    ],
    "parallel_group": "live",
    "defer_reason": "Runs only after explicit operator authorization and successful application of the exact saved plan in the approved business account."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/727/validate-issue-727-readiness.sh .`
- `bash .csdlc/prepared/issues/727/validate-issue-727-authorization-envelope.sh .`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-d/run-audit-security-readbacks.sh --lane=aws-readonly`

## Failure Semantics

Fail closed; do not apply a changed plan and report exact residual selectors without sensitive values.

## Handoff

Retain typed evidence before convergence.
