# PR #597 lifecycle linkage defect

Date: 2026-08-31
Actor: worker-6

## Context

Exact-head review of PR #597 reported that the pull request body said
`Closes #596` even though #596 local C-SDLC v2 state remained `phase: ready`
with no bound branch/worktree, review, publication, readiness, or terminal
truth.

## Repair applied

The live PR body was updated through the typed C-SDLC v2 PR owner so PR #597 no
longer contains a GitHub closing keyword for #596. The current typed PR state
readback reports:

- `linked_issue: null`
- `state: open`
- `merged: false`
- `head_ref: codex/sprints-5-6-cutover-fixes`

The prepared PR create/update/state requests now use non-closing `Part-Of`
linkage for #596, #505, and #534.

## Remaining lifecycle/tooling defect

#596 card truth still describes the older intended close-on-merge behavior.
Attempting to correct the STP through typed `csdlc-edit` was rejected with:

```text
stp mutation is not allowed during ready
```

The available v2 bind route also cannot bind #596 from root `main`, because the
issue record only exists on the PR branch/worktree. The available bound-topology
migration route only applies to records that are already in `phase: bound`.

## Required follow-up

Add a typed v2 repair/adoption route for a review-found ready-phase issue whose
tracking record exists only on an existing PR branch, or demote/recover the
issue through a typed route that permits card truth repair before publication.
Do not reintroduce `Closes #596` into PR #597 until #596 has truthful bound,
reviewed, published, and reconciled lifecycle state.
