# Structured Planning Prompt

Template: 1.0.0

Issue: 486

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #486, approve design, bind a FastWork worktree, implement the dedicated AWS Terraform backend/deployment-role bootstrap, run focused Terraform and readback validation, obtain exact-head review, publish, and finish when green.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the AWS-C Terraform bootstrap design.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the dedicated backend, lock, provider-pin, and deployment-role bootstrap surfaces.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused Terraform/static/readback validation and record retained evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review and publish with closing linkage.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Existing website/DDNS/public-edge/workload states remain separately owned
- Terraform backend resources are recoverable and have explicit names
- No retained evidence contains credentials or token material
- Quota or account access is not treated as workload deployment approval

## Risks

- Terraform backend bootstrap can accidentally overlap with existing state resources
- AWS plan/apply drift could invalidate saved plan review
- Deployment role policy can become too broad if future-scope permissions are pulled into this issue

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/486/design.md

Digest: 5431aea2b69bbca8e990b877455ede18628b6726d7fa983e9da500cc98edaa3e

## Diagram

.csdlc/prepared/issues/486/diagram.mmd

Digest: ed3c82681f657aeaa419c35dd4fd77259d665b01061b2acf586e0f441995a72c

## Stop Conditions

- An existing backend owner is unknown
- State recovery fails
- A reviewed plan differs at apply
- AWS account identity is not the approved Agent Logic business account
- Credential material would be retained

## Handoff

Proceed only after doctor readiness.
