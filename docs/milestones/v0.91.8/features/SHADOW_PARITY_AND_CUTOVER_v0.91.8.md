# Shadow Parity And Cutover

## Metadata
- Feature Name: ADL v2 parity, soak, selector, rollback, and deletion
- Milestone Target: `v0.91.8`
- Status: planned
- Owner: WP-11 through WP-13
- Doc Role: primary
- Supporting Docs: `../QUALITY_GATE_v0.91.8.md`
- Feature Types: policy, architecture, artifact
- Proof Modes: demo, tests, replay, review

## Template Rules

No default or deletion claim precedes exact-revision evidence.

## Purpose

Replace v1 safely, demonstrate restoration, and make source reduction a
verified terminal outcome.

## Context

- Related milestone: `v0.91.8`
- Related issues: WP-11 through WP-13 pending
- Dependencies: complete candidate product and characterization corpus

## Coverage / Ownership

- Primary owner doc: this document.
- Covered surfaces: parity, mismatch disposition, soak, selector, rollback,
  compatibility expiry, owner-band and final deletion.
- Related docs: demo matrix and release plan.

## Overview

The candidate begins opt-in, runs normalized shadow parity and representative
soak, proves rollback, becomes the reviewed default, and only then authorizes
legacy deletion.

## Design

### Core Concepts

- Authoritative generation selector.
- Approval-bound deletion manifest.

### Architecture

- Inputs: exact v1/v2 revisions, corpus, scenario manifest, approvals.
- Outputs: parity, soak, rollback, switch, deletion evidence.
- Interfaces: selector JSON and owner proof binaries.
- Invariants: v1 remains available through rollback; deletion is separately approved.

### Data / Artifacts

- parity matrix, soak packet, selector transaction, deletion manifest.

## Execution Flow

1. Compare and classify all normalized outcomes.
2. Run opt-in scenarios and restore v1.
3. Switch default after review; observe rollback window.
4. Delete owner bands and then final compatibility code.

## Determinism and Constraints

- Selector and deletion decisions bind exact digests and revisions.
- Deletion is at least 80%; 90% is the target.

## Integration Points

| System / Surface | Integration Type | Description |
|---|---|---|
| Installer/operator | write | Installs and selects generations. |
| v0.92 | observe | Consumes only final reviewed contracts. |

## Validation

- Full parity, representative soak, explicit restore, fresh install, deletion recount.

## Acceptance Criteria

- No unclassified mismatch.
- Rollback works before default switch.
- Final deletion is at least 80% with reviewed retained manifest.

## Risks

- Schedule pressure may shorten rollback; any change requires operator review.

## Future Work

Remove the read-only importer after its reviewed expiry.

## Notes

Cutover and deletion are separate authority decisions.
