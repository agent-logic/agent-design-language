# Issue 516 Design — Release-tail admission

## Goal

Produce one immutable release-tail admission decision for the converged milestone candidate.

## Required Outcome

The admission record indexes every exact reviewed-green ancestral root, its artifacts, and a zero-unresolved-collision result.

## Ownership

- `docs/milestones/v0.92.1/evidence/integration`
- `docs/milestones/v0.92.1/DEMO_MATRIX_v0.92.1.md`
- `.csdlc/prepared/issues/516/validate-release-tail-admission.rb`

## Dependencies

- Terminal #498
- Terminal #496
- Terminal #494
- Terminal #495
- Terminal #499
- Terminal #505
- Terminal #508
- Terminal #509
- Terminal #51
- Terminal #510
- Terminal #512
- Terminal #513
- Terminal #515
- Sprint 9 umbrella #537

## Safety Boundary

- This issue owns only the listed result and paths.
- Missing, stale, skipped, non-proving, or ambiguous evidence fails closed.
- Validation and independent exact-head review precede publication.

## Non-Goals

- Implementing child fixes
- Release approval
- Tagging, releasing, or publishing
