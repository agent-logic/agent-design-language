# Baseline And Architecture

## Metadata
- Feature Name: ADL v2 baseline and clean-room architecture
- Milestone Target: `v0.91.8`
- Status: planned
- Owner: WP-02
- Doc Role: primary
- Supporting Docs: `../DESIGN_v0.91.8.md`
- Feature Types: architecture, policy
- Proof Modes: schema, review

## Template Rules

All sections are completed for planning; measurements remain pending WP-02.

## Purpose

Pin what is being replaced and approve a product boundary capable of real deletion.

## Context

- Related milestone: `v0.91.8`
- Related issues: `#5335`, WP-02 pending
- Dependencies: current `main`, Runtime v3 and C-SDLC v2 evidence

## Coverage / Ownership

- Primary owner doc: this document.
- Covered surfaces: denominator, owner map, dependency/LoC budgets, architecture.
- Related docs: quality gate and decision log.

## Overview

WP-02 inventories every incumbent Rust/script surface, separates implementation
from tests/generated output, hashes the selected denominator, and assigns every
retained capability to exactly one product owner.

## Design

### Core Concepts

- Pinned denominator: immutable baseline for deletion accounting.
- Capability closure: every legacy module is represented in parity, deletion,
  or an explicit retained decision.

### Architecture

- Inputs: tracked source, manifests, binaries, schemas, examples, fixtures.
- Outputs: baseline manifests, diagrams, budgets, owner matrix.
- Interfaces: machine-readable JSON plus reviewer-facing Markdown.
- Invariants: no double counting; code movement cannot improve deletion.

### Data / Artifacts

- `adl_v1_baseline_modules.v1.json`
- `adl_v2_architecture_and_budgets.v1.json`

## Execution Flow

1. Pin revision and deterministic file lists.
2. Classify modules, tests, scripts, binaries, contracts, and dependencies.
3. Review architecture, budgets, and deletion denominator.

## Determinism and Constraints

- Identical revision and commands produce identical lists and hashes.
- Unknown ownership blocks construction.

## Integration Points

| System / Surface | Integration Type | Description |
|---|---|---|
| Runtime v3 | observe | Confirm runtime-owned capabilities. |
| C-SDLC v2 | observe | Confirm lifecycle-owned capabilities. |

## Validation

- Re-run counts and hashes.
- Verify module-to-capability closure and dependency graph.
- Run bounded architecture review.

## Acceptance Criteria

- Exact denominator and budgets are approved.
- Every incumbent file has one disposition.

## Risks

- Active main may drift; pin and report later additions separately.

## Future Work

Update only through a versioned baseline amendment; never rewrite history.

## Notes

Observed planning estimate is about 355,675 Rust LoC; WP-02 is authoritative.
