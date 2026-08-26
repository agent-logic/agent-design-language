# Structured Planning Prompt

Template: 1.0.0

Issue: 252

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Reproduce executable discovery under concurrency, correct the narrow shared resolution boundary, prove both regressions repeatedly plus the required Runtime lane and strict Clippy, then obtain fresh review and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Diagnose the shared executable discovery and invocation failure under hosted-like concurrency.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the smallest production or test-fixture correction without weakening SpawnFailed semantics.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run repeated focused proof, required Runtime local lane, and strict Clippy.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review, resolve findings, and publish a ready PR.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  }
]

## Invariants

- Executable selection is deterministic and repo-independent
- SpawnFailed continues to mean an actual process spawn failure
- Tests do not share mutable executable artifacts
- No unrelated Runtime authority changes

## Risks

- Cargo test binary discovery differs between macOS and hosted Linux
- Parallel tests may race over a shared fixture path
- A test-only workaround could conceal a production path defect

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/252/design.md

Digest: 618969e8355ffa07062849f5120390f7edf94bfea4a1d54cf91b4eb31edb3461

## Diagram

.csdlc/prepared/issues/252/diagram.mmd

Digest: 6c861470477b12cd4b89e38e26798ea7861d1877a7f7026115bfc50f7a6fd007

## Stop Conditions

- Correction requires broad Guardian lifecycle redesign
- Scope collides with another active issue
- Required proof remains nondeterministic
- Any edit is needed outside the declared Runtime boundary

## Handoff

Proceed only after doctor readiness.
