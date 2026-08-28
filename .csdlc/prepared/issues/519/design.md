# Issue 519 Design — Publication finalization

## Goal

Produce one exact-revision publication-candidate packet without performing publication or release mutation.

## Required Outcome

The packet binds the exact reviewed candidate, correct closing relationships, publication linkage, and redacted artifacts while leaving merge, tag, release, and external publication untouched.

## Ownership

- `docs/milestones/v0.92.1/evidence/release/tail-03`
- `.csdlc/prepared/issues/519/validate-publication-candidate.rb`

## Dependencies

- Terminal reviewed and ancestral TAIL-02 issue #518
- Sprint 9 umbrella #537

## Safety Boundary

- This issue owns only the listed result and paths.
- Missing, stale, skipped, non-proving, or ambiguous evidence fails closed.
- Validation and independent exact-head review precede publication.

## Non-Goals

- Merge
- Tag
- Release
- External publication
- Release ceremony
