# Structured Planning Prompt

Template: 1.0.0

Issue: 771

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Inspect current mappings and historical delta; author a fail-closed current mapping validator; freeze source SHA; independently review complete V3-F scope and replay full locked suite detached; retain receipts and map four rows; review and publish evidence PR.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Retain historical diff and define complete V3-F scope.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Author exact-current mapping and rejection validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "in_progress"
  },
  {
    "id": "S3",
    "action": "Freeze and independently review source; run full detached locked suite.",
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
    "id": "S4",
    "action": "Record four resolved V3-F rows and preserve CORP-A closure.",
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

- Historical receipts immutable; source review and suite use identical bytes; no CORP-A reopening.

## Risks

- Review and validation identity drift after changes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/771/design.md

Digest: c5de487bd08d61541324c06b3d20ea8f93bfcf33927f62edbe45fd8316c27fec

## Diagram

.csdlc/prepared/issues/771/diagram.mmd

Digest: f6dfe12a29db3619d547dcb7b2fb5fa10d87c88ac68e4089a8fc88f9064e6551

## Stop Conditions

- Stop on unowned issue state or identity ambiguity.

## Handoff

Proceed only after doctor readiness.
