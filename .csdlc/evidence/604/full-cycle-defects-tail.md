# Issue #604 full-cycle canary defects tail

## DEFECT-010: Review scope can make publication immediately stale

- Status: fixed in #604.
- Evidence: a pre-publication review whose scope included `.csdlc/issues/604/**`
  passed, but `csdlc-publish status` then rejected publication with
  `publication review guard failed: review_stale` because recording the review
  itself mutated the scoped lifecycle files.
- Impact: a correct-looking review can become unusable for publication unless
  the operator knows to exclude lifecycle metadata from the substantive review
  scope or provide explicit metadata-only proof.
- Fix: `csdlc-review assign` now rejects self-staling scopes that include the
  issue's generated `.csdlc/issues/<issue>` lifecycle record before recording an
  assignment, with a focused regression that confirms no partial lifecycle
  mutation remains after rejection.

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

- Status: fixed in #604.
- Evidence: copying the shape from an existing retained
  `.csdlc/prepared/issues/*/pr-state-request.json` failed with
  `unknown field action`; current `csdlc-github-pr state` expects the request
  body without `action`.
- Impact: operators following retained local examples can lose time on schema
  drift during PR readback.
- Fix: direct PR-state decoding now accepts both the current narrow request
  shape and the retained `pr_state` action-envelope examples, so existing
  evidence files remain executable while new requests can use the simpler
  schema.

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

## DEFECT-014: retained terminal-authority guard overmatched publication-ready reconciliation

- Status: fixed in #604.
- Evidence: full local `cargo test --locked --manifest-path csdlc-v2/Cargo.toml`
  reproduced the GitHub `csdlc-v2-standalone` failure in
  `gate_terminal_authority_deletion`: the retained terminal-authority deletion
  guard rejected the new `ReconcileReady` publication action solely because its
  name matched a retired terminal-reconciliation pattern.
- Impact: the full standalone lane failed even though `ReconcileReady` is a
  publication-readiness route and does not expose terminal/finish authority.
- Fix: the retained terminal-authority test now keeps terminal writer names on
  the denylist while explicitly allowing `ReconcileReady` as publication
  readiness, not closeout authority.

## DEFECT-015: retained terminal prerequisite digests drifted from live receipts

- Status: fixed in #604.
- Evidence: after DEFECT-014 was fixed, the full local standalone suite reached
  `projection_recovery_integration` and failed because the hard-coded retained
  terminal digests for #298 and #299 no longer matched the live
  `.git/csdlc-v2/derived-terminal/*.json` receipts. The merge SHAs and
  ancestry checks were still current.
- Impact: the full v2 suite could remain red even though the retained terminal
  receipts were present and internally consistent, masking real canary defects
  behind stale fixture constants.
- Fix: the retained-prerequisite constants now match the current live terminal
  receipts for #298 and #299 while preserving the existing merge-SHA and
  ancestry assertions.

## DEFECT-016: review evidence was recorded before a visible reviewer PASS

- Status: fixed in #604 lifecycle evidence.
- Evidence: while waiting on a slow final review subagent, the branch advanced
  through review and publication using `/root/review_604_terminal_digest_refresh`
  as the recorded reviewer even though the orchestrating session had not
  received that reviewer's final PASS. A separate tiny reviewer later returned
  an explicit PASS for the final digest-refresh surface.
- Impact: the PR could appear reviewed and published while the operator-visible
  review transcript did not justify the recorded reviewer identity.
- Fix: the issue is recovered and republished with review evidence tied to the
  reviewer that actually returned a visible PASS in this session.
