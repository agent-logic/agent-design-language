# Structured Planning Prompt

Template: 1.0.0

Issue: 9003

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Run the bounded hostile-recovery sample and retain exact evidence.

## Plan

Revision 1

## Steps

[
  {
    "id": "sample-proof",
    "action": "Execute hostile-recovery proof",
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

hostile-recovery/design.md

Digest: a8a7f5f02d19c62dd175ee4123fefc097051802afcc3be7172bee543324d29e8

## Diagram

hostile-recovery/diagram.mmd

Digest: 561e78ff2790aadcd0abebb0482127946313cc1597bbf3b004a55118b9f4ab75

## Stop Conditions

- unexplained critical parity difference

## Handoff

Proceed only after doctor readiness.
