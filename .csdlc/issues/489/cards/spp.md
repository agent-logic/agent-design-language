# Structured Planning Prompt

Template: 1.0.0

Issue: 489

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #489 after #488 and #122 terminal gates, approve a design centered on private AWS Runtime platform modules, bind a FastWork worktree, implement module/runbook/evidence surfaces, run focused proof, obtain exact-head review, publish with closing linkage, and finish when green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the AWS-F Runtime platform-module design from current main containing #488 and #122.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind the #489 FastWork execution worktree and preserve dependency/collision truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement private Runtime platform Terraform, operator runbook, redacted proof packets, and issue-owned validation scripts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Validate, obtain fresh exact-head review, publish with closing linkage, and finish when green.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Runtime hosts have no direct public ingress
- Public edge ownership remains #122
- Adoption disposition remains #488
- Edge, network, build, and node states remain separated
- Disposable cleanup selectors are exact and do not target adopted durable resources
- Evidence is redacted and never includes credential material

## Risks

- A reusable platform module could accidentally expose Runtime hosts directly
- Module state could collapse public edge, private network, build, and node concerns into one unsafe authority
- Cleanup could target durable or adopted resources if selectors are broad
- Paid proof requires bounded operator authorization
- Cloud account state can drift between readback and review

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/489/design.md

Digest: d04ad7f1fb46894da562bcf6544887beede453de71e2a8ffdeb2d207f4c495b4

## Diagram

.csdlc/prepared/issues/489/diagram.mmd

Digest: 426a1f454fe55727b0062c748448dfdec9a80e289472c4c723eb241963ec328d

## Stop Conditions

- A proposed change opens direct public Runtime host ingress
- The work attempts to re-own #122 public edge or #488 adoption-register classifications
- Disposable cleanup selectors could touch durable/adopted resources
- Live AWS proof would require unavailable credentials, mutation beyond the issue proof, or production traffic
- A proposed change implements #496, #495, or production cutover scope

## Handoff

Proceed only after doctor readiness.
