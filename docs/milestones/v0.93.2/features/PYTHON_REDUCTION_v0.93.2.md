# v0.93.2 Bounded Python reduction

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Planning issue: #922. Accountable role: milestone owner; named execution owners remain unassigned. Status: planned.

## Template Rules

All sections declare planning intent; implementation proof is pending.

## Purpose

Remove only the selected post-split Python dependency tranche, preserving user-facing behavior and portable build contracts.

## Context

Accepted v0.93.1 handoff and pinned repository lockset are prerequisites: external references v0.93.1/RD-11, v0.93.1/RD-09 and v0.93.1/TAIL-10. WP-01 also requires explicit opening authorization. The repository split is consumed, not repeated.

## Coverage / Ownership

PY-01; repository and acceptance are in the canonical execution graph.

## Overview

Remove only the selected post-split Python dependency tranche, preserving user-facing behavior and portable build contracts.

## Design

Consume accepted producer contracts and preserve the launched CodeFriend baseline.

## Execution Flow

Accept prerequisites, prepare bounded fixtures, execute installed checks, retain evidence and resolve findings.

## Determinism and Constraints

Prefer deterministic local fixtures; live providers and private data require explicit scope and authorization.

## Integration Points

INTEGRATE consumes the results; QUALIFY independently checks the release lockset.

## Validation

Inventory exact selected commands and consumers; prove positive/negative parity with the replacement and check installed entrypoints. No wholesale rewrite is implied.

## Acceptance Criteria

Inventory exact selected commands and consumers; prove positive/negative parity with the replacement and check installed entrypoints. No wholesale rewrite is implied. No skipped or zero-case run counts as success.

## Risks

Stale producer versions or source-only checks can misrepresent installed behavior; pin versions and test consumers.

## Future Work

Scope expansion needs a separately reviewed issue; no unbounded rewrite or deployment is authorized.

## Notes

Prepared under #922; see the milestone quality gate.
