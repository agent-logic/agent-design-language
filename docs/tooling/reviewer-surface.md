# Reviewer Surface

This document describes the bounded reviewer entry surfaces that make ADL runtime and card-review artifacts inspectable without reconstructing proof roots by hand.

## Purpose

The reviewer surface is a deterministic helper, not a full replacement for richer GPT-based review.

It exists to provide:

- a concrete repo-local review/helper surface
- a fixed subset of checklist-aware review behavior
- a stable fixture and deterministic review run
- one bounded runtime reviewer entrypoint for `v0.87.1`

## Current Entry Points

Use:

```bash
adl tooling review-card-surface \
  --input docs/tooling/examples/reviewer-regression/issue-661/input_661.md \
  --output docs/tooling/examples/reviewer-regression/issue-661/output_661.md
```

The v0.87.1 runtime walkthrough is [historically retired](V0871_HISTORICAL_AGGREGATE.md).
Its D8 assembler and private `review-runtime-surface` recipe are no longer ADL
execution routes; their original assertions and artifact format remain in custody.

For the broader milestone reviewer package, pair that walkthrough with:

- `docs/milestones/v0.87.1/FEATURE_DOCS_v0.87.1.md`
- `docs/milestones/v0.87.1/DEMO_MATRIX_v0.87.1.md`
- `docs/milestones/v0.87.1/README.md`

## Card Review Checks

The first slice checks:

- required output-card sections
- normalized output-card status
- non-blank execution metadata
- explicit repo-relative artifact paths
- absence of absolute host paths in the output card
- canonical Prompt Spec review-surface ordering when present

## Runtime Review Checks

The `v0.87.1` runtime review surface checks:

- one canonical D8 manifest and README exist
- manifest identity fields are normalized
- manifest paths are explicit and repo-relative
- the bounded D6 and D7 package entry ordering is stable
- referenced runtime proof surfaces exist
- the review package contains no absolute host paths

## Deterministic Fixtures

Use:

```bash
bash adl/tools/test_review_card_surface.sh
```

This fixture run compares the tool output against:

- `docs/tooling/examples/reviewer-surface/issue-661/expected_review_surface_output_661.yaml`

The former runtime walkthrough fixture is retired with its assembler. Its checks
for D8 manifest identity, D6/D7 ordering, README guidance and private CLI validation
are preserved as historical obligations, not counted as a current passing test.

## Scope Notes

This is intentionally narrower than the full reviewer GPT instructions and checklist ecosystem.

It is meant to:

- make review behavior more operational than prose-only guidance
- create a bounded stable proof surface
- support later integration with richer editor and lifecycle tooling
