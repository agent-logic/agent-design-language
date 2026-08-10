# Structured Planning Prompt

Template: 1.0.0

Issue: 168

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Encode lifecycle and correction authorization as a pure exhaustive transition kernel generated from the frozen capability matrix, including exact recovery invalidation, stale CAS, ownership, terminal, and cleanup predicates.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Generate the complete state/command transition-and-correction table from the V3-01 capability matrix and reject unmapped pairs.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement pure transition inputs/results with topology-only ownership, generation/digest CAS, review staleness, publication, terminal, and cleanup predicates.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement review recover only from reviewed/published/merge_ready to implemented with complete atomic invalidation; reject merged/closed_out.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Prove every supported invalidation/recovery state has a concrete reachable typed correction or terminal disposition and no abstract operator sink.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run exhaustive table, mutation/property, stale-CAS, correction invalidation, dead-end reachability, and cleanup-eligibility tests with no adapters.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain generated-table parity and stop if ambient I/O or claim/lease/heartbeat authority reappears.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Release property-test fixtures and verify the pure kernel created no files, processes, network calls, claims, leases, or hidden ownership state.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-07 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/168/design.md

Digest: 6aa2f15b74447a733dea80e7f448a6e8ab044eff0f204c5b4a6490138e2b4c93

## Diagram

.csdlc/prepared/issues/168/diagram.mmd

Digest: 3de0edd13f8382a87b036d8ffc43e732f9110df5e21cd7da5a26cfc4371d8457

## Stop Conditions

- A transition needs ambient I/O, an unknown state falls through, or claims, leases, heartbeats, or protected-path ledgers reappear as authority.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
