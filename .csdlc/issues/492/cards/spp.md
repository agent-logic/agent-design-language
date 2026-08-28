# Structured Planning Prompt

Template: 1.0.0

Issue: 492

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #492 after #491 terminal ancestry, approve a design centered on the GCP organization/billing denominator, bind a FastWork worktree, implement Terraform/docs/evidence/readback surfaces, run focused proof, obtain exact-head review, publish with closing linkage, and finish when green.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the GCP-C organization/billing baseline design from current main containing #491.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind the #492 FastWork execution worktree and preserve dependency/collision truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement organization/billing Terraform, operator runbook, redacted readback/evidence packets, and issue-owned validation scripts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate, obtain fresh exact-head review, publish with closing linkage, and finish when green.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- New managed projects have corporate group ownership, not individual-only ownership
- Every admitted project has cost-attribution labels and billing observability
- Organization policies remain scoped to the reviewed denominator
- Existing POC resources are unchanged unless a reviewed exception is explicit
- Terraform bootstrap ownership remains #491 and is consumed read-only
- Evidence is redacted and never includes credential material

## Risks

- A broad organization policy could affect unrelated POC resources
- Individual-only ownership can leave the baseline non-corporate
- Billing export or budget absence can hide spend
- Live GCP state can drift between readback and review
- Credential or account details can leak if evidence is not redacted

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/492/design.md

Digest: 0ae400153a8c50d2cda0b76dc94d9a94081574448c0013bb1b423410b1168673

## Diagram

.csdlc/prepared/issues/492/diagram.mmd

Digest: 02186bba0cc8f159d0c225a176c563e45fe26d7b22b103d57d870d6f675626fd

## Stop Conditions

- A broad policy could affect POC resources
- Individual-only ownership remains
- Cost attribution is absent
- Live GCP readback would require unavailable credentials or credential exposure
- A proposed change would implement #493, #494, #495, or production activation scope

## Handoff

Proceed only after doctor readiness.
