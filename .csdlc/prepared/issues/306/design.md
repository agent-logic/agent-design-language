# Issue 306 design

## Purpose

Issue #306 repairs a C-SDLC v2 publication/finish contract defect: publication
must not create required local metadata after the reviewed head has been pushed
in a way that makes `csdlc-finish` require another publication cycle just to
return to exact-clean state.

The observed tail pattern came from publication lanes such as #295/#301/#298.
This issue owns only the tooling defect and focused tests. It does not mutate
those active issue worktrees or root staging packets, does not create a #258 or
#5913 collision, and does not weaken exact-head review or exact-clean terminal
authority.

## Current failure model

`csdlc-publish` derives a publication intent, writes
`.csdlc/publication/<issue>.intent.json`, pushes or reconciles the PR branch,
then records publication truth locally. If any required lifecycle/publication
metadata is created after the pushed commit, the local worktree can become dirty
relative to the exact published head. `csdlc-finish` then correctly refuses to
derive terminal truth from a non-exact local/remote envelope, but the operator
is forced into a publication-tail recursion.

The repair must make the handoff explicit. A successful publication result must
leave one of these states, and no other state:

1. all required publication metadata is already included in the exact pushed
   head; or
2. the remaining local metadata is narrowly classified as safe, non-required,
   deterministic publication cache that `finish` never consumes as terminal
   authority and that is absent from `finish`'s exact-clean git status surface
   before publication reports success.

Anything else fails closed.

## Owned implementation boundary

Primary owner:

- `csdlc-v2/src/bin/csdlc-publish.rs`

Conditional owner only if the contract shape needs it:

- `csdlc-v2/src/publication.rs`

Focused tests should use a new isolated target such as:

- `csdlc-v2/tests/publication_tail.rs`

Read-only reference inputs:

- `csdlc-v2/src/finish.rs`
- `csdlc-v2/src/git.rs`
- existing publication/finish gate tests, including `csdlc-v2/tests/gate5.rs`
  and `csdlc-v2/tests/gate6.rs`

The design intentionally avoids editing `gate5.rs` so it does not collide with
the active #298 freeze on that file.

## Required contract

The implementation should introduce a fail-closed publication-tail contract with
these properties:

- Publication create/update/noop paths compute the exact metadata obligations
  before declaring success.
- Required metadata either lands in the pushed head or is rejected as a blocking
  dirty tail.
- Safe local-only metadata is explicitly narrow, deterministic, not consumed by
  terminal finish, and either stored outside the finish-checked git status
  surface, cleaned before successful publication return, or otherwise absent
  from `git status --porcelain --untracked-files=all`.
- Interruption windows are deterministic: interrupted-after-intent,
  interrupted-after-push, and interrupted-after-record retry without duplicate
  truth, silent overwrite, or ambiguous PR identity.
- `csdlc-finish` remains exact-head/exact-remote authority and does not learn a
  broad untracked-file allowlist.

## Validation design

Use isolated temporary repositories and local provider fixtures; do not use live
active issue worktrees as fixtures. The focused validation target should prove:

- create publication succeeds without a post-push required metadata tail;
- update publication succeeds without a post-push required metadata tail;
- noop publication remains deterministic and does not rewrite equivalent truth;
- interrupted-after-intent retry is deterministic;
- interrupted-after-push retry is deterministic and cannot hide a required
  dirty tail;
- interrupted-after-record retry is deterministic and cannot duplicate or
  overwrite publication truth after local record creation;
- finish-readiness can verify the exact published head without a second
  publication cycle solely for publish-created metadata;
- any safe local-only cache path is absent from the finish exact-clean status
  surface before publication reports success;
- existing committed typed metadata compatibility remains intact.

Strict Clippy for the focused target and touched csdlc-v2 code is required.

## Non-goals and stop conditions

Non-goals:

- no implementation in #295, #301, #298, #258, #5913, or other active issue
  worktrees/staging packets;
- no broad lifecycle redesign;
- no weakening exact-head review, exact-clean finish, or GitHub remote identity
  checks;
- no blanket untracked-file safe list;
- no PR merge or issue closeout as part of this issue.

Stop if the repair requires broad finish-authority changes, active issue
worktree fixtures, a second publication protocol, or any mutation outside the
declared owned boundary without Planning approval.

## Sprint 6 finish impact

This issue is terminal and ancestral for truthful finish/closeout of affected
open Sprint 6 publication-tail lanes unless Planning authorizes a separate
bounded workaround. It does not retroactively change already terminal issue
truth; it prevents the currently observed recursion from recurring in open and
future publication tails.
