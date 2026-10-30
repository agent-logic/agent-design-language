# ADR 0045: Validation Manager And Fast/Slow Proof Boundary

- Status: Accepted
- Date: 2026-07-06
- Accepted in: v0.91.7
- Related issues: #4676, #4678, #4679, #4806, #4989
- Related ADRs: ADR 0032, ADR 0036
- Source evidence:
  - `docs/milestones/v0.91.7/review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md`
  - `docs/milestones/v0.91.7/review/build_throughput/NESSUS_VALIDATION_MANAGER_LANE_4678.md`
  - `docs/milestones/v0.91.7/review/build_throughput/REMOTE_BUILDER_OPERATIONAL_PROOF_4679.md`

## Context

The test surface grew large enough that running broad validation for every
issue became a project blocker. v0.91.7 added validation-manager lanes, remote
build options, and path-sensitive validation routing.

## Decision

ADL should separate fast local proof, slow proof, remote proof, release-gate
proof, and docs-only proof as first-class validation lane categories. A PR's
validation plan should choose the minimum proving lane for the touched surface,
while preserving slow/release proof as explicit deferred or required gates.

## Consequences

- Ordinary changes can finish without paying every slow-test cost.
- Slow proof remains visible instead of disappearing into skipped checks.
- New tests and tools must be PVF-classifiable when authored.

## Alternatives Considered

### Always run the full workspace

Rejected. It creates unnecessary wait states and hides path-specific proof.

## Validation Notes

Future work should inspect validation-manager lane output, PVF/VPP cards, and
CI lane classifications together.

## Non-Claims

- This ADR does not certify every lane selector rule as complete.
- This ADR does not remove release-gate proof where required.
