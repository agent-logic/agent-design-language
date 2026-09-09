# Structured Planning Prompt

Template: 1.0.0

Issue: 519

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Finalize the exact reviewed #518 candidate after PR753 merged; prove review/source/merge linkage, hashes and redaction. Publish the independently reviewed #519 packet without GitHub merge, tag, release or issue closure.

## Plan

Revision 7

## Steps

[
  {
    "id": "S1",
    "action": "Prepare a provisional packet from the immutable #518 reviewed source; freeze final revision only after its reviewed merge.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Assemble artifact hashes and explicit publication and closing relationships.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Validate redaction and portable paths, including rejection of altered hashes and ambiguous linkage.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Refresh against merged #518, independently review and finalize the candidate packet without merge, tag, release or closure.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
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

- Candidate drift, stale review, ambiguous linkage or failed redaction blocks publication.
- No GitHub merge, tag, release or issue closure.

## Handoff

Proceed only after doctor readiness.
