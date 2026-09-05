# Structured Planning Prompt

Template: 1.0.0

Issue: 679

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind a dedicated issue worktree, inspect #512 static bundle inputs, add or normalize AWS static-hosting plan artifacts, add redacted no-mutation/readback validators, prove local deployability and redaction, obtain exact-head review, then publish without merging until authorized.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bind #679 to a dedicated FastWork issue worktree and preserve root main as inspection-only.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Inspect #512 static bundle inputs and define the S3/CloudFront deployment contract without editing #512 product ownership.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add Terraform or deployment-plan artifacts for S3 CloudFront ACM Route53 CSP headers logging invalidation and rollback.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Add redacted AWS profile-gated readback scripts and local validators for redaction no-secret and no-live-mutation defaults.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused local validation, exact-head review, typed publication, and leave merge/live AWS apply for explicit authority.",
    "acceptance_ids": [
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "in_progress"
  }
]

## Invariants

- static Observatory assets remain credential-free
- Runtime endpoints are configured as non-secret per-polis inputs
- CloudFront is the public static edge and S3 is not public
- live AWS mutation is never implied by readiness
- all retained cloud evidence is redacted

## Risks

- accidentally absorbing #512 UI behavior
- mistaking dry-run or local proof for live deployed proof
- leaking account identifiers or credentials in readback logs
- using personal/default AWS account state
- weakening CSP/CORS/WSS compatibility to make local tests pass

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/679/design.md

Digest: a48d95127e20d9603a52e22192f6965feaf89b61b6cde6d0bcedabb22e9c2432

## Diagram

.csdlc/prepared/issues/679/diagram.mmd

Digest: 686d6edb4bd7ee75ebb306d057030126ee366a7c6b69d759ef64d94b02d85d0c

## Stop Conditions

- #512 bundle contract is unavailable or actively changing in conflicting ways
- AWS account/profile authority is ambiguous
- live AWS mutation is required but not explicitly authorized
- Terraform/backend state ownership is ambiguous
- secret or credential material would need to be copied into the repo

## Handoff

Proceed only after doctor readiness.
