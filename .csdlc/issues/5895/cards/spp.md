# Structured Planning Prompt

Template: 1.0.0

Issue: 5895

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Inventory current authority, make the smallest proven correction or evidence-only disposition, then prove install, resolve, provenance, retired-surface guard, and one installed lifecycle canary.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Classify every current csdlc-migrate reference and determine whether active drift remains.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Delete only proven active stale expectations and add or tighten one focused negative guard, or record an already-resolved no-code disposition.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Build and install the declared current v2 set and verify selector plus exact provenance.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run the installed claim-free create, validate, and bind canary and retain results.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- V2 selector remains authoritative
- Installed provenance is exact
- No retired binary or compatibility shim
- Historical evidence unchanged
- No broad validation

## Risks

- Mistaking historical text for active authority
- Passing with a stale installed generation
- Overlapping #5883 changes
- Adding code when the issue is already resolved

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5895/design.md

Digest: 2658e02a3e5a3f336825a62bebbc6f5a51ade76603f01ce90c7914a5de24f28c

## Diagram

.csdlc/prepared/issues/5895/diagram.mmd

Digest: 9b1195f1288c7940cab08b36aedd765356d68df7f9aefa09c915af589137e752

## Stop Conditions

- Any need to restore or wrap csdlc-migrate
- Installed provenance cannot be tied to exact source
- The representative lifecycle canary does not use installed binaries
- Scope expands beyond installer/coexistence authority

## Handoff

Proceed only after doctor readiness.
