# Structured Planning Prompt

Template: 1.0.0

Issue: 520

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #519 merges, freeze its exact candidate, build complete path/issue/acceptance denominators, run findings-first specialist lanes, synthesize all raw findings, validate the packet, and obtain exact-head review.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify #519 reviewed merge truth and freeze full base/candidate SHAs.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build complete changed-path, issue/PR, acceptance-surface, canonical-doc, demo, and evidence inventories without sampling.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run bounded findings-first specialist lanes covering every inventoried row.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Synthesize raw findings, acceptance coverage, disagreements, blockers, and #522 dispositions into one canonical register.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Validate counts, schemas, digests, redaction, portability, and exact-head identity; obtain independent packet review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- The candidate stays immutable during review
- Every denominator row is dispositioned
- Green CI never replaces semantic review
- Findings are not fixed inside #520

## Risks

- A truncated inventory could hide half-work
- Late candidate drift could stale reviews
- Lifecycle or docs claims could overstate implementation
- Parallel lane results could overlap or omit rows

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/520/design.md

Digest: a92ee174e71d477315c1979f5f78c713ce37679b869f04317b9ab313740c7555

## Diagram

.csdlc/prepared/issues/520/diagram.mmd

Digest: d8ccec569a47ba807da9754860256420256dbcc147c2a3df123752c668f34fdb

## Stop Conditions

- #519 is not a reviewed green merge
- Candidate drift
- Incomplete or empty denominator
- Unassigned or stale mandatory lane
- Packet validation failure

## Handoff

Proceed only after doctor readiness.
