# Structured Planning Prompt

Template: 1.0.0

Issue: 75

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Introduce a small strum-backed linkage enum, validate exact body syntax, retain it end to end, and prove terminal closeout remains closing-only.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Add linkage_mode to request, intent, evidence, schema, and compatibility decoding.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement exact closing and part_of body validation for same and split repository authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Carry observed mode through reconciliation and make finish reject non-closing terminal authority.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused publication and finish regressions and exact-head review.",
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

- Closing linkage stays fail closed
- Part of never closes
- Remote identity remains exact
- Schema output matches Rust types

## Risks

- Backward compatibility drift for existing requests
- A part_of PR accidentally becoming closeout authority
- Qualified linkage parsing accepting lookalikes

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/75/design.md

Digest: cb1e2451b3b0eb50f7fa102784a7fdfffd23f499d32839a95cea6bb625d99546

## Diagram

.csdlc/prepared/issues/75/diagram.mmd

Digest: a053decb6bc47264966049f81f1d83cafc55e4ff5ba8e20bf79ebda58a91993e

## Stop Conditions

- Finish requires weakening terminal authority
- GitHub cannot expose enough evidence to distinguish modes
- Scope expands outside publication and finish contracts

## Handoff

Proceed only after doctor readiness.
