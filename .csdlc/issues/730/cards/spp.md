# Structured Planning Prompt

Template: 1.0.0

Issue: 730

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Replace static-key defaults, add fail-closed impersonation/plan/recovery/residue tooling, run no-mutation validation, obtain operator authorization for the exact plan, then execute and retain live readback.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Implement impersonation-only configuration and validators.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run focused static and negative validation and review the saved plan.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Under exact authorization, apply and prove backend controls, recovery, and cleanup.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No static key execution.
- No mutation beyond the exact reviewed denominator.
- Adopted state is never automatically deleted.
- Secrets never enter evidence.

## Risks

- Credential fallback could silently retain static-key dependence.
- A plan could widen IAM or resource scope.
- Cleanup could delete adopted state.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

docs/milestones/v0.92.1/evidence/cloud/gcp-b1/design.md

Digest: fe0f5f73d33d207c417e1c17f0ece19823e838b6702adbf52c5c95d46fb1b4bc

## Diagram

docs/milestones/v0.92.1/evidence/cloud/gcp-b1/diagram.mmd

Digest: 9f671babdc2733ea1e03df0e59a3e1acd13a504baae17f72f3ae990807d03d08

## Stop Conditions

- Impersonation is unavailable.
- Plan includes any unapproved resource.
- Remote recovery fails.
- Residue or credential material is detected.
- Budget or authorization expires.

## Handoff

Proceed only after doctor readiness.
