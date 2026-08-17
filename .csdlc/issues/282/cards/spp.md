# Structured Planning Prompt

Template: 1.0.0

Issue: 282

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Validate terminal dependencies, assemble the exact-revision qualification packet and runbook, validate packet structure, obtain fresh independent review, then publish and finish #282 if CI and lifecycle gates pass.

## Plan

Revision 4

## Steps

[
  {
    "id": "S1",
    "action": "Validate #279/#280/#281 terminal caches and #282 preparation packet",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Assemble exact-revision qualification artifact and runbook",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate the issue-owned qualification packet",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain independent product, architecture, and security review outcomes",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Publish, shepherd CI, and finish terminal truth",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- #282 does not change Runtime, browser UI, API, cloud, Unity, or provider behavior
- All proof references are exact-revision and terminal-cache backed
- Residual risks and non-claims are retained rather than hidden

## Risks

- Stale terminal evidence if origin/main advances before publication
- Overclaiming release readiness beyond the exact local/browser proof graph
- Confusing #282 qualification assembly with child proof ownership

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/282/design.md

Digest: 3079c8247b039c558f201cfbe2ca61745ecdf32a60771ac53e17a28463b23625

## Diagram

.csdlc/prepared/issues/282/diagram.mmd

Digest: f97c5419f147209a7d82bad66197b850feb3e881db79a4434b4e2ffaa1b3f3c3

## Stop Conditions

- Any terminal dependency cache fails canonical validation
- Any fresh review returns unresolved actionable findings
- The packet requires credentials, cloud deployment, or implementation changes outside #282 scope
- Publication or finish observes stale head/base/review truth

## Handoff

Proceed only after doctor readiness.
