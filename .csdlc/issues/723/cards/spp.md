# Structured Planning Prompt

Template: 1.0.0

Issue: 723

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Port the bounded typed install authority checks, repair typed child-output normalization and fixture drift, then run the exact all-target suite.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Port bounded install authority checks",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Repair shadow typed-output normalization and deterministic fixture",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused and all-target v3 tests",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  }
]

## Invariants

- blocked routes remain nonzero
- mismatches remain observable
- typed authority binds exact content

## Risks

- accepting arbitrary stderr
- fixture dependence on mutable lifecycle state

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

docs/milestones/v0.92.1/evidence/csdlc-v3/issue-723/design.md

Digest: 6bf38e51258c2269452482ecef6c4f3cba0d841af1e5cd96586443cbf39bb479

## Diagram

docs/milestones/v0.92.1/evidence/csdlc-v3/issue-723/diagram.mmd

Digest: 797ff9ca9e4cdc24d01a8d28061c2a5271e408de5e7498fd3cd6dd72c93674ee

## Stop Conditions

- fix requires weakening authority
- tests still depend on mutable remote state

## Handoff

Proceed only after doctor readiness.
