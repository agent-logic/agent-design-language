# Structured Planning Prompt

Template: 1.0.0

Issue: 323

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and bind #323, add the smallest typed migration command/schema/store path for active bound issue identity recovery, cover it with focused tests, validate the owner tooling, obtain review, and then use it later to recover #5913.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Add typed request/report schema and csdlc-issue subcommand for bound issue identity migration.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement fail-closed namespace/record/card migration with provenance and publication truth handling.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused regression coverage for #5913 -> #322 and unchanged finish invariants.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation and exact-head review.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Finish keeps exact issue/repository identity checks
- Migration is request-driven and idempotent only for the exact source/target/provenance
- Existing terminal records cannot be rewritten
- Existing target lifecycle truth cannot be overwritten silently
- Generated locks remain excluded from substantive proof

## Risks

- Identity migration can become a dangerous generic state editor if underconstrained
- Published-phase recovery may need to clear publication truth and require republish
- Namespace move must avoid losing append-only audit/provenance

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/323/design.md

Digest: 35d6b4863cc00f61360b746971834a36d46e41df57f7729a41a6cc466306b270

## Diagram

.csdlc/prepared/issues/323/diagram.mmd

Digest: 46c5608ffec4e90eb969e119d97fb5f733be2c4e17cc9995de004a89f1d903c7

## Stop Conditions

- Implementation would require hand-editing live #5913 records
- Implementation would weaken finish canonical identity validation
- Implementation collides with active #112 or #298 paths
- Focused tests or exact-head review fail

## Handoff

Proceed only after doctor readiness.
