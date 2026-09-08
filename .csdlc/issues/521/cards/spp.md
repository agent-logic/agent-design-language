# Structured Planning Prompt

Template: 1.0.0

Issue: 521

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Verify #520 and candidate identity, construct a context-complete external handoff, obtain independent findings-first review, reconcile findings without suppressing limitations, and validate the retained report.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify #520 reviewed merge and unchanged candidate identity.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build the reviewer handoff with complete scope, evidence index, non-claims, and limitations.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Obtain independent findings-first external review without candidate mutation.",
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
    "action": "Retain and validate the report, raw findings, limitation register, identity, and digests.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Candidate and #520 packet remain unchanged
- Reviewer is independent
- Limitations are never hidden
- External review does not approve release

## Risks

- Reviewer context may be incomplete
- Candidate drift may stale the report
- External findings may be accidentally summarized away

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/521/design.md

Digest: 751a9223732c0aec4ccf537bb26dd5cd3d94740bc96625535433f094ba4115db

## Diagram

.csdlc/prepared/issues/521/diagram.mmd

Digest: e9ff187b5a8bc1cbc4f491d00302428d8a6138b4d3a47d0cbbe838fb96004065

## Stop Conditions

- #520 is not reviewed and merged
- Candidate drift
- Reviewer conflict
- Incomplete handoff or unavailable evidence

## Handoff

Proceed only after doctor readiness.
