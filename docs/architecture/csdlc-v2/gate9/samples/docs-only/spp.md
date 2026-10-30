# Structured Planning Prompt

Template: 1.0.0

Issue: 9001

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Run the bounded docs-only sample and retain exact evidence.

## Plan

Revision 1

## Steps

[
  {
    "id": "sample-proof",
    "action": "Execute docs-only proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  }
]

## Invariants

- v1 remains the default
- review precedes publication

## Risks

- sample evidence could overclaim production behavior

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

docs-only/design.md

Digest: 351d2d683cfea6ea9fc989fccaf64be2d86dbfd9c4be90aa8c819c0237a61417

## Diagram

docs-only/diagram.mmd

Digest: 663561e20b74b07eb6b84bc8f037b78628507d5e8d00d5482ae0194926aab1eb

## Stop Conditions

- unexplained critical parity difference

## Handoff

Proceed only after doctor readiness.
