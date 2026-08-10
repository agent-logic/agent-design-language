# Structured Planning Prompt

Template: 1.0.0

Issue: 181

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the distributed qualification contract before any live node starts: enumerate topology and scenario denominators, define producer receipt schemas, validate uniqueness and cleanup guarantees, then independently review the exact contract.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inventory both qualification windows and freeze exactly three voters, three governed agents, one non-voting Shepherd, and one quorum-leased Observatory with distinct identities, credentials, ports, state roots, storage, and failure domains.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Define every election, quorum-loss, stale-lease, restart, snapshot, partition, healing, replay, and cleanup scenario with bounded setup, action, expected state transition, timeout, receipt fields, and abort behavior.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Specify machine-readable topology, scenario, producer-receipt, resource, timing, cleanup, and claim schemas; explicitly distinguish production processes from harness orchestration and forbid in-process substitutes and hard-coded counts.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement the issue-owned contract validator so it recomputes actor denominators, uniqueness, scenario completeness, production-path ownership, receipt digests, and cleanup obligations from exact artifacts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run the contract and diff-hygiene lanes; retain exact revision and digest-bound receipts, and stop on any missing proof owner, timeout, cleanup action, or independently materialized state root.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Obtain independent exact-head review of the frozen contract and validator, remediate findings without provisioning nodes, and publish only the reviewed planning artifact.",
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

- Issue DRT-01 owns only its declared repository paths and named external operation/evidence boundary.
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
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/181/design.md

Digest: c5bc763aebf54321d7a1e409db548449a60b0a9fe746ae319306be76ce47fb2b

## Diagram

.csdlc/prepared/issues/181/diagram.mmd

Digest: 63d6b245dc2b6bf4718e6099a17c1ca2541be9a6bb98b7f1436a15ece95d2b12

## Stop Conditions

- A production path lacks a named proof owner
- A scenario has no bounded timeout or cleanup
- Topology can collapse to one process or shared state
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
