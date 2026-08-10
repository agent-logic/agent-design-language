# Structured Planning Prompt

Template: 1.0.0

Issue: 182

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Build a deterministic ACIP conformance corpus and independent replay verifier: freeze canonical vectors, exercise every semantic mutation and ordering failure, compare exact digests, and stop before distributed provisioning.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Freeze the supported ACIP message-family denominator and canonical positive vectors, including identity, authority, permit, causation, correlation, sequence, term, polis, and payload bindings.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Generate mutation vectors for every bound field plus duplicate, reordered, stale, malformed, unsigned, wrong-domain, and cross-polis cases, each with one typed expected outcome.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run encode-decode-reencode and mutation conformance through the production ACIP implementation, retaining byte digests and typed outcomes rather than assertion labels or aggregate pass counts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Replay the retained input corpus in an independent process with no hidden mutable state and compare message, committed-outcome, and receipt digests exactly.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run the issue validator and focused conformance lane; fail on any noncanonical round trip, unbound semantic field, missing mutation class, nondeterministic outcome, or digest divergence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Complete independent exact-head review and publish the deterministic corpus and verifier without provisioning a cluster or replacing ACIP.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue DRT-02 owns only its declared repository paths and named external operation/evidence boundary.
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
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/182/design.md

Digest: 21d181702a322c5928f13e2c979767f8e732ecfa2e2af789015a4b1cf7b5b501

## Diagram

.csdlc/prepared/issues/182/diagram.mmd

Digest: 0ddca25810a69ca5b537ae13cd4097e757cd49ccdefb01d5abfcb25a25e01620

## Stop Conditions

- A semantic field is authenticated but not canonically bound
- A noncanonical representation round-trips to a different value
- Replay requires hidden mutable state
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
