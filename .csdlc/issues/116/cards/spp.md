# Structured Planning Prompt

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #111/#112/#114 are terminal and ancestral, inspect their exported contracts, replan typed cards if paths drift, implement the isolated attention domain and schema, compose authorization/durability/conversation adapters, add the Observatory inbox, run focused PVF lanes and adversarial recovery proof, then hand off for exact-head review.

## Plan

Revision 1

## Steps

[
  {
    "id": "P1",
    "action": "Wait for #111, #112, and #114 terminal merged ancestry; inspect exact exported contracts and apply typed replan if any prepared path or assumption drifted.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "P2",
    "action": "Implement the isolated typed attention lifecycle, schema, authorization inputs, queue ordering, deduplication, expiry, grouping, retention, rate limits, and durable transition receipts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "P3",
    "action": "Compose narrow Runtime API adapters with #111 reply ingress, #112 authorization/audit, and #114 durability/recovery without editing predecessor-owned internals.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "P4",
    "action": "Implement the accessible Observatory inbox, unread projection, filters, deep links, outcomes, notification preferences, and explicit degraded/reconnect states.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "P5",
    "action": "Run focused PVF lanes for schemas, Runtime behavior, overload/spoofing, restart/reconnect/recovery, and browser behavior; remediate all failures.",
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
    "id": "P6",
    "action": "Run bounded exact-head review, resolve every actionable finding, record truthful SRP/SOR evidence, and hand off to publication authority without publishing silently.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Runtime, not the Observatory, owns canonical attention state and transition authority.
- Accepted actionable requests are never silently lost; overload outcomes are bounded and durably accounted.
- Deduplication never merges distinct source, authorization, Polis, reason, correlation, or governed-work identities.
- Expiry and rate decisions use Runtime authority rather than client clocks or client priority.
- Every read, deep link, transition, and reply is re-authorized; revocation fails closed.
- Restart and reconnect preserve ordering and actionable state without duplicate notifications.
- No projection exposes secrets, private cognition, raw policy inputs, or unauthorized conversation content.

## Risks

- Predecessor exported APIs and paths may differ from preparation assumptions; typed replan is mandatory before implementation if they drift.
- Deduplication or grouping could erase distinct authorization/correlation identities.
- Priority and quiet-mode semantics could create starvation or covert authority escalation.
- Restart recovery could duplicate notification delivery or resurrect expired/revoked requests.
- Browser caching and deep links could leak revoked or cross-principal state.
- Shared Runtime API and HTML files may overlap downstream sibling work and require serial integration.

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/116/design.md

Digest: 330de9cb9c3cdf277dfcf24319c01c2d857f90a14149c9f5d30109d89f87d105

## Diagram

.csdlc/prepared/issues/116/diagram.mmd

Digest: ccd9e09571310dac3deb563403180da159e6e97ee75c05449cf2e145874bd0a3

## Stop Conditions

- Stop before bind while #111, #112, or #114 is non-terminal or not ancestral to the execution base.
- Stop and typed-replan if predecessor contracts, ownership, API paths, schemas, or shared-file assumptions differ materially.
- Stop on any request to mutate #83, widen into push-vendor/AWS/public deployment, or redefine predecessor-owned authority.
- Stop on ambiguous identity, authorization, expiry, deduplication, recovery, projection, or retention semantics.
- Stop before publication, push, PR creation, merge, issue closure, or cleanup without separate operator authorization.

## Handoff

Proceed only after doctor readiness.
