# Structured Planning Prompt

Template: 1.0.0

Issue: 173

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement a pure PVF planning domain that validates lane classification, proof role, resource/gate posture, command allowance, evidence destinations, dependency DAGs, and complete acceptance ownership before any execution begins.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Define versioned validation manifest, lane/result/resource/gate types, exhaustive classifications, and typed planning errors.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement acceptance-to-lane ownership and reject missing, duplicate, or hidden acceptance coverage.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement DAG validation for missing dependencies, cycles, duplicate ownership, and undeclared routing policy.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement command-allowance and evidence-destination validation from declared inputs only, with no ambient I/O.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Exercise representative serial/parallel/deferred/blocked/failed/skipped plans and every malformed-plan rejection.",
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
    "action": "Retain normalized plans and stop before execution if any classification or route depends on ambient state.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Discard invalid-plan scratch output, retain normalized fixtures, and verify planning created no process, network, clock, filesystem, or hidden routing side effect.",
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

- Issue V3-11A owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/173/design.md

Digest: 4db4fffda9a8f037c3919ac60005e62119d71cbfd88f23f54b20e22cde93d4c4

## Diagram

.csdlc/prepared/issues/173/diagram.mmd

Digest: 79e22d2db385982b29e32121809bd1c53fd5c5830f119e56281d9bcfce4234bc

## Stop Conditions

- Ordinary test code acquires routing policy, classification depends on ambient state, or a malformed plan can reach execution.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
