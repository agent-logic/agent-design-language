# Structured Planning Prompt

Template: 1.0.0

Issue: 306

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Approve the publication-tail contract, bind, implement the safe publish/finish interaction, prove retry and interruption behavior in focused tests, obtain fresh review, publish ready, and stop before merge.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Approve the exact publication-tail contract and Sprint 6 finish-blocking map.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the narrow safe metadata/order contract without active-lane fixture mutation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Run focused create/update/noop/interruption/finish-readiness tests and strict Clippy.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review, publish ready, and stop before merge.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- Exact-head review remains required before publication
- Exact-clean finish remains required for terminal truth
- Safe metadata classification is narrow and issue-specific
- Arbitrary untracked files are never treated as safe finish metadata
- Active issue worktrees are not mutated as fixtures

## Risks

- Treating publication metadata as safe too broadly could weaken finish
- Committing metadata at the wrong point could stale review or publish the wrong head
- Retry handling could duplicate intent or record truth
- Interruption windows could remain untested

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/306/design.md

Digest: 100d95e5c26c97b0978f64ffeef2dd8da79771ae2ca1d4028bd32ec3aa1b5271

## Diagram

.csdlc/prepared/issues/306/diagram.mmd

Digest: c49ebb317100ef859df21849967c3db0e4384c5e50f11b406d59c18bfaa42af4

## Stop Conditions

- Implementation would touch #258, #295, #298, #301, #5913 active worktrees, root staging, locks, or lifecycle
- A contract would weaken exact-head review or exact-clean finish
- Typed lifecycle reports collision
- Focused proof or exact-head review fails

## Handoff

Proceed only after doctor readiness.
