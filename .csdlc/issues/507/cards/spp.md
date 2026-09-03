# Structured Planning Prompt

Template: 1.0.0

Issue: 507

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #507 from current main after #506 terminal and #345 closed predecessor observation, approve a design centered on six-resident UTS continuity/reclamation qualification, bind a FastWork execution worktree, implement focused deterministic DRT-B proof/evidence/validation, obtain exact-head review, publish with closing linkage, and finish when green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the DRT-B design from current main containing #506.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind the #507 FastWork execution worktree and preserve dependency/collision truth.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement deterministic six-resident UTS, dehydrate/restore, continuity/reclamation, resource-envelope, and cleanup evidence surfaces.",
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
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Exactly six distinct residents participate in the qualification denominator
- Dehydration and restoration preserve exact resident population and workload receipts
- Qualification output records local deterministic proof separately from any paid/cloud/GPU proof
- #508 final qualification and #509 GCP portability remain separate issues
- No credential material or sensitive cloud account data enters retained evidence

## Risks

- The proof could accidentally rely on labels or fixture names instead of actual resident identity and receipt state
- Cloud/GPU proof expectations could silently widen beyond available authorization
- DRT-B changes could collide with #508 DRT-C failure/observatory paths or #509 GCP portability paths
- The #345 closed predecessor lacks a local derived-terminal cache; the prebind validator must accept a future local cache or prove live GitHub state is CLOSED and fail closed otherwise

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/507/design.md

Digest: 98ee0b91f6b5ca60446be945be671806c00fdf50217f8554de3e1e39f869b40f

## Diagram

.csdlc/prepared/issues/507/diagram.mmd

Digest: 2532bce733a5afdef1330fe2feb308e870515883f47b7ce6db31f5859f34de8a

## Stop Conditions

- A proposed change executes #508 or #509 scope
- Six resident identities cannot be proven distinct from actual contract state
- Dehydrate/restore does not preserve exact population and workload receipts
- A required paid/GPU proof lacks explicit authorization or bounded cleanup controls
- Evidence would expose credentials or sensitive account details

## Handoff

Proceed only after doctor readiness.
