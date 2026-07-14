# v0.91.8 Decisions

## Metadata
- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Date: `2026-07-14`
- Owner: ADL maintainers

## Purpose

Record the architecture, budget, migration, and deletion decisions that bind
the milestone.

## How To Use

Changes require an issue, rationale, and impact on parity/deletion truth.

## Decision Log

| ID | Decision | Status | Rationale | Alternatives | Impact | Link |
|---|---|---|---|---|---|---|
| D-01 | Create v0.91.8 as an ADL-core bridge before v0.92 consumption. | accepted for planning | v0.92 already owns birthday semantics. | Absorb into v0.92. | Preserves milestone boundaries. | #5335 |
| D-02 | Build the replacement clean-room from behavioral contracts. | accepted for planning | Runtime v3 and C-SDLC v2 proved this method. | In-place refactor. | Enables a measurable deletion denominator. | #5335 |
| D-03 | Limit the product to language, compiler, engine, and thin CLI/adapters. | accepted for planning | These are the minimum coherent ADL responsibilities. | Retain monolithic exports. | Makes ownership enforceable. | DESIGN_v0.91.8.md |
| D-04 | Target 90% deletion; fail below 80%. | accepted for planning | Reduction is a product outcome, not optional cleanup. | Unbounded best effort. | Requires pinned denominator and retained manifest. | QUALITY_GATE_v0.91.8.md |
| D-05 | Compare normalized outcomes, not internal layouts. | accepted for planning | Independent implementations should not reproduce legacy internals. | Byte-identical internals. | Requires a normalization contract. | features/SHADOW_PARITY_AND_CUTOVER_v0.91.8.md |
| D-06 | Runtime v3 and C-SDLC v2 remain independent owners. | accepted for planning | Recombining them would reverse the successful rearchitecture. | Shared monolith. | Default ADL cannot link their full graphs. | DESIGN_v0.91.8.md |
| D-07 | Default switching must be reversible before deletion. | accepted for planning | Cutover risk must be bounded. | Flag day replacement. | Adds selector, soak, and rollback gates. | WP_ISSUE_WAVE_v0.91.8.yaml |
| D-08 | Code movement does not count toward deletion. | accepted for planning | Otherwise the metric rewards relabeling. | Count path movement. | Deletion report tracks actual incumbent removal and new-product size separately. | QUALITY_GATE_v0.91.8.md |

## Open Questions

- Should the three core layers be separate crates or enforced modules in one crate? Owner: architecture issue.
- Which legacy artifact formats require byte compatibility rather than normalized compatibility? Owner: characterization issue.
- What is the shortest safe rollback window after default cutover? Owner: cutover issue.

## Exit Criteria

- Every milestone-critical decision has a final disposition before cutover.
- Deferred questions have explicit owners and cannot silently block deletion.
