# Structured Planning Prompt

Template: 1.0.0

Issue: 525

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Freeze the merged planning head, construct the complete denominator, run independent findings-first review and focused validators, and retain one exact-revision report.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Record the exact merged #524 revision, worktree state, and complete planning denominator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Run independent semantic review across scope, dependencies, units, deferrals, gates, and non-claims.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Retain the findings-first report; route actionable findings to #523 or #524 and require repair plus fresh #525 review before #526.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Review is independent and read-only
- Exact revision and denominator are explicit
- Every finding is evidence-backed
- Changed planning requires fresh review

## Risks

- A thin denominator may miss planning surfaces
- Validator success may mask semantic gaps
- Planning may change after review

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/525/design.md

Digest: 661be78a1e440440ea1d55e9728be1db197b2eaa610f724c433ec40dd8f68218

## Diagram

.csdlc/prepared/issues/525/diagram.mmd

Digest: 89d5d63669b4265d8f17bd1d236d85ea066814ebbc5c6a22a65abe7c92fff6f1

## Stop Conditions

- #524 lacks a reviewed merge
- Successor plan changes during review
- The complete denominator cannot be resolved
- The reviewer is asked to remediate its own findings

## Handoff

Proceed only after doctor readiness.
