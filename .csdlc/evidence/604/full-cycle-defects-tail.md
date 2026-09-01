# Issue #604 full-cycle canary defects tail

## DEFECT-010: Review scope can make publication immediately stale

- Status: open workflow defect.
- Evidence: a pre-publication review whose scope included `.csdlc/issues/604/**`
  passed, but `csdlc-publish status` then rejected publication with
  `publication review guard failed: review_stale` because recording the review
  itself mutated the scoped lifecycle files.
- Impact: a correct-looking review can become unusable for publication unless
  the operator knows to exclude lifecycle metadata from the substantive review
  scope or provide explicit metadata-only proof.
- Required fix: the one-command lifecycle should derive a publication-valid
  review scope automatically and reject self-staling review scopes before they
  are recorded.

## DEFECT-011: ready/reconcile-ready did not publish metadata tail

- Status: fixed in #604.
- Evidence: the live `csdlc-publish ready` run successfully marked PR #610
  non-draft and recorded generation 9 locally, but left `.csdlc/issues/604/**`
  dirty instead of committing, pushing, and reobserving the metadata-only
  lifecycle tail the way `csdlc-publish publish` does.
- Impact: ready publication truth could remain local-only and diverge from the
  PR branch head.
- Fix: `ready` and `reconcile-ready` now commit the governed issue metadata
  tail, push the branch, reobserve the PR at the metadata head, and validate the
  metadata-only follow-up before returning success.

## DEFECT-012: Retained PR-state request examples use a stale action field

- Status: open compatibility/documentation defect.
- Evidence: copying the shape from an existing retained
  `.csdlc/prepared/issues/*/pr-state-request.json` failed with
  `unknown field action`; current `csdlc-github-pr state` expects the request
  body without `action`.
- Impact: operators following retained local examples can lose time on schema
  drift during PR readback.
- Required fix: update retained examples or provide a schema-guided request
  generator for PR-state readbacks.

## DEFECT-013: published resume path did not push an advanced local head

- Status: fixed in #604.
- Evidence: after committing evidence-only metadata at local head `31d8b99e`,
  `csdlc-publish publish` on an already-published record failed with
  `metadata publication PR did not converge to the exact governed follow-up
  head`; typed PR readback and `git ls-remote` still showed remote branch head
  `4a4da4b8`.
- Impact: a valid local metadata/evidence head could remain unpublished when
  the publish command entered its resume path with no new issue metadata tail.
- Fix: the resume path now pushes the current branch head before reobserving the
  PR at the expected metadata head.
