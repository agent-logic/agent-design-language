# Structured Planning Prompt

Template: 1.0.0

Issue: 96

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and bind issue 96, implement v3 topology reconciliation in the umbrella validator, prove generated Git histories, obtain exact-head review, and publish a green ready PR without merge.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind exact tooling/test scope and inspect current v2/v3 proof topology.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement S-to-E-to-H validation while preserving all terminal and denominator gates.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused generated-history positive and negative tests plus Ruby syntax checks.",
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
  },
  {
    "id": "S4",
    "action": "Resolve independent exact-head review and publish one green ready PR.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Exactly sixteen WP-04 children and exact owned-path denominator
- Dependency DAG and terminal merged envelopes remain mandatory
- Product bytes unchanged after S and evidence bytes unchanged after E
- #5878 integrated and native receipt bindings remain mandatory
- No self-referential evidence requirement

## Risks

- Accepting ambiguous proof mappings
- Treating ancestry alone as byte immutability
- Weakening #5878 integrated/native proof
- Fixture passing without exercising real Git object topology

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/96/design.md

Digest: b7c82d093043f28973898e6cf86a990284b51378e2e366f8e007b7a2d18488a2

## Diagram

.csdlc/prepared/issues/96/diagram.mmd

Digest: fc2a29c1ec2859aac4bbfc0c3397f5aa6d81ca7b785e93d8fe57e37602c6636d

## Stop Conditions

- Owned scope must widen beyond the validator and focused test
- Terminality or denominator checks would need weakening
- Generated fixture cannot reproduce current v3 proof topology
- Independent review has unresolved actionable findings

## Handoff

Proceed only after doctor readiness.
