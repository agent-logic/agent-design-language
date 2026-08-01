# v0.91.8 Release Plan

## Release Posture

`v0.91.8` is not released by this documentation package. WP-16 merged at
`2e9d2dd7c4260dcf6ec6af954b0eea97554212df` and records an integrated platform
quality-gate pass, but final release still requires WP-17 documentation
alignment, WP-18 internal review, formal review, remediation, next-milestone
handoff/review, and release ceremony work.

## Gates

1. Architecture and denominator approval.
2. Characterization and parity corpus acceptance.
3. ADL v2 implementation proof.
4. Runtime v3 adapter and deployment proof.
5. C-SDLC v2 lifecycle deployment proof.
6. Rollback and reversible selector proof.
7. Deletion eligibility and post-deletion validation.
8. WP-14A acceptance and deployment.
9. Demo and integrated quality gate. WP-16 is merged at `2e9d2dd7c` with 67
   audited issues, 0 unacceptable outcomes, 0 release blockers, and focused,
   integrated, and complete lanes passing.
10. WP-17 documentation and release-truth alignment.
11. WP-18 internal review, formal milestone review, remediation, and preflight.
12. WP-21 exact-revision handoff and release ceremony closeout.

The release-tail review sequence must preserve WP-17 documentation alignment,
WP-18 internal review, formal review/remediation, WP-21 next-milestone handoff,
WP-21A handoff/review alignment, WP-22 review, and release ceremony. Current
v0.91.7 WP-21A `#5489` is historical preparation evidence and does not execute
v0.91.8 work.

The release plan must consume current blocker/non-claim truth explicitly:
`#5408` is closed/remediated via PR #5419, while #4906 remains retained
blocked-with-evidence unless separately dispositioned.

## Rollback

Rollback must restore the previous generation selector and stable binary path
state. The release cannot rely on Cargo target directories or local build cache
state as operational truth.

## Current Non-Claims

- Final `v0.91.8` release approval is not claimed.
- WP-18 internal review and formal third-party milestone review are not claimed
  complete.
- v0.92 birthday activation is not claimed.
- Partial or ambiguous release-tail, umbrella, and lifecycle-drift items
  recorded by WP-16 remain explicit limitations unless later evidence closes
  them.
