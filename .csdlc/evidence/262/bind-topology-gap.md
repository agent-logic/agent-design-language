# #262 bind topology gap

## Summary

`csdlc-bind` has no direct happy path for an issue bundle that already exists
only on its FastWork issue branch/worktree with `phase: ready` and
`branch/worktree: null`.

## Observed behavior

- Invocation from primary `main` failed because the #262 issue bundle is not
  present on `main` yet:
  - request:
    `/Users/daniel/git/agent-design-language/.git/csdlc-v2/requests/262-bind-existing-fastwork.json`
  - root:
    `/Users/daniel/git/agent-design-language`
  - result:
    `{"code":"io","message":"No such file or directory (os error 2)","schema":"csdlc.error.v1"}`
- Invocation from the existing #262 issue worktree failed because the worktree
  is already on the issue branch while the record is not yet bound:
  - root:
    `/Volumes/FastWork/adl-worktrees/adl-issue-262-podcast-production-hosting`
  - result:
    `{"code":"unsafe_checkout","message":"binding must start from the declared base branch or the exact issue worktree","schema":"csdlc.error.v1"}`

## Governed workaround used

A temporary FastWork source worktree was created from the current #262 head:

- worktree:
  `/Volumes/FastWork/adl-worktrees/adl-issue-262-bind-source`
- branch:
  `codex/262-bind-source`

Then `csdlc-bind` was invoked from that source branch with the real #262
worktree as the target. The typed bind succeeded:

```json
{"created":false,"branch":"codex/262-podcast-production-hosting","worktree":"/Volumes/FastWork/adl-worktrees/adl-issue-262-podcast-production-hosting"}
```

## Follow-up

Consider adding a typed bind/recover route for branch-local ready issues whose
canonical execution worktree already exists, so operators do not need a
temporary source worktree to stamp the registered topology.
