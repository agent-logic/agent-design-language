# Structured Planning Prompt

Template: 1.0.0

Issue: 101

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind #101 on FastWork, align root and boundary ownership policy, add one focused policy/fixture regression test, validate narrowly, independently review the exact head, and publish without merge.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bind the issue and inspect the existing policy and test conventions.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Update policy and add the focused drift and connector-403 regression proof.",
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
    "action": "Run focused validation and verify typed issue access through the default resolver.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain exact-head independent review, resolve findings, and publish one ready PR without merging.",
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
  }
]

## Invariants

- Repo-native Rust owner authority
- Fail-closed route selection
- No token contents in output or evidence
- No connector/raw-gh fallback
- No issue #100 mutation

## Risks

- Policy prose may drift between root and boundary document
- A fixture could accidentally encode real credentials
- A broad test could trigger unrelated validation cost

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/101/design.md

Digest: 2456d1aa2214dffe6cd2e5f36603dfe376305a3f79d615d3a5dcb7bac9c40a70

## Diagram

.csdlc/prepared/issues/101/diagram.mmd

Digest: 01e4024ebc092371853e7dd4dca1602d640aac807b4f3ea333540e9fe97f18de

## Stop Conditions

- The fix requires changing token resolver behavior
- The fix requires connector authorization
- The issue cannot be bound to FastWork
- Exact-head review or focused validation fails

## Handoff

Proceed only after doctor readiness.
