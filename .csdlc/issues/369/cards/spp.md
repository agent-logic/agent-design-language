# Structured Planning Prompt

Template: 1.0.0

Issue: 369

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Review, bind, implement the narrow recovery request/store/CLI/tests, prove and publish, finish terminally, then recover #275.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Review exact recovery design and bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement request store CLI schema and focused regression.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Validate review publish CI finish and release #275.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Prior audit is immutable
- Recovery grants no approval/review/publication authority
- Topology and product state are unchanged

## Risks

- Recovery could become a generic approval bypass
- Repeated correction could mask provenance
- Later lifecycle authority could be weakened

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/369/design.md

Digest: 2ad9cf776f3eba1b88207f5bbf08aa2798284932a4ef443498aaec4de3017b77

## Diagram

.csdlc/prepared/issues/369/diagram.mmd

Digest: 1924ab6c91d8cd135a5ac7f4fc5cd0c00d3acf8d4ff3245fbd152ff4db3c0712

## Stop Conditions

- Any Runtime path edit
- Any generic audit/state editor
- Any approval or publication grant
- Any loss of prior false-review event

## Handoff

Proceed only after doctor readiness.
