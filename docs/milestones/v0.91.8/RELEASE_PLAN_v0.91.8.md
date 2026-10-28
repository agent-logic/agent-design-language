# v0.91.8 Release Plan

## Release Posture

`v0.91.8` is ready for the WP-23 release ceremony. The implementation,
acceptance, quality, documentation, two internal reviews, external review,
remediation, v0.92 handoff, and next-milestone planning review are merged.
WP-23 `#5348` owns the final tag, GitHub release, and sprint-umbrella closure.

The release tag must point to the merge commit of the WP-23 PR. Release actions
must run through `adl/tools/release_ceremony.sh`; the script's GitHub release
path uses the repository-native Rust command surface.

## Completed Gates

1. Architecture and denominator approval.
2. Characterization and parity corpus acceptance.
3. ADL v2 implementation proof.
4. Runtime v3 adapter and deployment proof.
5. C-SDLC v2 lifecycle deployment proof and v1 command-surface sunset.
6. Rollback and reversible selector proof.
7. Deletion eligibility and post-deletion validation.
8. WP-14A acceptance and deployment.
9. Demo convergence and integrated quality gate.
10. WP-17 documentation and release-truth alignment.
11. WP-18 first internal milestone review.
12. WP-18 final second pass `#5791` after residual coding.
13. WP-19 independent external review.
14. WP-20 remediation and release preflight.
15. WP-21 exact-revision v0.92 handoff ledger.
16. WP-21A next-milestone closeout plan.
17. WP-22 next-milestone planning review, merged as `703ee31f2c02bb6c8fda7d6bc51ff7963075132e`.

## Ceremony Sequence

1. Merge the reviewed WP-23 PR with `Closes #5348` and `Closes #5809`.
2. At that exact merge commit, run the release script to create and push the
   annotated `v0.91.8` tag.
3. Use the release script to create and publish the GitHub release from
   `RELEASE_NOTES_v0.91.8.md`.
4. Verify the remote tag and published GitHub release resolve to the WP-23
   merge commit.
5. Close sprint umbrella `#5595` with the exact tag, release URL, and merge
   commit recorded in its final comment.

## Rollback

Rollback restores the previous generation selector and stable binary-path
state. The release does not rely on Cargo target directories or local build
cache state as operational truth.

## Non-Claims

- `v0.92` execution or birthday activation is not part of this release.
- Retained preparation packets are inputs, not proof of future implementation.
- The release claims only the merged and reviewed v0.91.8 surfaces named in
  the milestone evidence.
