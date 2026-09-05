# Structured Planning Prompt

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Define the partial schema and cadence, implement bounded local snapshots for all residents, add an asynchronous durable S3 spool and restore rules, project per-agent continuity through API and Observatory, add Terraform, prove accelerated end-to-end behavior, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Define configuration bounds, partial schema, lineage, redaction, atomic local storage, and restore rules.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the bounded periodic coordinator for Shepherd and dynamic residents with deterministic roster-change and overlap behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the durable bounded asynchronous S3 spool, idempotent archive worker, and Terraform security boundary.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Project per-agent continuity fields through roster/detail APIs and render them in Observatory.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run accelerated production-shaped tests, focused validation, exact-head review, and typed publication.",
    "acceptance_ids": [
      "AC-6",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Cloud unavailability never blocks or terminates Runtime
- At most one partial per agent per sequence is authoritative
- Every partial is bound to canonical identity and continuity lineage
- The only unarchived recoverable local partial is never evicted
- API and Observatory never infer checkpoint or archive success
- Secrets and credentials never enter partials, projections, logs, or evidence

## Risks

- Periodic serialization could contend with active conversations
- Roster mutation during a cycle could skip or duplicate an agent
- Upload backlog could consume unbounded disk
- Restore could apply a valid partial to the wrong lineage
- Public API fields could leak infrastructure details
- Terraform could accidentally couple checkpoint storage to log retention

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/695/design.md

Digest: 2bb94e45f8196e51920b846e1de50a8c83b60ecfc12a1505be154b3aeb5aeb6a

## Diagram

.csdlc/prepared/issues/695/diagram.mmd

Digest: 4326f3bec5441bd45699951e2bc7da6847dbe3d7b4d8500ce2e15dc62f6a6b16

## Stop Conditions

- Implementation requires synchronous cloud I/O on an agent or readiness path
- A partial cannot be bound to canonical agent and continuity lineage
- Local spool or retention cannot be bounded without losing the only recoverable partial
- Scope expands into log archival, provider execution, A2A, or transcript recovery
- Validation requires live Wuji restart or AWS resource creation
- Exact-head review has unresolved findings

## Handoff

Proceed only after doctor readiness.
