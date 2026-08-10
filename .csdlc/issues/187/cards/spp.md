# Structured Planning Prompt

Template: 1.0.0

Issue: 187

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Run bounded two-hour local and four-hour hybrid production soaks, preserve every attempt, prove resource and error thresholds, replay deterministically, verify cleanup, and synthesize truthful residual risks.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Freeze exact local and hybrid workload, fault schedule, duration, resource, cost, and error thresholds plus attempt identifiers before either soak begins.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run the uninterrupted two-hour local production soak and retain per-phase commands, clocks, terms, committed indexes, envelopes, source/model digests, faults, resources, errors, and cleanup receipts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "After business-AWS identity and budget verification, run the uninterrupted four-hour hybrid production soak with the same producer-derived denominator and provider resource/cost readback.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Preserve failed attempts without resetting their clocks or evidence, classify every expected and unexpected failure, and stop when resource or error thresholds are exceeded.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Replay retained deterministic inputs without live-provider dependence and compare committed outcomes, terms, indexes, envelopes, and state digests exactly.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Verify local process/port cleanup and AWS provider cleanup after normal completion and every failure phase; reject any surviving process or cloud resource.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S7",
    "action": "Synthesize the exact qualification report, non-claims, residual risks, and all attempt receipts; run the issue validator and independent exact-head review without claiming release approval.",
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

- Issue DRT-07 owns only its declared repository paths and named external operation/evidence boundary.
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
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/187/design.md

Digest: bcc95757e789cd3e054f41a2cd15c49190f1c1a308966dc15fe3f180c9a6ae2d

## Diagram

.csdlc/prepared/issues/187/diagram.mmd

Digest: 4499a2c837b9f57dd68bf4fba95fc28f4ee10bad1c4f72ab5640cd785491725c

## Stop Conditions

- A soak restarts without retaining the failed attempt
- Resource or error thresholds are exceeded
- Replay diverges
- Any cloud or local process survives cleanup
- Typed doctor is not ready
- A required dependency is nonterminal
- An owned-path collision is discovered

## Handoff

Proceed only after doctor readiness.
