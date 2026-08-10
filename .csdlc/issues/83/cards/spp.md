# Structured Planning Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Complete the investor-visible Layer 8 chat slice with fresh live capture time and one governed signed identity-envelope path for Layer 8 and agent-to-agent messages: canonical sender verification before delivery, recipient-selected Runtime-side response signing, browser verification against the live roster, no browser private-key material, reconnect continuity, and retained live proof.

## Plan

Revision 20

## Steps

[
  {
    "id": "S1",
    "action": "Retain the live Runtime roster and WSS state machine with fresh, stale, unavailable, and bounded reconnect behavior, and drive every visible Capture Time surface from the same fresh live timestamp.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement one bounded signed identity envelope for Layer 8 and agent-to-agent messages with sender identity, recipient identity, correlation, causation, nonce, sequence, expiry, content, key id, algorithm, and signature verified against the canonical identity registry before delivery.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Configure one distinct Runtime-side signing identity per roster agent, sign the response with the selected recipient identity, verify before canonical completion/public projection, and expose only public verification identity in the roster.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Replace browser private-key input with authenticated WSS Layer 8 intent delegation; verify the selected agent response against its roster key and display verified identity without persisting or rendering private material.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused forged-envelope, wrong-agent-key, wrong-correlation, expiry, replay, response-signature, fresh-capture, Runtime, OpenAPI, browser-shell, live-browser, reconnect, redaction, refusal, and diff-hygiene proof.",
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
  },
  {
    "id": "S6",
    "action": "Review exact-head behavior and preserve the strict no-publication boundary while legacy issue #5836 remains open.",
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

Digest: 3574fc0bab0670d64c9f5bce7c60cd3ec26a24926cc7c71726983ec462c9fb50

## Diagram

.csdlc/prepared/issues/83/diagram.mmd

Digest: 73c36947a5904687d840ee521edfe34e2e888c91649ef3a9f38fe73b83e6117d

## Stop Conditions

- The implementation requires Runtime, Unity, storage, room, notification, or policy work outside the design-approved paths
- The live endpoint cannot support the approved signed Layer 8 message through canonical Runtime ingress
- Required secrets would enter URLs, logs, screenshots, browser persistence, or repository files

## Handoff

Proceed only after doctor readiness.
