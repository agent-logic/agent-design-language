# Structured Planning Prompt

Template: 1.0.0

Issue: 169

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Implement one crash-consistent transaction store where state.json atomic replacement is the sole commit point, projections follow state, durable remote intents precede effects, and platform-specific recovery never guesses.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify terminal V3-D11 and implement the approved per-platform commit/capability matrix, including fail-closed Windows mutation where unproven.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement lock, generation/digest CAS, staging, file/parent sync, and atomic state.json replacement in store/transaction.rs.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement state-first projection replacement and explicit repair-required results without rollback or projection authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement durable pre-network intents, exact remote readback reconciliation, competing-mutation blocking, and state consumption of resolved outcomes.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Implement pure recovery classification/plan building and execute every repair through the transaction API.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Run interruption, stale-writer, projection-failure, intent, platform-capability, native-platform, and concurrency fault matrices; stop on guessed recovery.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Release transaction locks, reconcile or explicitly retain every durable intent, remove staging files only through classified recovery, and prove no ambiguous partial state remains.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-08 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.
- No unsupported completion, legal, production, or release claim
- No mutation outside exact owned paths

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/169/design.md

Digest: a72005025ab0716d422041ddd12876cb896d5d406e8421beb1f05b146592ddeb

## Diagram

.csdlc/prepared/issues/169/diagram.mmd

Digest: 231177475f465bbbefdbe21cc55a4d9197996dd13f368b1b47d4017213b492f6

## Stop Conditions

- Recovery requires guessing, a partial projection becomes authority, remote mutation enters a local transaction, or platform semantics cannot satisfy the declared commit guarantee.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
