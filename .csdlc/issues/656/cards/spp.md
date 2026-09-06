# Structured Planning Prompt

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Define one generation contract; implement complete staging, verification, activation, and rollback; add pre-mutation CSM checks; prove with focused tests and review.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Define generation layout, receipt, compatibility, and verification.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement staging, atomic activation, predecessor retention, and rollback.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add CSM start and reload preflight before service mutation.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused proof and exact-head review.",
    "acceptance_ids": [
      "AC-9",
      "AC-10"
    ],
    "status": "completed"
  }
]

## Invariants

- One matched three-artifact generation
- One atomic current reference
- Preflight before mutation
- Previous verified generation retained
- Live Runtime untouched

## Risks

- Receipt-to-artifact mismatch
- Partial generation activation
- launchd/kernel generation disagreement
- Late preflight
- Unverified rollback

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/656/design.md

Digest: ae1e31e488fedefe6d1b9f37692cd4ee37e6dddc60cc35191a042a2c3dfbeb52

## Diagram

.csdlc/prepared/issues/656/diagram.mmd

Digest: b73d56623c72f4e63d53bbc165da4751483020f11f41a227f810b7f4d74e4b62

## Stop Conditions

- Live restart becomes necessary
- Integrity cannot be proven before mutation
- Atomic reference switch is unavailable
- Scope widens into a later slice
- Review has unresolved findings

## Handoff

Proceed only after doctor readiness.
