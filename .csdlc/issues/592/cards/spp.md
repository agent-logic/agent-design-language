# Structured Planning Prompt

Template: 1.0.0

Issue: 592

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After #528 is terminal, configure Polis for explicit Vertex AI provider use, document redacted GCP credential/project/location/model sourcing, prove failure modes, and retain canary evidence.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Verify #528 terminal truth through typed C-SDLC readback before binding execution.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Design explicit Vertex AI provider configuration for Polis on GCP.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the bounded configuration path after dependency clearance.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Validate redaction and distinct Vertex AI failure modes without secret exposure.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- #528 gates execution
- Credentials are never printed or committed
- Provider config is explicit not ambient
- V2 remains live authority before #505
- Canary defects are retained instead of worked around silently

## Risks

- GCP project/location authority may be ambiguous
- Vertex API enablement or quota may be unavailable
- Credential source may be confused with secret material
- Tooling setup friction may obscure task readiness

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/592/design.md

Digest: 52344f3d53425ecafda019572e6b91844f58c034170a52585767865c531d9565

## Diagram

.csdlc/prepared/issues/592/diagram.mmd

Digest: 514f9d344bfa5f73c5ddfb14b4869f63254a9723171908bc1b5210332f06b3e2

## Stop Conditions

- #528 is not terminal
- GCP credential or project authority is ambiguous
- A live paid call is needed without explicit authorization
- Tooling cannot represent dependency or credential boundaries truthfully

## Handoff

Proceed only after doctor readiness.
