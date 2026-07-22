# Structured Planning Prompt

Template: 1.0.0

Issue: 5500

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Prepare and independently review a small read-only extension of the existing milestone dashboard, hold product scope behind exact #5498/#5349 terminal gates, then later implement deterministic secure fixture-backed projection and Runtime Observatory composition.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Render, validate, and independently review all six cards, design, diagram, dependencies, protected paths, COTS, budgets, security boundaries, and PVF lanes",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Wait fail-closed for #5498 and final gate #5349 merged typed closeout, claim release, retained receipts, and ancestry",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Extend the existing dashboard with typed snapshot and bounded live observation adapters, safe deterministic rendering, mobile layout, and zero mutation capability",
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
    "id": "S4",
    "action": "Run focused and full proof, exact-revision review, typed publication, serialized merge, post-merge validation, and closeout",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- no tracked work on main
- no product implementation before #5498 and #5349 terminal gates
- preparation claim protects only issue-local lifecycle paths
- future product paths are docs/tooling/milestone-dashboard and adl/tools/test_milestone_dashboard.sh only until reviewed amendment
- no Runtime v2, AWS, raw gh, provider, unauthenticated HTTP, hard-coded IP, or secret retention
- zero mutation or autonomous merge/closeout authority
- deterministic safe rendering with explicit provenance and freshness

## Risks

- the view could accidentally become a second source of truth or mutation surface
- stale retained or partial live observations could be displayed as current
- untrusted text or URLs could create XSS, credential, or origin-boundary defects
- runtime observations could overwrite lifecycle or GitHub authority
- dashboard scope could overlap #5502 convergence or another child
- a small static surface could regrow into a backend framework

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5500/design.md

Digest: 430ffbd58df8fd8f05dce14f34ebb1bae0e7850fe1b8c28fe7a442315aeadfc6

## Diagram

.csdlc/prepared/issues/5500/diagram.mmd

Digest: f6f86ac1a71dcb887ee6a2b6ecec6d9e7c550946bb34fe2ff57fc952688e2f72

## Stop Conditions

- #5498 or #5349 lacks a merged typed closed_out receipt, released claim, or ancestry
- a future dashboard path overlaps an active typed claim including #5502 or another workcell child
- implementation requires mutation, a second state store, a backend, or lifecycle authority
- live access cannot remain authenticated HTTPS and configuration-driven
- the declared LoC, test, time, dependency, or security budget is exceeded without reviewed typed exception

## Handoff

Proceed only after doctor readiness.
