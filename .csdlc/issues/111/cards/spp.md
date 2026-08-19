# Structured Planning Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Under live #110 decomposition authority, implement a bounded authenticated selected-agent conversation intent through canonical Runtime ingress, expose only a typed public reply, render it safely in the Observatory, run focused PVF lanes, and complete exact-head independent review before publication. #83 remains a separate asynchronous decomposition reconciliation and is not an execution gate.

## Plan

Revision 14

## Steps

[
  {
    "id": "P1",
    "action": "Revalidate live #110 decomposition authority, inspect the existing Runtime ingress and Observatory contract, bind #111, and record any topology divergence through a typed replan.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "P2",
    "action": "Implement canonical conversation contracts and the Runtime-owned bounded session engine with ordering, idempotency, explicit negative outcomes, and a fail-closed unavailable boundary.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "P3",
    "action": "Integrate the provider-neutral adapter and authenticated Runtime ingress/egress without widening existing identity or policy authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "P4",
    "action": "Integrate the Observatory client using Runtime conversation identity and correlation while prohibiting acknowledgement, result hash, or arbitrary adapter output from appearing as an agent response.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "P5",
    "action": "Run the declared focused PVF lanes and fix all failures at the exact implementation revision.",
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
    "id": "P6",
    "action": "Run bounded exact-head review, fix every actionable finding, and record truthful residual risk before publication handoff.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Runtime alone owns canonical session state, sequence allocation, idempotency, recipient eligibility, and outcome truth
- One conversation binds exactly one authenticated operator session and one visible running agent
- At most one adapter dispatch and one correlated typed public response occur per accepted turn identifier
- Browser reconnect cannot create turns or replay dispatch, and a replayed result without a pending turn cannot render
- No provider-specific payload, credential, private cognition, private agent state, acknowledgement, or result hash crosses the public reply projection
- #111 uses the existing authenticated Runtime reachability boundary under live #110 authority; #83 is asynchronous reconciliation and downstream #112 supplies no authority or gate

## Risks

- Concurrent #113 roster work overlaps Runtime control and Observatory paths, so integration must preserve both issue-owned commits and rerun exact focused proof
- A public helper or browser-held key could bypass authenticated WSS authority unless the conversation entrypoint remains private and session-gated
- Duplicate turn identifiers, disconnects, and late adapter completion can confuse visible outcome truth unless canonical ingress remains the sole idempotency authority
- An adapter could return arbitrary fields or a mismatched recipient unless the public projection is strict and fail closed
- A browser may accidentally render an acknowledgement, result hash, stale replay, or arbitrary adapter output as conversational content
- Durable history, rooms, and broader identity hardening must remain routed to #114, #115/#116, and #112

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

design/issue-111.md

Digest: 807112016269e5b923102d52fcaa63f235d53e71174c271b5e256bcfbe01fe67

## Diagram

design/issue-111.mmd

Digest: 7a7b26563a9aa9e1466f66e846ffecca7c34ece7a60dc705a9f60c4d9e0f1410

## Stop Conditions

- Live #110 decomposition authority is revoked or the bounded #111 selected-agent conversation scope is reassigned
- The implementation requires broader Layer 8 identity or policy hardening owned by downstream #112
- The implementation requires durable history owned by #114 or multi-agent rooms and routing owned by #115 and #116
- The implementation would expose provider payloads, private cognition, credentials, or browser-held signing keys
- A proving validator target becomes unavailable or the exact changed paths diverge materially without another typed replan
- Any path would require product edits on root main or mutation of #83, #110, #122, or #213

## Handoff

Proceed only after doctor readiness.
