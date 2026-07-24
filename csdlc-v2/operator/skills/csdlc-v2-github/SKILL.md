---
name: csdlc-v2-github
description: Execute repo-native C-SDLC v2 GitHub issue and PR actions through the typed Rust command surface.
---

Invoke `csdlc-github run --request <request.json>` with a typed
`github_action_request` payload. Do not use the GitHub connector, raw `gh`,
legacy wrappers, shell/Python lifecycle mutation, or AWS. Every issue/comment
mutation must carry an `operation_key`; the command renders it as a stable
marker, searches/readbacks remote state, and fails closed on missing,
duplicated, or mismatched reconciliation.

Supported action values:

- `issue_create`
- `issue_update`
- `issue_comment`
- `issue_close`
- `issue_read`
- `pr_state`

`pr_state` is read-only readiness observation. PR publication, draft-to-ready,
merge, readiness recording, and terminal closeout remain under the existing
repo-native Rust v2 command surface: `csdlc-publish`, `csdlc-merge`, and
`csdlc-closeout`. Do not route those operations through connector actions or
legacy wrapper commands.

Use the shared GitHub token resolver through `token_file`,
`ADL_GITHUB_TOKEN_FILE`, `ADL_GITHUB_TOKEN`, `GITHUB_TOKEN`, `GH_TOKEN`, or the
operator-approved default token file. Never print or persist token contents.
