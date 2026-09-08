# Structured Planning Prompt

Template: 1.0.0

Issue: 261

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reconcile exact #261/#342/#262 ownership, bootstrap and review the candidate identity packet, record operator and mailbox gates truthfully, implement deterministic packet validation, then obtain exact-head review and terminal handoff without publication actions outside this issue.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile live #51/#261/#262/#342 graph and exact collision-free path allocation.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Obtain explicit operator identity decision and privacy-safe mailbox receive proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Create the canonical identity rights collision mailbox and validation packet in the bound #261 worktree.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused validation obtain fresh exact-head review publish and finish only after external gates pass.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Pending external decisions remain pending and cannot be inferred from candidate repository text
- Production feed and episode-package paths remain untouched
- Artwork and metadata digests are recomputed from exact retained bytes
- No secret or private mailbox content enters public evidence
- Downstream consumers use one versioned canonical packet

## Risks

- Existing candidate title may conflict or lack operator approval
- Existing artwork may lack complete source or rights provenance
- Configured mailbox address may not prove receive readiness
- Historical #342 ownership may collide with production feed or artwork authority
- Metadata could drift across feed episode and launch surfaces

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/261/design.md

Digest: ea7882ecf45d307793e2a71f0524040d7257fdb6c9c59880d4dba766c3647638

## Diagram

.csdlc/prepared/issues/261/diagram.mmd

Digest: 8656619a890479888dcea9f75aa002e79656f06ce10b29e8429b61da01441ff3

## Stop Conditions

- Operator identity decision or mailbox proof is unavailable for terminal completion
- Artwork rights cannot be established
- Any required edit enters #342 episode/audio/package paths or #262 production feed/hosting paths
- Any credential verification code private mailbox content or unsupported approval claim would be retained
- Validation review CI or terminal truth fails

## Handoff

Proceed only after doctor readiness.
