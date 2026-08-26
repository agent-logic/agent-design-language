# Structured Planning Prompt

Template: 1.0.0

Issue: 540

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add characterization tests for the configured extra origin, repair only any failing local CORS/config path, validate focused Runtime kernel lanes, and prepare for exact-head review.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Inspect current Runtime v3 CORS/configuration path and add failing-or-characterizing tests for localhost:8000.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply the smallest implementation repair only if the characterization tests expose a gap.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused configuration/control validation and prove port 8000 remains non-listening for ADL software.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Allowed origins remain exact configured origins, not wildcard-open.
- Origin values remain full browser origins with scheme, host, and port.
- Port 8000 is never an ADL-owned bind/listen port.
- Existing HTTPS Observatory origin remains accepted.

## Risks

- Confusing an Origin header value with an ADL server bind port.
- Accidentally widening CORS defaults.
- Changing documented API behavior when only local configured-origin proof is needed.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/540/design.md

Digest: 0df482853fe212f927dfba2c806ef7c2987e5835622d8ffb5b54b4dc9a6e62d6

## Diagram

.csdlc/prepared/issues/540/diagram.mmd

Digest: 217557995e9b3588958fa94510b77f5707aa53cda49e4a2f361509743ad7f42f

## Stop Conditions

- Any need to change public API, authentication, or production ingress.
- Any need to bind ADL software to port 8000.
- Any collision with active Runtime refactor work beyond the listed test/config/control files.

## Handoff

Proceed only after doctor readiness.
