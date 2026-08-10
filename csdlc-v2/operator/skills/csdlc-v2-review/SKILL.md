---
name: csdlc-v2-review
description: Assign and record exact-revision pre-publication review truth.
---
Generate or refresh the standard SRP at the complete substantive commit, then
send that SRP, the repository/worktree path, and the exact commit SHA to a fresh
external review session that does not inherit the implementation conversation.
Follow `docs/tooling/INDEPENDENT_EXACT_HEAD_REVIEW.md`; it is mandatory review
procedure, not optional background. Classify authority before assigning review
depth. Authentication, authorization, security boundaries, lifecycle authority,
and proof-producing changes require code, security, and evidence coverage even
when every changed file is documentation. Other documentation-only work uses a
documentation reviewer; other code requires code and test coverage.

The standard SRP must ask the reviewer to check every acceptance criterion and
report findings first, ordered P0 through P3, with repository-relative file and
line evidence. The reviewer must state explicit limitations and operate
read-only: it must not edit the worktree, lifecycle state, PR state, or GitHub
state. PASS is allowed only when no actionable finding remains.

Invoke `csdlc-review record` with evidence naming that reviewer, exact scope,
and exact clean scoped revision. This updates the standard SRP, which remains
the sole review-result authority. Resolve actionable findings in the
implementation session. Any substantive fix requires a refreshed SRP and a new
fresh-session exact-head review before publication.

A passing record atomically advances to `Reviewed`; routine review does not
require `assign`. Existing assignment records remain valid compatibility
evidence. If reviewed work becomes stale before publication, use typed
`recover` to return it to `implemented`, preserving the audit trail before
re-review. Keep the one-shot request at the Git-common path
`.git/csdlc-v2/requests/<issue>.json` and overwrite it.
