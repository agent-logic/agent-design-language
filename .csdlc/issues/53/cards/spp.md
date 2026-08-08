# Structured Planning Prompt

Template: 1.0.0

Issue: 53

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the two-revision contract, implement strict Git identity and ancestry checks, add an isolated A/B/C regression with tamper cases, run focused proof, resolve exact-head review, and publish.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Review and bind the two-revision proof schema, ancestry, evidence-only path, compatibility, and failure contracts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement strict v3 revision resolution, ancestry, and evidence-only diff validation while preserving v2 behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add and run focused A/B/C acceptance plus source, receipt, log, artifact, ancestry, and path-drift rejection tests.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Resolve exact-head independent review and publish the qualified issue-closing PR.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "in_progress"
  }
]

## Invariants

- Every accepted revision resolves to an exact Git commit
- Source ancestry and evidence ancestry are machine verified
- The source-to-evidence diff contains no product or unrelated path
- Existing command, log, artifact, negative-case, native receipt, and runner checks remain mandatory
- Retained v2 evidence is immutable

## Risks

- An overly broad evidence path rule could hide product drift
- Merge ancestry could be mistaken for direct evidence ancestry
- Compatibility logic could silently reinterpret v2 receipts
- Temporary Git fixture construction could become brittle

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/53/design.md

Digest: 41ea7f573d657eb50129ac512fefcaece259ba1d2cedb0b37d6651c8373a2746

## Diagram

.csdlc/prepared/issues/53/diagram.mmd

Digest: f94a69f5b37cac71634f7e74d31c4d66d09efa737e8f580ee71faf9c7b78a6a4

## Stop Conditions

- The design would permit product changes between source and evidence revisions
- The validator cannot resolve either revision exactly
- A retained v2 receipt would require mutation
- Scope must widen beyond the shared WP-04 receipt contract and issue-local proof

## Handoff

Proceed only after doctor readiness.
