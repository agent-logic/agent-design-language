# Structured Planning Prompt

Template: 1.0.0

Issue: 13

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add explicit producer selectors, preserve fail-closed aggregation, prove the Runtime-only and full-coverage routes, review exact head, and publish a PR closing #13.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Model explicit producer selection and the bounded Runtime-owner route.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply job-level guards and deterministic aggregate checks.",
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
    "id": "S3",
    "action": "Add and run focused producer/aggregator routing contracts.",
    "acceptance_ids": [
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Review exact head, publish the PR, and retain GitHub timing evidence.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- No unselected producer work
- Selected coverage remains fail closed
- Required aggregate remains terminal
- No threshold weakening

## Risks

- A selector combination could expect the wrong producer result
- Runtime-only proof could accidentally route to fast workspace coverage
- Full coverage could omit Runtime coverage

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/13/design.md

Digest: 8e4518a9091499661d221171aba42f127a973ecbd485144705e1b9490bbac5f7

## Diagram

.csdlc/prepared/issues/13/diagram.mmd

Digest: 3b47586ddeacec60057415846e1564d70298c075275901d62cdefffa172588d1

## Stop Conditions

- Producer selectors cannot be derived deterministically
- Required aggregate semantics cannot distinguish selected from skipped
- The fix requires changing coverage thresholds

## Handoff

Proceed only after doctor readiness.
