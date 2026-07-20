# Structured Planning Prompt

Template: 1.0.0

Issue: 5600

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Add explicit card-owned collection replacement operations, enforce Bound-phase compare-and-swap and atomic cross-card validation, prove all negative cases, and exercise a complete #5337 typed conversion before exact-revision review and publication.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Define typed card-owned replacement operations for all remaining SIP, STP, SPP, and SRP planning collections",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement Bound-phase authorization, atomic regeneration, audit semantics, and cross-card acceptance validation",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add complete positive, negative, compatibility, and #5337 conversion fixtures",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused and all-target proof, strict lint, exact-revision review, typed publication, and check monitoring",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- main remains untouched
- rendered cards and values files are never directly edited
- failed operations change no card, generation, digest, or audit entry
- acceptance coverage is complete before commit
- no acceptance criterion is deferred

## Risks

- broad replacement APIs could weaken card ownership
- cross-card changes could commit partially
- acceptance identifiers could drift across STP, SPP, and VPP
- serialized enum changes could regress existing records

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5600/design.md

Digest: fb97af4d5a4b4a7c2a0b4d7760e2825f0aababe6cbf57e2758a323c8a172d3c6

## Diagram

.csdlc/prepared/issues/5600/diagram.mmd

Digest: 8495aaea349825fdd80ce41a4a46d8f3e0f911f4a4e0a249b295e8c3aa8266be

## Stop Conditions

- typed lifecycle authority is unavailable
- another live claim owns issue #5600 or an exact protected path
- the requested behavior requires changes outside csdlc-v2 and issue-local lifecycle paths

## Handoff

Proceed only after doctor readiness.
