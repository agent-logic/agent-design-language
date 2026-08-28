# Issue 517 Design — Quality gate

## Goal

Produce one quality-gate decision for the exact candidate admitted by #516.

## Required Outcome

Every required proving lane passes for the exact candidate and the gate reports zero unowned exceptions.

## Ownership

- `docs/milestones/v0.92.1/evidence/release/tail-01`
- `docs/milestones/v0.92.1/QUALITY_GATE_v0.92.1.md`
- `.csdlc/prepared/issues/517/validate-quality-gate.rb`

## Dependencies

- Terminal reviewed and ancestral INT-01 issue #516
- Sprint 9 umbrella #537

## Safety Boundary

- This issue owns only the listed result and paths.
- Missing, stale, skipped, non-proving, or ambiguous evidence fails closed.
- Validation and independent exact-head review precede publication.

## Non-Goals

- Documentation repair
- Release ceremony
- Implementing failed-lane fixes
