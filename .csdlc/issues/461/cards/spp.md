# Structured Planning Prompt

Template: 1.0.0

Issue: 461

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Remove TLS argv fields, validate config-owned paths, update the Guardian-generated config, and prove the config-only lifecycle path.

## Plan

Revision 1

## Steps

[
  {
    "id": "step-1",
    "action": "remove lifecycle TLS command flags and consume validated config fields",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "update Guardian config generation and regression coverage",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Runtime configuration remains the sole TLS authority
- TLS material and sensitive paths never enter command argv
- invalid or ambiguous TLS configuration fails closed

## Risks

- fixture configuration may diverge from production schema
- path validation may regress HTTPS or WSS lifecycle startup

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/461/design.md

Digest: d900815e5a20e2406883bde10ba970d7e0e1083a737a14d96a7b4709b2c43246

## Diagram

.csdlc/prepared/issues/461/diagram.mmd

Digest: 7708a92f20db0256d0d15f5690a029e2a705848c1729de336f54c8b7da95b2ec

## Stop Conditions

- the fix requires certificate issuance or public DNS changes
- the Runtime configuration cannot represent required TLS authority

## Handoff

Proceed only after doctor readiness.
