# Structured Planning Prompt

Template: 1.0.0

Issue: 745

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Apply the reviewed source packet in a bound worktree, validate the bounded documentation, review independently, and deliver through explicitly authorized typed v2.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bind the issue and refresh source evidence and candidate-number availability.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply the reviewed ADR drafts and eight-topic disposition table; record #744 and v2 authorization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Validate links, sections, statuses, source hashes and diff; obtain independent exact-head review.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Publish through typed v2 recovery, settle CI, finish and clean the registered worktree.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Proposed and Deferred statuses remain explicit.
- Exactly eight planned topics are accounted for.
- Existing accepted records remain unchanged.

## Risks

- Candidate numbers may collide with concurrent work.
- Source evidence may have advanced since draft preparation.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/745/design.md

Digest: a9956cdd37d14c49e07d235efbf070238223af515fa1921b6cbd75bf4c7ab6fd

## Diagram

.csdlc/prepared/issues/745/diagram.mmd

Digest: 6cb1b67911038c13dd9e2351f6695b0d1a61655c9467cbc4bcff2ec9a84eb428

## Stop Conditions

- Unresolvable source or numbering conflict
- Authority-changing scope required

## Handoff

Proceed only after doctor readiness.
