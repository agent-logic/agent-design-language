# Structured Planning Prompt

Template: 1.0.0

Issue: 167

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Define state.json as the sole versioned authority and deterministically project all six cards, audit view, evidence indexes, optional placeholders, and drift diagnostics from canonical state plus declared immutable inputs.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Define the versioned aggregate, enums, embedded audit events, no-pruning policy, digest profile, and schema evolution rejection behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Encode the per-card/per-phase required/optional field table and one declared unset placeholder for all six lifecycle cards.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement deterministic card, audit.jsonl, and evidence-index projection builders without reading rendered Markdown as authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Implement projection digest comparison, typed drift diagnosis, and repair from canonical state only.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Build golden fixtures for every card/phase, unknown schema/enum, missing required field, optional unset field, and deterministic rerender.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Retain compatibility results and stop on nondeterminism, undeclared authority, unknown-field loss, or Markdown input.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Remove projection scratch output, retain only golden fixtures, and prove repair leaves canonical state unchanged while regenerated views match their digests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Issue V3-06 owns only its declared repository paths and named external operation/evidence boundary.
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

.csdlc/prepared/issues/167/design.md

Digest: ee4e0cbbe25e26356a6b0bf30a999defed39f32f79f83d660f903ba4c8bbbee3

## Diagram

.csdlc/prepared/issues/167/diagram.mmd

Digest: c10773ec4da687d9a9a3bd17ff61e1eec0119a2cab7b44477a6eddfa17ededb9

## Stop Conditions

- A card requires undeclared authority, rendering is nondeterministic, or state evolution can silently discard unknown fields.
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
