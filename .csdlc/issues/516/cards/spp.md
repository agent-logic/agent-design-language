# Structured Planning Prompt

Template: 1.0.0

Issue: 516

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Enumerate the complete expected baseline, collect exact observed evidence, run findings-first gap classification, route each material gap to an existing owner, and emit a fail-closed release-tail decision.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Build the canonical issue, dependency, acceptance, and execution-specification denominator.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Collect exact merged ancestry, implementation call-path, validation, review, documentation, integration, and closeout evidence for every denominator row.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Generate findings-first Markdown and JSON gap reports with severity, evidence, uncertainty, disposition, classification, and owner.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Resolve denominator collisions and verify every material gap is fixed, explicitly release-blocking, or routed to an existing owner.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Emit and validate the immutable release-tail admission decision without approving the release.",
    "acceptance_ids": [
      "AC-2",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Complete denominator
- Exact revision and ancestry evidence
- Production-path proof rather than string presence
- No unresolved P0/P1 admission
- No unowned material gap
- No child implementation in #516

## Risks

- Treating closed issues as proof of implemented behavior
- Missing inherited or retained dependency scope
- Admitting test-only or unused implementation
- Stale review or closeout evidence
- Duplicating already-routed remediation

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/516/design.md

Digest: 965ab7b077be961324d424363b09b7fa629838492dd37fd3dd3db5d1af00c839

## Diagram

.csdlc/prepared/issues/516/diagram.mmd

Digest: 5b96c9bd5e3067a32349ca90a59de550f0ae4677443fbe4957fc8e6aa0ffc5a0

## Stop Conditions

- The expected issue or acceptance denominator is incomplete
- Any required lane lacks reviewed merged ancestry
- Evidence revisions disagree
- A required behavior exists only as a stub, test fixture, unused path, or unsupported claim
- Any unresolved P0/P1 or unowned material gap remains

## Handoff

Proceed only after doctor readiness.
