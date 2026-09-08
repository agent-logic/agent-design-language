# Structured Planning Prompt

Template: 1.0.0

Issue: 490

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #490 from current main, run only read-only gcloud discovery, normalize a decision register under owned docs/evidence paths, validate redaction and no-mutation posture, obtain fresh review, and publish one closing PR.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind issue #490 from current main in a FastWork worktree.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Verify current gcloud identity, project, billing, and organization decision inputs with read-only commands.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Write the GCP decision register with ownership, region, cost ceiling, and quota/capacity boundaries.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Validate redaction, no-mutation command posture, diff hygiene, and lifecycle truth.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Obtain fresh exact-head review, fix findings, publish, shepherd green checks, finish, and leave cleanup async.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- No GCP mutation commands are run.
- No credential material is captured.
- Quota is never claimed as available runtime capacity.
- Cost ceiling is explicit before downstream bootstrap.
- Downstream GCP-B scope remains separate.

## Risks

- The active gcloud context could point at the wrong account.
- Billing visibility may be insufficient for a final decision.
- Readbacks could expose sensitive token fields if command selection widens.
- Quota could be mistaken for approved capacity.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/490/design.md

Digest: 6f09347efe8e393f52fae5e358ab0e0b38bfa06770304c73aef78396cf069c94

## Diagram

.csdlc/prepared/issues/490/diagram.mmd

Digest: c47f872b0b7cb0ce093fe929d1897a93703b3a52e7a1bf21da843d4ea9293927

## Stop Conditions

- Identity or billing is ambiguous.
- Credit expiry or cost basis is unknown.
- A decision would require mutation.
- Evidence would require credential disclosure.
- Fresh review finds unresolved actionable issues.

## Handoff

Proceed only after doctor readiness.
