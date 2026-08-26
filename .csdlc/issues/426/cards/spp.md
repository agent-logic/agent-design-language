# Structured Planning Prompt

Template: 1.0.0

Issue: 426

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Select a backend by OS, implement bounded Linux process lifecycle semantics, test both routing and Linux behavior, document, review, and publish.

## Plan

Revision 4

## Steps

[
  {
    "id": "step-1",
    "action": "Implement explicit OS backend selection and Linux lifecycle functions",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "step-2",
    "action": "Add deterministic tests and Linux documentation",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "step-3",
    "action": "Run bounded and Gemini reviews, fix findings, and publish",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Darwin launchd behavior remains available
- unsupported OS fails closed
- tests never mutate real service state
- no #268 or #269 AWS action

## Risks

- PID reuse could target an unrelated process
- OS test overrides could escape test mode
- Linux shell portability could drift

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/426/design.md

Digest: 1a14733786a8e9f3931307940f1b8e78a53ea0c64c9c031f9ee9928d0c63362a

## Diagram

.csdlc/prepared/issues/426/diagram.mmd

Digest: 52f17edf80108c10fe5b3a9ac1d3a4b0a9691ea01d07aba76a9e0063255c6224

## Stop Conditions

- Linux backend could signal a process not proven to be the configured Runtime
- Darwin behavior regresses
- Gemini reports an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
