# Structured Planning Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Extend the existing Observatory into the first truthful Layer 8 chat vertical slice: live roster and status, selected-agent signed messaging through canonical Runtime ingress, bounded public-safe response or refusal, reconnect continuity, and retained live-browser proof.

## Plan

Revision 11

## Steps

[
  {
    "id": "S1",
    "action": "Implement the live Runtime roster and WSS client state machine with fresh, stale, unavailable, and bounded reconnect behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement selected-agent Layer 8 messaging through canonical signed Runtime ingress with visible-recipient validation and bounded public-safe response or refusal.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement the familiar Observatory chat surface without persisting or exposing signing material and retain truthful failure states.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused Runtime, OpenAPI, browser-shell, live-browser, reconnect, redaction, refusal, and diff-hygiene proof.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "in_progress"
  },
  {
    "id": "S5",
    "action": "Review exact-head behavior and preserve the strict design-approved path boundary for publication.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- No fixture is labeled live
- Read access never implies write authority
- Reconnect never duplicates events or escalates authority
- No private state or secrets enter browser evidence
- HTML remains separate from Runtime and Unity

## Risks

- The browser could show cached state as live during disconnect
- Reconnect could duplicate events or replay commands
- Control state could imply authority that the Runtime has denied
- Live browser proof could accidentally exercise a fixture or stale Runtime

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/83/design.md

Digest: 4cc32245a7c8f499aa4df653ecd32ec29af5202ea548a4ad84cd206a8e1f01bc

## Diagram

.csdlc/prepared/issues/83/diagram.mmd

Digest: 73c36947a5904687d840ee521edfe34e2e888c91649ef3a9f38fe73b83e6117d

## Stop Conditions

- The implementation requires Runtime, Unity, storage, room, notification, or policy work outside the design-approved paths
- The live endpoint cannot support the approved signed Layer 8 message through canonical Runtime ingress
- Required secrets would enter URLs, logs, screenshots, browser persistence, or repository files

## Handoff

Proceed only after doctor readiness.
