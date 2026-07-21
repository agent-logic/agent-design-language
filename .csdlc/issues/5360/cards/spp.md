# Structured Planning Prompt

Template: 1.0.0

Issue: 5360

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Render and validate six current-registry cards; freeze exact #5351 terminal gating, preparation and future path ownership, claim taxonomy, product boundaries, COTS, budgets and PVF; obtain bounded review and fix findings; typed approve, bind and doctor; commit and push preparation only.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Complete six cards, design, diagram, exact paths, dependency gate, COTS, budgets, PVF, bounded review and fixes, typed approval/bind/doctor, commit, and push preparation",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-6",
      "AC-8"
    ],
    "status": "in_progress"
  },
  {
    "id": "S2",
    "action": "Wait fail-closed until #5351 is merged, typed closed_out, claim-free, retained-receipt-backed, and ancestral",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Amend exact reviewed shared-document paths, inventory evidence, reconcile claims, and run focused and complete alignment validation",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run exact review, typed publication, green CI, authorized serialized merge, post-merge proof, typed closeout, and release WP-18",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- preparation owns only four exact #5360 issue-local lifecycle and evidence paths
- no shared-document or broader claim begins before the complete #5351 terminal gate
- unsupported, stale, missing, deferred, or contradictory evidence cannot become a proven release claim
- documentation reconciliation preserves separate product ownership and never becomes runtime, deployment, review, publication, merge, or closeout authority
- Runtime v2, credentials, host-absolute retained paths, hard-coded addresses, product changes, and new dependencies are forbidden during preparation
- all applicable validation, review, CI, merge, post-merge, closeout, and WP-18 release gates complete without deferral

## Risks

- aggregate milestone prose could be mistaken for exact product proof
- #5351 or product revisions could drift between quality closeout and documentation execution
- unsupported claims could be softened into optimistic release wording
- broad documentation ownership could collide with active product or milestone work
- alignment tooling could duplicate structured parsers or release databases
- retained evidence could leak host paths, credentials, addresses, stale identities, or private context

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5360/design.md

Digest: bdfce9b6454323e72a3f62c2e00065e53897dc06b564ba8dce426337163040fb

## Diagram

.csdlc/prepared/issues/5360/diagram.mmd

Digest: 45598598c35e6ef60d4b91f31a62ee3fef9a82b3f3246ab50647cab4eb4c3176

## Stop Conditions

- #5351 lacks actual merge, typed closed_out, claim release, retained merged receipt, or ancestry
- a required statement lacks exact evidence or has contradictory owner truth
- a future documentation path is unreviewed, colliding, generated, or outside typed claim scope
- implementation would change product behavior or require Runtime v2, AWS, credentials, paid services, hidden network authority, hard-coded addresses, or private context
- a new dependency, duplicate authority, unsupported parser, budget breach, failed/deferred gate, or stale review appears
- WP-18 would begin before #5360 merged typed closeout

## Handoff

Proceed only after doctor readiness.
