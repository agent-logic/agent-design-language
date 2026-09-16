# Structured Planning Prompt

Template: 1.0.0

Issue: 511

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Define the Observatory experience contract, enumerate state and accessibility behavior, verify Runtime field provenance, and retain review-ready evidence.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory the existing Observatory UI and Runtime projection fields that may be consumed.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Write the per-view information contract and state matrix.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Write keyboard and screen-reader flows for every designed state.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run the focused contract, accessibility-plan, Runtime-field-census, v3 local canary, and review proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Design does not invent Runtime fields
- Production implementation remains in #512
- V3 remains non-authoritative before #505
- Accessibility states are first-class design truth

## Risks

- Existing Runtime projection docs may be incomplete
- Design may accidentally depend on future #512 implementation
- Accessibility denominator may be underspecified

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/511/design.md

Digest: 5c2405bb74605f5fb10fc4bfd9e8eda98927b120b2af1f00dc50a45970411748

## Diagram

.csdlc/prepared/issues/511/diagram.mmd

Digest: 1f3a16fb9e9437fb5cd5dd3646356d098aa225717895126b0fc610c1856b9ec0

## Stop Conditions

- A design requires unavailable Runtime authority
- Accessibility denominator is incomplete
- A mock or invented Runtime field is needed
- V3 local output is treated as lifecycle authority

## Handoff

Proceed only after doctor readiness.
