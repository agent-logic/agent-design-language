# Structured Planning Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Preserve the existing UI, centralize truthful browser connection state, bind controls to real Runtime authority, add bounded reconnect behavior, and prove the live browser path.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Implement the versioned HTTPS/WSS browser client state machine and fresh/stale projection handling.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Bind menus, controls, proof links, and packet links to real authorized behavior with explicit refusal states.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add and run the focused live browser proof against Runtime HTTPS/WSS, including disconnect and reconnect.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Review exact-head behavior and preserve the strict issue path boundary for publication.",
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

Digest: 592360cc30b2792f88a30e837f2e84cbd2a9392b061f5239314cb505da66d946

## Diagram

.csdlc/prepared/issues/83/diagram.mmd

Digest: 73c36947a5904687d840ee521edfe34e2e888c91649ef3a9f38fe73b83e6117d

## Stop Conditions

- The implementation requires changing Runtime or Unity-owned paths
- The live endpoint does not match the approved Runtime v3 contract
- Required secrets would enter URLs, logs, screenshots, or repository files
- Issue #5836 remains open when final implementation credit is requested

## Handoff

Proceed only after doctor readiness.
