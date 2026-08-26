# Structured Planning Prompt

Template: 1.0.0

Issue: 319

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Replace the obsolete all-records-closed ceremony gate with an exact disposition-evidence manifest, prove the script on the clean candidate, retain reviewed candidate evidence and a post-merge receipt template, and publish #319 without release mutation.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind exact predecessor dispositions and candidate authority in a machine-readable manifest.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Repair and prove the ceremony gate and negative paths on the clean exact candidate.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Finalize reviewed candidate evidence and the post-merge final-receipt template.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain typed independent exact-head review and publish the closing PR without tag/release mutation.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Merge authority gates downstream work; typed finish and cleanup do not
- Check-only ceremony causes no network mutation
- Every release identity and claim is exact-candidate bound
- Published release history is never deleted by rollback

## Risks

- Stale release identity or non-ancestral predecessor
- Legacy same-version records falsely blocking ceremony
- Partial tag/release mutation or unsupported milestone claims

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/319/design.md

Digest: d99e315746b165b7246654a8e37dd1ab4fc1de7f44293e75cf42fc45fdea62e4

## Diagram

.csdlc/prepared/issues/319/diagram.mmd

Digest: 1a43fa130f7761efc696f74071007e26f7dcf3e9948ba76a0440a3949d9c3f68

## Stop Conditions

- #318 merge or required check/review identity cannot be proven
- Main or candidate is dirty, divergent, or not exact
- Tag/release identity conflicts
- Ceremony validation or exact-head review fails

## Handoff

Proceed only after doctor readiness.
