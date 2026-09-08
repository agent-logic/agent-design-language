# Structured Planning Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #518 merges, freeze its exact reviewed candidate, assemble publication linkage and redacted artifacts, and emit one non-mutating publication-candidate packet.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify #518 reviewed-merge authority and freeze its exact reviewed candidate revision.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Assemble the exact artifact index and validate publication and closing relationships.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run redaction and path-portability checks over the complete packet.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Emit the single publication-candidate packet without performing merge, tag, release, or closure mutations.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- The packet revision equals the reviewed candidate
- Closing relationships are explicit and unambiguous
- Sensitive and machine-local data are absent
- Preparation performs no release mutation

## Risks

- Post-review drift could invalidate the packet
- Ambiguous linkage could close the wrong issue
- Evidence could expose secrets or local paths
- Packet preparation could be mistaken for release authority

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/519/design.md

Digest: d57e1f849698fa6840961dc6b30e2e11b05073a49ca61cef6d65312ebc861070

## Diagram

.csdlc/prepared/issues/519/diagram.mmd

Digest: d48979ece9151d49508c102c53acb6f0612bad212572729825ccd9d92fa8a175

## Stop Conditions

- #518 lacks a reviewed merge
- The candidate changes after review
- Publication linkage is ambiguous
- Redaction fails
- Any release mutation is requested

## Handoff

Proceed only after doctor readiness.
