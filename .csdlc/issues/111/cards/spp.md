# Structured Planning Prompt

Template: 1.0.0

Issue: 111

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #83 is terminal and ancestral, rebase onto its exact dependency head, implement Runtime-owned contracts/session ordering and provider-neutral execution, integrate authenticated Observatory ingress/egress, run focused PVF lanes, and complete exact-head review before any publication.

## Plan

Revision 8

## Steps

[
  {
    "id": "P1",
    "action": "Revalidate live dependencies, establish #83 ancestry, inspect the exact inherited ingress/Observatory contract, and update the typed plan if topology changed.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "P2",
    "action": "Implement canonical conversation contracts and the Runtime-owned bounded session engine with ordering, idempotency, reconnect, cancellation, timeout, and restart semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "P3",
    "action": "Integrate the provider-neutral adapter and authenticated Runtime ingress/egress without widening existing identity or policy authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "P4",
    "action": "Integrate the Observatory client using Runtime conversation identity and cursor while prohibiting acknowledgement or browser text from appearing as agent response.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
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
    "status": "pending"
  },
  {
    "id": "P6",
    "action": "Run bounded exact-head review, fix every actionable finding, and record truthful residual risk before publication handoff.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Runtime alone owns canonical session state, sequence allocation, idempotency, and outcome truth
- One session binds exactly one authenticated operator principal and one policy-reachable agent
- At most one provider dispatch and one correlated public response occur per accepted submission key
- Browser reconnect cannot create turns or replay dispatch, and browser state cannot resume a lost Runtime session
- No provider-specific payload, credential, private cognition, or private agent state crosses the public projection
- #111 consumes only the authenticated reachability boundary inherited from #83; downstream #112 depends on #111 and supplies no authority or gate to this issue

## Risks

- #83 owns overlapping unmerged ingress/control/Observatory changes, so pre-terminal implementation would create stale or conflicting architecture
- Cancellation, timeout, disconnect, and late provider completion can race unless terminal outcome commitment is atomic
- In-memory restart semantics may be mistaken for durable continuity unless the unavailable boundary is explicit
- A browser may accidentally render an ingress acknowledgement or result hash as conversational content

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

- #83 is not terminal, independently validated, and ancestral to the candidate base
- Exact post-#83 affected paths differ materially from this SPP without a typed replan
- Any implementation would need to define broader Layer 8 identity or policy semantics outside the inherited #83 boundary; route that downstream scope to #112
- Any implementation requires durable history owned by #114 or multi-agent routing owned by #115
- A proving validator target is unavailable without an explicit initialized-phase defer and issue ownership
- Any path would require product edits on root main or mutation of #83

## Handoff

Proceed only after doctor readiness.
