# PR #865 review remediation

## Source review

The operator reviewed issue #864 / PR #865 at
`3fc16781e3779bca99866693aa37169d385a0233` and returned **FAIL — changes
required**. This supersedes the earlier independent PASS at that revision.

- **P1:** OBS-S3 and ARCH-ADR were required in prose but could be bypassed by
  TAIL-10. The execution graph and acceptance contract lacked final completion
  gates, and the validator prohibited release-tail dependencies too broadly.
- **P2:** the committed SRP still described exact-head review as pending while
  the PR and local typed receipt recorded a completed review.

The operator reported that planning self-tests (75 negative fixtures), atomic
validation, native six-card validation, link checks, diff hygiene and GitHub CI
passed at that revision. Those passes did not establish either missing
obligation.

## Correction boundary

Enforce both required tasks at final milestone closure, preserve the earlier
integration and TAIL-01 boundary, add negative fixtures for missing completion
edges and acceptance obligations, and reconcile affected planning projections.
Normalize review history and dispositions through native v3 card editing.

The tracked SRP names the revision actually reviewed. A later card-recording
commit is independently checked at its own exact head; the native review and
publication inputs under the resolved Git `csdlc-v3/reviews/864` directory bind
that final head. A commit cannot embed its own hash, so no self-referential
review SHA is invented in the tracked record.

No child creation, merge, milestone closure, or product execution is part of
this correction.
