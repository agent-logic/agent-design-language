# Structured Planning Prompt

Template: 1.0.0

Issue: 524

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Compare the two successor closeout surfaces, repair their shared denominator and gates, validate exact order and authority, then publish one reviewed documentation PR.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Bind the closeout denominator to the reviewed #523 planning revision.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Reconcile exact tail order, operator gates, and asynchronous bookkeeping boundary across release plan and checklist.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused closeout-contract validation, obtain exact-head independent review, and publish one closing PR.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Tail order is explicit
- Release mutation requires operator authorization
- Merge gates execution; closeout bookkeeping does not
- Planning never claims execution

## Risks

- Release plan and checklist may drift
- Bookkeeping may accidentally serialize execution
- Operator gates may be implicit

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/524/design.md

Digest: 743cc597d0cb3d0a65bcb8f591a4ed05fe27bd3c7d8758e982967987eb8af18c

## Diagram

.csdlc/prepared/issues/524/diagram.mmd

Digest: ca942303fa206e075a170c0ab6ac84262740991f38f08a372865a0d23a3da201

## Stop Conditions

- #523 lacks a reviewed merge
- Tail authority is implicit
- The two documents disagree on denominator
- The task would perform a live release action

## Handoff

Proceed only after doctor readiness.
