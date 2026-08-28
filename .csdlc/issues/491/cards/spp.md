# Structured Planning Prompt

Template: 1.0.0

Issue: 491

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #491, obtain design approval, bind a FastWork worktree after #490 terminal ancestry, implement the GCP Terraform backend/deployment-identity bootstrap, run focused static and safe GCP readback validation, obtain exact-head review, publish, and finish when green.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the GCP-B Terraform bootstrap design.",
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
    "id": "S2",
    "action": "Bind the #491 FastWork execution worktree after #490 terminal ancestry is verified.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement Terraform backend, deployment identity, runbook, saved-plan, cleanup, and proof surfaces.",
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
    "action": "Validate, obtain fresh exact-head review, publish with closing linkage, and finish when green.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Terraform bootstrap targets only cs-host-377d41e71a824f92802120
- Steady-state execution prefers impersonation over static keys
- The local static key stays outside the repository under /Users/daniel/keys
- Retained evidence never includes credentials, token material, or service-account private-key JSON
- Provider pins and saved-plan review are explicit before apply
- Local bootstrap state is removed or quarantined recoverably after backend migration

## Risks

- Static key material can leak if commands print or copy the JSON
- Terraform can target the wrong project if provider identity is implicit
- Remote state can be public, unversioned, or unrecoverable if bucket controls are incomplete
- Local bootstrap state can retain resource identifiers or secrets after migration
- Unpinned providers or modules can drift between review and apply

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/491/design.recovered.md

Digest: d8612146dc6fb5d24236862dae8fb16ba26836230b16e6d99324f841969a0452

## Diagram

.csdlc/prepared/issues/491/diagram.recovered.mmd

Digest: 51310babdb0c0104d55cf637a987d8047da8bdffa06fd9480880ee6d97b3df1f

## Stop Conditions

- State recovery fails
- Credential material would enter retained evidence
- Reviewed plan identity drifts
- Provider pins are absent
- Local bootstrap state cannot be removed recoverably
- GCP mutation would target a non-company or non-host project

## Handoff

Proceed only after doctor readiness.
