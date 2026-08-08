# Structured Planning Prompt

Template: 1.0.0

Issue: 5881

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Rebase after the two narrow deletion issues, classify claim surfaces, normalize current records once, delete all claim-specific production logic, and prove crash-safe binding plus the full lifecycle from branch/worktree topology.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Rebase after #5895 and #5883 and classify all claim-related occurrences as active authority, temporary current-record normalization, historical evidence, or unrelated language.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Delete residual active claim fields, operations, gates, schemas, fixtures, skills, and current operator guidance without adding compatibility routes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Normalize current claim-bearing records through a verified one-time path, preserve topology and audit truth, then delete the claim-specific decoder and structs.",
    "acceptance_ids": [
      "AC-3",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused atomic, idempotent, concurrent, and interrupted-bind validation plus claim-free review, publication, finish, and cleanup proof.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- One authoritative branch/worktree pair per issue
- No canonical claim state
- Historical evidence immutable
- Exact-head review and terminal authority unchanged
- Focused validation only

## Risks

- Deleting temporary normalization support before current claim-bearing legacy-format records normalize
- Failing to delete claim-specific production logic after normalization
- Treating ordinary evidence claims as lifecycle claims
- Overlapping #5895 or #5883 operator edits
- Leaving an installed skill or schema route stale

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5881/design.md

Digest: 44e67b37265d9143069bf9db75750c7d3c646ed23de6082ee33f43f73a1f90ff

## Diagram

.csdlc/prepared/issues/5881/diagram.mmd

Digest: 965cc168665ed591f0024bcdb62c16a08582c20900becd4b90b4d7c6dbbff4de

## Stop Conditions

- A proposed change weakens exact-head review or terminal authority
- Current claim-bearing legacy-format records cannot be normalized without destructive rewriting
- The issue would require a new compatibility wrapper
- Shared #5895 or #5883 changes have not settled

## Handoff

Proceed only after doctor readiness.
