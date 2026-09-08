# Structured Planning Prompt

Template: 1.0.0

Issue: 517

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #516 merges, freeze the exact candidate, enumerate every required proving lane, fail closed on non-proving evidence, and emit one quality-gate decision.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify #516 reviewed-merge authority and freeze the exact candidate revision.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build the complete required proving-lane denominator and resolve retained evidence for the exact candidate.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Classify every lane and exception, rejecting skipped, absent, zero-test, stale, or non-proving results.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Emit and validate the single exact-candidate quality-gate decision record.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- The candidate revision remains exact
- No missing or non-proving lane is treated as passing
- Gate evaluation does not implement remediation
- One issue produces one decision

## Risks

- An incomplete denominator could create a false green
- Zero-test output could be mistaken for proof
- Candidate drift could stale the decision
- An exception could lack an owner

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/517/design.md

Digest: 97b5eefa0f8d5d6a1fe2e1264b1c74e75074c3975c4c073835a31727edd91759

## Diagram

.csdlc/prepared/issues/517/diagram.mmd

Digest: ad0c16ae7d0eb9be819781b7c1906954e82f8ef84338a2b4fdf709c86d26678b

## Stop Conditions

- #516 lacks a reviewed merge
- Any required lane fails or is absent
- The candidate changes during evaluation
- An exception lacks an owner

## Handoff

Proceed only after doctor readiness.
