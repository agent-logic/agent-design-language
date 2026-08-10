# INT-U Design

Issue: #152

## Objective

Coordinate the Integrated review and release lane without absorbing child implementation ownership.

## Scope

Dependency sequencing, status, serialization, evidence inventory, child handoffs, findings routing, and lane closeout for INT-01, INT-02, INT-03.

## Dependencies

- No child dependency; setup issue #146 and umbrella readiness only

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Exact child dependency and status ledger
- Lane evidence inventory, finding route, and closeout synthesis

## Owned Paths

- `docs/milestones/v0.92.1/evidence/umbrellas/int-u/**`
- `.csdlc/prepared/issues/152/**`
- `.csdlc/prepared/issues/152/validate-outcome.rb`
- `.csdlc/issues/152/**`
- `.csdlc/evidence/152/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. All 3 declared children have complete six-card packets and approved designs before start.
2. Every dependency and serialization gate is checked before child binding.
3. No child implementation path is changed by the umbrella.
4. The umbrella closes only after all non-deferred children reach their declared terminal state.

## PVF Lanes

- `int-u-coordination`: Validate exact child denominator, readiness, dependency, terminal, and no-product-ownership truth. Command: `ruby .csdlc/prepared/issues/152/validate-outcome.rb`.

## Validation Proof

The issue-owned validator verifies terminal CORP-08, V3-16, and DRT-07 gates,
recomputes the three integration child records and producer-artifact digests,
enforces finding disposition before release-candidate work, checks exact Git
ancestry, and rejects any umbrella change outside coordination evidence.

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
