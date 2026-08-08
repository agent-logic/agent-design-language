# Structured Planning Prompt

Template: 1.0.0

Issue: 55

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Trace the exact coverage aggregation job and focused contracts, change only its runner selector, add a regression assertion, run focused syntax and routing proof, then obtain exact-head review and publish.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Confirm heavyweight versus lightweight coverage job boundaries and current runner-contract tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Route only adl_coverage_hosted through the established selected 16-core runner expression.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add focused fail-closed regression coverage for the aggregator runner route.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused workflow and contract proof, validate typed truth, and obtain exact-head review.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Only heavyweight coverage aggregation changes runner class
- Stable coverage status semantics remain unchanged
- Producer and publication boundaries remain unchanged
- No AWS route is introduced
- Regression proof is focused and deterministic

## Risks

- The job identifier differs from its display name and the wrong aggregator is changed
- A broad text assertion passes while stable-status routing regresses
- Runner selector syntax is malformed
- Focused tests accidentally encode unrelated workflow details

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/55/design.md

Digest: a6de185b7f76f44b8285dedfae5df578599b41cd96f313f512c80be7f9977f05

## Diagram

.csdlc/prepared/issues/55/diagram.mmd

Digest: 57d94a6b84ff218cc2546faa888e65036290214c13c80494235da14238b6b809

## Stop Conditions

- The requested job cannot be separated from the lightweight stable aggregator
- The change requires modifying coverage thresholds, test selection, or artifact semantics
- The established heavy-runner selector is unavailable
- Scope expands into AWS or general CI redesign

## Handoff

Proceed only after doctor readiness.
