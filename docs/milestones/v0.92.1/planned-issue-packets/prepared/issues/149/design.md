# CORP-U Design

Issue: #149

## Objective

Coordinate the Corporate and IP transfer lane without absorbing child implementation ownership.

## Scope

Dependency sequencing, status, serialization, evidence inventory, child handoffs, findings routing, and lane closeout for CORP-01, CORP-02, CORP-03, CORP-04, CORP-05, CORP-06, CORP-07, CORP-08.

## Dependencies

- No child dependency; setup issue #146 and umbrella readiness only

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Exact child dependency and status ledger
- Lane evidence inventory, finding route, and closeout synthesis

## Owned Paths

- `docs/milestones/v0.92.1/evidence/umbrellas/corp-u/**`
- `.csdlc/prepared/issues/149/**`
- `.csdlc/prepared/issues/149/validate-outcome.rb`
- `.csdlc/issues/149/**`
- `.csdlc/evidence/149/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. All 8 declared children have complete six-card packets and approved designs before start.
2. Every dependency and serialization gate is checked before child binding.
3. No child implementation path is changed by the umbrella.
4. The umbrella closes only after all non-deferred children reach their declared terminal state.

## PVF Lanes

- `corp-u-coordination`: Validate exact child denominator, readiness, dependency, terminal, and no-product-ownership truth. Command: `ruby .csdlc/prepared/issues/149/validate-outcome.rb`.

## Validation Proof

The issue-owned validator recomputes the exact eight-child denominator from the
milestone wave, validates each terminal child through typed C-SDLC, binds every
ledger row to the child index digest and merged revision, checks dependency
sequence numbers and producer-artifact SHA-256 digests, and rejects any diff
outside the umbrella coordination roots.

## Authority Boundary

- The umbrella may coordinate and synthesize but cannot modify child-owned product paths.
- Children retain exclusive implementation and review ownership.

## Non-goals

- Product implementation
- Bypassing child gates
- Treating coordination status as release proof

## Risks

- Umbrella scope could absorb child work
- A stale status could start a child early

## Stop Conditions

- A child lacks complete readiness
- A dependency or serialization gate is ambiguous
- Coordination would require a product-path edit

## Review Prompts

- Does the umbrella preserve child ownership?
- Are all child and dependency states live and exact?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml`
