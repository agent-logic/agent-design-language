# Structured Planning Prompt

Template: 1.0.0

Issue: 113

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Revalidate serial gates and shared-path ownership; freeze the Runtime policy, identity, pagination, presence, freshness, event, and failure contracts; implement the exclusive roster model and serial integration; prove exact Runtime, OpenAPI, large-Polis, browser, reconnect, denial, and rollback behavior; resolve exact-head review; then hand off for separately authorized publication.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Bind #113, reconcile live #110/#113 authority, confirm #83 and deferred work are non-gating, and reserve #142 only for non-local projection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4",
      "AC-10"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the Runtime-owned local roster model, policy filter, stable Shepherd identity, presence/freshness derivation, deterministic pagination, and bounded token contract in issue-owned paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Integrate the local Shepherd projection with production Runtime and the existing Observatory selection surface in isolated shared-path commits; do not claim distributed projection.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run exact roster, production Runtime, OpenAPI, browser selection, scale, freshness, policy, strict-Clippy, and diff proof at one candidate revision.",
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
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Resolve independent exact-head review and leave a truthful unpushed, unpublished execution handoff with shared-path overlap reported.",
    "acceptance_ids": [
      "AC-10"
    ],
    "status": "completed"
  }
]

## Invariants

- Stable agent identity is independent of connection, process, node, location, display name, and roster ordering
- Policy filtering and redaction complete before serialization; client code cannot widen visibility or communication eligibility
- Presence and health never become fresher or more authoritative than their Runtime evidence
- Pagination and events are deterministic, revision-bound, bounded, gap-aware, replay-safe, and never imply unseen completeness
- Relocation has one authoritative current owner and preserves identity while stale owners remain fenced
- No page, event queue, response, browser DOM, wait, retry, or retained proof grows without a declared bound

## Risks

- The open #142 implementation may change the final production identity/topology adapter and require an explicit replan before binding
- The #110 sequence and #122 deferral currently conflict and could make execution authority ambiguous
- Shared #83 Observatory paths could overlap an active implementation or carry a newer contract at handoff
- Cursor or token design could omit, duplicate, reorder, or leak agents across policy changes
- Naive presence derivation could confuse transport reachability, workload availability, health, and migration
- Large rosters could cause unbounded Runtime snapshots, browser DOM growth, event queues, or reconnect replay

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/113/design.md

Digest: ff3228dc05028cccc97d16dbbf1ebb7a9480b19e612096a37c3946347dd6de73

## Diagram

.csdlc/prepared/issues/113/diagram.mmd

Digest: 929a32f90d3e78d69a5aaa7bbe93c94c387b5786af9878eea9083c40783764ed

## Stop Conditions

- #83 or #142 is not merged, terminal, and ancestral to the execution base
- #110 and #122 do not provide one unambiguous serial rule for #113 binding
- An active issue owns or modifies an intended shared path
- Runtime cannot provide stable policy-subject, identity, topology, freshness, capability, or communication-authority inputs without inventing browser authority
- Pagination, event replay, memory, latency, response size, or browser DOM bounds cannot be stated and proven
- Implementation would expose private state, unauthorized existence, credentials, raw provider output, or stale ownership as current
- Scope must widen into another WP-18C child, #83 mutation, public AWS, Unity, or distributed Runtime implementation

## Handoff

Proceed only after doctor readiness.
