# Structured Planning Prompt

Template: 1.0.0

Issue: 133

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add small authority-owned redacted snapshot views and monotonic revision guards to each owning module, prove the complete cross-module contract in one focused integration test, obtain exact-head review, and merge the closing PR.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Define bounded authority-owned row and snapshot APIs with redaction and revision guards.",
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
    "action": "Preserve current placement decisions and complete migration/recovery state across mutation and restore.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused cross-module negative and restart tests and run strict validation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain independent exact-head review, publish with closing linkage, shepherd CI, and merge.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- Rows are constructed from live authority state and cannot be self-attested
- Enumeration order and capacity behavior are deterministic
- Every authoritative state transition advances or restores the exact snapshot revision
- N/N+1 drift never returns mixed-revision data
- Redacted surfaces contain no private or raw evidence
- Issue #5877 owned files remain unchanged

## Risks

- An accessor could expose only active entries and silently omit unavailable authority rows
- Revision changes could miss removal or restore transitions
- Snapshot restoration could reproduce rows but not the revision guard
- Public row constructors could reintroduce self-attestation

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/133/design.md

Digest: eda9d46051222fd1b7afccace057fb9d23d595f646e49bbb19ef30bdf06b4199

## Diagram

.csdlc/prepared/issues/133/diagram.mmd

Digest: a21ea98622e9301d38922f6c7777b53f20e90cf391ba96d2ec251dce27c2ee9d

## Stop Conditions

- The design requires modifying issue #5877 owned files
- A complete bounded snapshot cannot be produced without weakening an existing authority invariant
- A public constructor or raw evidence exposure becomes necessary
- Tracked edits appear on primary main
- Exact-head review or CI reports an actionable failure

## Handoff

Proceed only after doctor readiness.
