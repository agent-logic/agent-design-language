# Structured Planning Prompt

Template: 1.0.0

Issue: 512

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

After OBS-A and Sprint 8 gates are terminal, implement the HTML Observatory redesign against authentic Runtime projections and prove browser, accessibility, redaction, and recovery behavior.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Confirm #511 reviewed contract and #536 Sprint 8 coordination are terminal before binding execution.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement OBS-A view/state/accessibility contracts in the HTML Observatory.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Replace any mock data with authentic Runtime projection consumption and redaction handling.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run exact browser, accessibility, redaction, recovery, and review proof.",
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

- #511 is terminal before execution
- #536 is terminal before execution
- No mock substitutes for required Runtime route
- #84 remains non-gating backlog
- V3 remains non-authoritative before #505

## Risks

- OBS-A may change implementation requirements
- Runtime projection route may be unavailable
- Mock data may be accidentally retained
- Accessibility recovery behavior may lag visual redesign

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/512/design.md

Digest: 45854156618cc355f3db36aa4668fab34e8b10024b557cb96f114520b1dc8c94

## Diagram

.csdlc/prepared/issues/512/diagram.mmd

Digest: ebda5846e223f52d65d645672972fd0d29384b18b6c21b726014ee310920552c

## Stop Conditions

- Issue #511 is not reviewed and terminal
- Issue #536 is not terminal
- A mock substitutes for the required Runtime route
- The implementation requires #84, #251, or #122

## Handoff

Proceed only after doctor readiness.
