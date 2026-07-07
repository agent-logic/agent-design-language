# ADR 0047: Repo Binaries And Warm-Cache Validation Boundary

- Status: Accepted
- Date: 2026-07-06
- Accepted in: v0.91.7
- Related issues: #4726, #4806, #4837, #4838, #4879, #4989
- Related ADRs: ADR 0036, ADR 0043, ADR 0045
- Source evidence:
  - `docs/milestones/v0.91.7/review/V0917_WP06_BUILD_THROUGHPUT_VALIDATION_COST_REDUCTION_4633.md`
  - `docs/milestones/v0.91.7/review/build_throughput/ADL_BUILDER_IMAGE_4879.md`
  - `docs/milestones/v0.91.7/review/build_throughput/AWS_SPOT_REMOTE_VALIDATION_LANE_4837.md`
  - `docs/milestones/v0.91.7/review/build_throughput/AWS_CODEFRIEND_BUILD_LANE_4838.md`

## Context

Cold Rust builds and owner-binary rebuilds were consuming issue time and making
docs/tooling work feel nondeterministic.

## Decision

ADL should prefer repo-owned binaries and trusted warm dependency caches for
workflow command execution. Cache warmup is acceleration evidence only; it does
not replace validation proof. Remote build lanes may provide capacity, but they
must report cost, platform, cache, and proof status explicitly.

## Consequences

- Faster local workflow is possible without weakening validation truth.
- Binary decomposition must preserve command ownership and compatibility.
- Remote validation lanes need explicit cost/resource records.

## Alternatives Considered

### Rebuild through Cargo for every command

Rejected for normal workflow use because it turns control-plane commands into
unnecessary build jobs.

## Validation Notes

Check warm-cache wrapper behavior, remote-builder packets, and validation lane
classification when command or build surfaces change.

## Non-Claims

- This ADR does not claim cache warmup proves correctness.
- This ADR does not require always-on cloud builders.
