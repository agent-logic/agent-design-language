# Structured Planning Prompt

Template: 1.0.0

Issue: 5336

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Pin the truthful Runtime v3 baseline, define live parity and feature preservation, organize four parallel lanes, repair canonical dependencies and proof surfaces, validate, review, and publish the plan.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Pin Runtime v3 implementation, fixture, feature, and budget baseline",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Define four implementation lanes and corrected dependency graph",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Update every affected canonical planning, proof, demo, release, and handoff surface",
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
    "id": "S4",
    "action": "Validate YAML, JSON, links, planning truth, and diff hygiene",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run bounded review, fix findings, and prepare publication",
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

- No tracked work on main
- Generated cards remain typed-tool owned
- Fixture-only evidence is not live parity
- No feature disappears during Runtime v2 deletion
- No AWS use

## Risks

- The existing plan assumes Runtime v3 parity that current source does not expose through live ingress
- Two Runtime v3 crates can grow in parallel unless one canonical owner is selected
- A broad #5361 acceptance issue would prevent safe parallel implementation without bounded child lanes
- Cutover or deletion could erase Runtime v2-only features if the feature ledger is incomplete

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5336/design.md

Digest: 9023d6d2842b7382845abc74d0252d1c267c175e366a79144c1248dd14e36f3f

## Diagram

.csdlc/prepared/issues/5336/diagram.mmd

Digest: 6c2a925bbae9f08af3006cc52a80a1943c2117ec1749f14840c9e5a905afc856

## Stop Conditions

- The plan would require Runtime implementation or deletion in #5336
- A proposed lane duplicates another milestone owner without explicit disposition
- The feature ledger cannot account for a Runtime v2-only capability
- The work would require AWS

## Handoff

Proceed only after doctor readiness.
