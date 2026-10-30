# Structured Planning Prompt

Template: 1.0.0

Issue: 9002

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Run the bounded small-rust sample and retain exact evidence.

## Plan

Revision 1

## Steps

[
  {
    "id": "sample-proof",
    "action": "Execute small-rust proof",
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

small-rust/design.md

Digest: b0a4905fd45f9d0273c29467b0b714f75353d26505de853ec968ce9030be2638

## Diagram

small-rust/diagram.mmd

Digest: d8903a14d42d24fe83975b27d1ddd14fcffa48f6cfa227125c61ac361545f0e1

## Stop Conditions

- unexplained critical parity difference

## Handoff

Proceed only after doctor readiness.
