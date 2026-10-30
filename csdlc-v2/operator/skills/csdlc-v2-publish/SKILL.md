---
name: csdlc-v2-publish
description: Publish only after current pre-publication review truth.
---
C-SDLC v2 remains the live lifecycle authority only until explicit V3-F/#505
cutover. Before that cutover, `csdlc-v3/**` is non-authoritative construction
evidence and must not publish PRs, mutate GitHub, finish issues, or retire v2.

Invoke `csdlc-publish publish` with `draft: false` for the routine path; it creates and records one exact ready PR directly. Existing governed draft publications may use `csdlc-publish ready --request <json>` to mark the exact observed draft PR ready, or `csdlc-publish reconcile-ready --request <json>` after an uncertain ready mutation when live readback already proves the exact open non-draft PR at the expected head. New routine work must not create a draft first unless the issue explicitly tests draft reconciliation. Keep the one-shot request at the Git-common path `.git/csdlc-v2/requests/<issue>.json` and overwrite it. Do not publish on missing/stale review, ambiguous remote state, or prose-only approval.

`repository` is always the issue-tracker repository recorded by the issue. Set
the optional `code_repository` only when the PR belongs to a different
canonical code repository. Split-authority requests must use a qualified
closing reference such as
`Closes danielbaustin/agent-design-language#5844`; `Closes #5844` is rejected.
The requested `code_repository` must match the explicit identity already
recorded by typed bind; do not substitute a different repository at
publication time.
The selected Git remote's complete effective fetch and push URL sets, the PR
base and head repositories, the branch, and every matching open PR page must
all resolve unambiguously to `code_repository` before publication can mutate
remote state. Omitting `code_repository` preserves the same-repository path.
