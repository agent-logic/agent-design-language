# Structured Planning Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Revalidate #111-#113 terminal ancestry and shared-path ownership; freeze exact room, participant, recipient, security, fan-out, partial-delivery, replay, reorder, and recovery contracts; implement exclusive room logic and serial integrations; run exact PVF lanes and exact-head review; then hand off without publication unless separately authorized.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Re-read #110-#113; require #111, #112, and #113 terminal merged ancestry and no shared-path owner; bind only after every gate passes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement versioned Runtime room membership, frozen recipient sets, exact authorization binding, deterministic bounded fan-out, monotonic outcomes, replay, reorder, and attribution in exclusive paths.",
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
    "id": "S3",
    "action": "Integrate canonical conversation, authority, roster, ACIP-compatible dispatch, control/WSS/OpenAPI, and the accessible Observatory room experience only through handed-off shared paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run exact room, Observatory, OpenAPI, browser, adversarial, resource, strict-Clippy, diff, restart, replay, partial-delivery, and rollback proof at one candidate revision.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve independent exact-head security and correctness review and produce a truthful execution handoff without publication unless separately authorized.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Room membership and every turn recipient set are explicit, bounded, revisioned, canonical, and Runtime-owned
- Exact whole-set authorization completes before sequence commitment or dispatch; no component may widen the recipient set
- Per-recipient outcomes are monotonic and canonical ordering makes aggregate truth independent of completion order
- Exact replay has no mutation or redispatch; conflicting reuse and event gaps fail closed
- Every response is attributed to a stable dispatched participant and correlated to one room turn and delivery record
- Browser, roster, presence, mentions, display names, and provider output never grant membership, routing, or policy authority

## Risks

- Merged #111-#113 contracts may change shared paths or identifiers and require typed replanning before binding
- Concurrent membership, revocation, timeout, cancellation, and late responses can produce ambiguous outcomes unless commit points are explicit
- Fan-out concurrency can make aggregate ordering nondeterministic or permit hidden redispatch
- Roster visibility or UI mentions can be mistaken for participant or recipient authority
- Shared control, OpenAPI, and Observatory paths may remain actively owned at dependency handoff
- Unbounded participants, response fan-in, event buffers, retries, or transcript DOM can exhaust Runtime or browser resources

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/115/design.md

Digest: a702863a1f8c2e7525820552fd74a68f7a7729942a890ba772ae914379b012a8

## Diagram

.csdlc/prepared/issues/115/diagram.mmd

Digest: fb1cf6e38f0404ecf0997395f08af0e30ff6206783c4a22dd91e524bcab37848

## Stop Conditions

- #111, #112, or #113 is open, unmerged, nonterminal, non-ancestral, or lacks a compatible concrete contract
- An active issue owns or modifies an intended shared path
- Exact participant and recipient authority cannot be enforced before dispatch without widening another issue
- Partial delivery, replay, reorder, revocation, restart, or late-response commit points cannot be deterministic and bounded
- Implementation would permit implicit broadcast, cross-Polis routing, unattributed responses, forbidden data, or browser authority
- Any exact PVF lane is missing, selects zero tests, fails, times out, or leaves an unresolved exact-head finding

## Handoff

Proceed only after doctor readiness.
