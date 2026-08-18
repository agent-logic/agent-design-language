# Structured Planning Prompt

Template: 1.0.0

Issue: 358

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Review, approve, bind, implement the minimal sealed extension, validate, review, publish and finish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Approve exact sealed extension design.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind and implement with focused proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Review, publish, CI and finish.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Action is committed artifact truth
- Time components are durable authority truth
- Projection construction stays private

## Risks

- Ambiguous predecessor semantics
- Lossy time accessor
- Raw authority exposure

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/358/design.md

Digest: bf1a795e6581e7c079ed386859855d528e190192a013a7f19b614392d0132930

## Diagram

.csdlc/prepared/issues/358/diagram.mmd

Digest: ee0c235d6034c0957fc4963b74a87db7cadd46c5cbe628fea00053d9c47d93b2

## Stop Conditions

- Scope requires another product path
- Caller authority is introduced
- Validation/review/CI fails

## Handoff

Proceed only after doctor readiness.
