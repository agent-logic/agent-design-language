# Structured Planning Prompt

Template: 1.0.0

Issue: 708

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add the smallest typed Runtime resource needed to validate and snapshot the welcome package, inject it during admission before the first model turn, retain exact per-agent delivery provenance, and expose that provenance in Runtime and Observatory surfaces.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Locate the existing configuration, admission, initial-context, agent-record, and Observatory projection seams; define the minimal typed orientation resource contract.",
    "acceptance_ids": [
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement fail-closed resource loading and hot reload with last-valid preservation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Snapshot and inject exact orientation content during admission before the first model request; retain version and digest on the agent record.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Expose per-agent orientation provenance through Runtime projections and render it in Observatory agent details.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run focused behavioral, reload, fail-closed, projection, and rendering validation; obtain independent review before publication.",
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
  }
]

## Invariants

- The source welcome-package document remains byte-for-byte unchanged
- No model turn occurs before orientation injection for a newly admitted agent
- Recorded provenance describes exact delivered bytes, not mutable global state
- Existing agents retain their actual delivery provenance across reload
- Invalid updates cannot replace the last valid active resource
- Orientation never grants authority

## Risks

- A digest of source rather than delivered bytes would create false provenance
- Late injection could allow a first model turn without orientation
- Global-only provenance could misreport existing agents after reload
- Observatory rendering could imply authority rather than orientation
- Scope could drift into a general prompt framework

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/708/design.md

Digest: 8ae95c9e3004c2770afe6e6b9b3ff8986881e9ecc1c8e2e70cb0cdb478a7f919

## Diagram

.csdlc/prepared/issues/708/diagram.mmd

Digest: d778bf9ccf2edbfcf47af8b19bc0105b0f0d7372b6f0e94fa95ecf3ab399aea8

## Stop Conditions

- Implementation would modify the canonical welcome-package source
- The admission path cannot guarantee injection before the first model request
- The selected projection is not deterministic and digest-bound
- Implementation requires a broad prompt framework outside #708

## Handoff

Proceed only after doctor readiness.
