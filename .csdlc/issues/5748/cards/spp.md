# Structured Planning Prompt

Template: 1.0.0

Issue: 5748

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Collect issue-local terminal receipts, repair special cases through typed authority, materialize exact receipt-backed projections in a dedicated worktree, and prove terminal and artifact consistency.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory remote disposition, local projection, worktree ownership, and retained receipt state for every target",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Create or repair terminal receipts only through issue-local typed routes, including the named special cases",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Materialize validated receipts in the #5748 authority worktree and prove doctor, receipt, artifact, and diff hygiene",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- retained receipt identity is the terminal authority
- tracked projections remain claim-free and closed_out after reconciliation
- remote PR or no-PR disposition is never inferred away
- dirty and foreign-owned work remains intact
- all generated lifecycle mutation is typed and atomic

## Risks

- receipt identity can conflict with stale tracked projection truth
- false closed_no_pr disposition may lack a supported typed correction route
- retained authored-artifact paths can be non-portable or inconsistent
- active issue claims can collide with aggregate projection ownership

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5748/design.md

Digest: ae02c8459bdeb5ddf0045ea4552a685db827ecbb99b189c96190a2abb2bb0c1e

## Diagram

.csdlc/prepared/issues/5748/diagram.mmd

Digest: ac4a40fa8a3f21d7363a3d6cb3cc9210e817c1494267552e1a165cfb46b67b9c

## Stop Conditions

- typed receipt identity conflict
- missing supported repair route
- dirty or foreign-owned worktree would need destructive cleanup
- remote disposition cannot be verified
- doctor or receipt equality fails after typed reconciliation

## Handoff

Proceed only after doctor readiness.
