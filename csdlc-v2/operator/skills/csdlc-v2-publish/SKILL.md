---
name: csdlc-v2-publish
description: Publish only after current pre-publication review truth.
---
Invoke `csdlc-publish publish` with `draft: false` for the routine path; it creates and records one exact ready PR directly. Existing governed draft publications may still use the bounded ready reconciliation commands, but new routine work must not create a draft first. Keep the one-shot request at the Git-common path `.git/csdlc-v2/requests/<issue>.json` and overwrite it. Do not publish on missing/stale review, ambiguous remote state, or prose-only approval.

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

## C-SDLC v3 transition boundary

C-SDLC v3 is construction evidence only until an explicit operator-reviewed
V3-F cutover changes root authority. Continue using this v2 publication route
for live PR creation and ready reconciliation. V3 publication or adapter
construction surfaces must not publish, relink, or mark PR readiness before
cutover.
