---
name: csdlc-v2-github
description: Execute repo-native C-SDLC v2 GitHub issue and PR actions through the split typed Rust command surface.
---

C-SDLC v2 remains the live lifecycle authority only until explicit V3-F/#505
cutover. Before that cutover, `csdlc-v3/**` is non-authoritative construction
evidence and must not mutate GitHub, publish PRs, finish issues, or retire v2.

Prefer the split owner binaries:

- Invoke `csdlc-github-issue run --request <request.json>` for
  `issue_create`, `issue_update`, `issue_comment`, `issue_close`, and
  `issue_read`.
- Invoke `csdlc-github-pr state --request <pr-state-request.json>` for direct
  PR-state observation.
- Invoke `csdlc-github-pr run --request <github_action_request.json>` only for
  `action: "pr_state"`, `action: "pr_create"`, or `action: "pr_update"`.

`csdlc-github run --request <request.json>` remains a compatibility facade for
the same typed `github_action_request` payload while callers migrate. Do not use
the GitHub connector, raw `gh`, legacy wrappers, shell/Python lifecycle
mutation, or AWS.

Every issue/comment/PR mutation must carry an `operation_key`. Issue and
comment mutations render it as a stable marker. PR create appends the same
marker to the governed body. PR update writes the governed body exactly as
provided, then reads back the body and fails closed on mismatch.

Supported action values:

- `issue_create`
- `issue_update`
- `issue_comment`
- `issue_close`
- `issue_read`
- `pr_state`
- `pr_create`
- `pr_update`

`pr_state` is read-only readiness observation. PR publication and terminal
delivery remain under the repo-native Rust v2 command surface:
`csdlc-publish` and `csdlc-finish`. Do not route those
operations through connector actions or legacy wrapper commands.

The install/coexistence inventory must include `csdlc-github`,
`csdlc-github-issue`, `csdlc-github-pr`, and `csdlc-pr-state`.
Treat a missing split binary as an installation failure, not as permission to
fall back to raw GitHub tooling.

Use the shared GitHub token resolver through `token_file`,
`ADL_GITHUB_TOKEN_FILE`, `ADL_GITHUB_TOKEN`, `GITHUB_TOKEN`, `GH_TOKEN`, or the
operator-approved default token file. Never print or persist token contents.
