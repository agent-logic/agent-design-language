# Characterization And Parity

## Metadata
- Feature Name: ADL normalized characterization corpus
- Milestone Target: `v0.91.8`
- Status: planned
- Owner: WP-03 and WP-11
- Doc Role: primary
- Supporting Docs: `../DEMO_MATRIX_v0.91.8.md`
- Feature Types: artifact, policy
- Proof Modes: tests, replay, review

## Template Rules

Corpus membership and normalization fields remain subject to WP-03 review.

## Purpose

Preserve declared behavior without porting the legacy internal test structure.

## Context

- Related milestone: `v0.91.8`
- Related issues: WP-03/WP-11 pending
- Dependencies: WP-02 contract inventory

## Coverage / Ownership

- Primary owner doc: this document.
- Covered surfaces: documents, schemas, plans, errors, execution, traces,
  artifacts, signing, resume, provider/tool outcomes.
- Related docs: design and shadow cutover feature.

## Overview

A compact corpus captures normalized v1 outcomes for positive and negative
cases. The independent implementation is compared against those outcomes and
every mismatch receives a typed disposition.

## Design

### Core Concepts

- Normalized outcome: semantically stable fields with irrelevant formatting removed.
- Mismatch disposition: v1 defect, v2 defect, intentional change, formatting,
  or unsupported compatibility.

### Architecture

- Inputs: selected examples, fixtures, captured external responses.
- Outputs: canonical outcome packets and parity report.
- Interfaces: versioned corpus manifest and normalizer schema.
- Invariants: no live nondeterminism in required parity; no silent mismatch.

### Data / Artifacts

- corpus manifest, normalized outcomes, mismatch register.

## Execution Flow

1. Select capability-covering cases.
2. Capture repeated v1 outcomes.
3. Compare v2 and disposition all differences.

## Determinism and Constraints

- Required cases run repeatedly with canonical digest equality.
- External inputs are fixtures or captured events.

## Integration Points

| System / Surface | Integration Type | Description |
|---|---|---|
| v1 ADL | observe | Black-box behavior source. |
| v2 ADL | observe | Candidate outcome source. |

## Validation

- Corpus coverage against WP-02 capability inventory.
- Repeated outcome equality and negative-case assertions.

## Acceptance Criteria

- All release-critical capabilities have cases.
- WP-11 has no unclassified mismatch.

## Risks

- Accidental bugs may look contractual; reviewer disposition is mandatory.

## Future Work

Retain the corpus as a compatibility suite after legacy deletion.

## Notes

Test count is a budget, not a coverage proxy.
