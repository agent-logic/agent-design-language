---
name: csdlc-v2-review
description: Assign and record exact-revision pre-publication review truth.
---
Generate or refresh the standard SRP at the complete substantive commit, then
send that SRP, the repository/worktree path, and the exact commit SHA to a fresh
external review session that does not inherit the implementation conversation.
The reviewer reports findings first with severity and file/line evidence and
must not edit the worktree or GitHub state.

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
