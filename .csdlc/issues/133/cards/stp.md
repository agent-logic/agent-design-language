# Structured Task Prompt

Template: 1.0.0

Issue: 133

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement, prove, independently review, publish, shepherd, and merge the issue #132 authority snapshot accessors without implementing issue #5877 projections.

## Deliverables

- Revisioned redacted certificate authority snapshot
- Revisioned redacted failure-detection authority snapshot with missing members
- Revisioned redacted complete lease authority snapshot
- Revisioned redacted complete fencing-floor authority snapshot
- Revisioned redacted current placement-decision snapshot
- Revisioned redacted migration and recovery snapshots with restart parity
- adl-runtime/tests/distributed_authority_snapshots.rs

## Acceptance

1. AC-1: Every accessor returns complete deterministic bounded rows and an authority-owned revision/checkpoint without public row constructors.
2. AC-2: Authoritative mutations advance the revision, and an N/N+1 guarded read fails closed on drift.
3. AC-3: Certificate rows expose only redacted generation/status metadata and correctly represent current-generation overlap.
4. AC-4: Failure-detection rows explicitly represent unavailable or missing members without exposing raw probe evidence.
5. AC-5: Placement snapshots retain current decisions and correctly reflect replacement and removal.
6. AC-6: Migration and recovery snapshots enumerate complete state and reproduce the same rows and revision after snapshot restore.
7. AC-7: Focused tests and strict Clippy pass, and exact-head review has no actionable findings.

## Dependencies

- agent-logic/agent-design-language#5877
- Merged distributed guardian authority implementations on origin/main

## Inputs

- AGENTS.md
- docs/architecture/runtime-v3/DISTRIBUTED_GUARDIAN_ARCHITECTURE.md
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/failure_detection.rs
- adl-runtime/src/distributed/placement.rs
- adl-runtime/src/distributed/migration.rs
- adl-runtime/src/distributed/recovery.rs

## Non Goals

- Editing issue #5877 projection files
- Adding caller-created snapshot or proof constructors
- Exposing secret or raw authority evidence
- Changing distributed module registration or Cargo manifests unless unavoidable for compilation
- Post-merge closeout bookkeeping
