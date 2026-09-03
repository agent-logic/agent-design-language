# Emergency branch adoption for C-SDLC v2

Issue-owned emergency product work can occasionally land on a Git branch/worktree before the typed lifecycle bind completes. That product action is not lifecycle authority by itself. Use typed adoption only to recover a verified, issue-owned, pre-existing worktree into the normal C-SDLC v2 lifecycle.

The recovery sequence is:

1. Confirm the issue record is `ready` and unbound.
2. Confirm the target worktree is a registered FastWork worktree on the issue branch.
3. Capture the exact target `HEAD` SHA and the ready issue generation/digest.
4. Run `csdlc-bind` with `adopt_existing: true`, exact `expected_head`, `expected_generation`, `expected_digest`, and a session/operator `actor`.
5. Continue with ordinary typed finalization, exact-head review, publication, merge, finish, and cleanup gates.

Adoption is fail-closed. It rejects `main`, stale generation or digest, a missing or mismatched worktree, wrong `HEAD`, missing base ancestry, dirty target state, unsafe worktree parents, conflicting typed bindings, or ambiguous branch/worktree topology.

Successful adoption advances only `ready` to `bound`. It records machine-readable adoption evidence in the issue audit and result surface; it does not claim implementation, review, publication, merge readiness, or closeout.
