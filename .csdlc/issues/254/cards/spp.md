# Structured Planning Prompt

Template: 1.0.0

Issue: 254

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Retain one Azure heavy workspace coverage producer, publish summary/provenance as the artifact, and convert the aggregate check into a light summary verification and merge gate.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Replace the workspace shard/profraw aggregation topology with one workspace summary producer and one light aggregate verifier.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Update CI contract tests so aggregate Rust coverage reruns fail the policy checks.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused local validation, complete review, and publish a ready PR.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- required adl-coverage remains fail-closed
- producer artifacts bind to the current head SHA and run attempt
- aggregate job never recompiles Rust coverage
- Azure heavy runner is reserved for Rust-producing jobs
- workflow contract tests encode the cost-control invariant

## Risks

- artifact layout mismatch could fail the aggregate provenance check
- coverage summary merge must continue to see the expected workspace path

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/254/design.md

Digest: fb61a0700ed333514f4b40295c7b7b013d659d06b88b9a272580cfc9eab46ce9

## Diagram

.csdlc/prepared/issues/254/diagram.mmd

Digest: d5505692e64fc5a2688b43bb77f53910b4d9eb485cf36492673fa6079a2adea9

## Stop Conditions

- focused CI contract validation fails
- typed lifecycle bootstrap or publication fails closed
- pre-PR review returns actionable in-scope findings
- remote state is ambiguous

## Handoff

Proceed only after doctor readiness.
