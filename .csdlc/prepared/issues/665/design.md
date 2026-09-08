# Issue #665 design: emergency branch adoption into typed lifecycle

## Problem

C-SDLC v2 currently has a stranded-work defect: an emergency issue-specific branch/worktree can contain legitimate reviewed product work before typed binding completes, but the lifecycle has no fail-closed adoption route. Ordinary `csdlc-bind` creates or owns new topology, while `csdlc-publish` correctly refuses a record that never moved through bound, implemented, and reviewed phases.

Issue #665 owns the tooling recovery route for that defect. It does not own the original #660 product hotfix or any live AWS mutation.

## Design goal

Add one explicit typed v2 adoption operation in the bind owner. The operation adopts a verified pre-existing issue branch/worktree into lifecycle authority, records immutable recovery evidence, and advances only from `ready` to `bound`. Implementation finalization, exact-head review, publication, merge, and closeout remain separate lifecycle gates.

## Scope

Primary code surfaces:

- `csdlc-v2/src/lifecycle.rs`
- `csdlc-v2/src/store.rs`
- `csdlc-v2/src/model.rs`
- `csdlc-v2/src/bin/csdlc-bind.rs`
- `csdlc-v2/tests/**`
- `docs/tooling/**`
- `.csdlc/prepared/issues/665/**`
- `.csdlc/issues/665/**`

## Adoption contract

The typed request must include exact issue, repository, branch, worktree, base branch, expected HEAD SHA, expected generation, expected digest, and actor. A successful adoption must verify:

- the target issue record exists and is currently `ready`;
- the request matches the issue record generation and digest;
- the branch is not `main` and is unique to the target issue;
- the worktree is under the approved FastWork worktree parent;
- the worktree is registered or otherwise verifiably adoptable without copying through `main`;
- the worktree is on the requested branch and at the requested HEAD;
- the requested HEAD is descended from the requested base branch;
- no conflicting typed branch/worktree binding already exists;
- no other issue-owned branch/worktree collision is visible.

The operation must preserve existing commits and tracked content. It must not reset, force checkout, overwrite, rebase, merge, or copy product work through `main`.

## Result contract

On success, the bind owner records durable machine-readable evidence containing the observed pre-state, exact adopted HEAD, base relationship, issue identity, worktree path, branch, actor, lifecycle generation, and digest. It advances only to `bound`; it must not claim implementation, review, publication, or merge readiness.

On failure, the operation fails closed with actionable diagnostics for stale generation/digest, HEAD mismatch, unsafe branch, unsafe worktree parent, dirty or ambiguous state, missing base ancestry, conflicting typed binding, or issue ownership collision.

## Validation plan

- focused positive adoption regression for the #660-shaped ready-phase emergency branch/worktree;
- negative cases for stale generation/digest, wrong issue/repository, `main`, wrong head, missing base ancestry, dirty state, unsafe worktree parent, multiple matching worktrees, and conflicting typed binding;
- proof that ordinary bind/create behavior remains unchanged;
- proof that an adopted issue can then continue through ordinary typed implementation finalization, exact-head review, and publication readiness;
- issue-owned focused validation lanes and diff hygiene;
- exact-head independent review before publication.

## Stop conditions

- the fix requires weakening publication or exact-head review gates;
- the fix needs raw GitHub lifecycle writes;
- adoption would mutate product commits or copy changes through `main`;
- collision detection cannot distinguish issue ownership safely;
- validation degenerates into zero-test proof.
