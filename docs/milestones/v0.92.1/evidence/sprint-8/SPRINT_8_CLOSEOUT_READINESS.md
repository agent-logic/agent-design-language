# Sprint 8 closeout readiness

Issue: #536 — `[v0.92.1][Sprint 8] Podcast and Observatory`

Status: **waiting only on #512**

This record prepares the Sprint 8 umbrella for closeout without claiming that
the sprint is already complete. Membership is the issue-body membership v5:
#51, #261, #262, #263, #264, #342, #511, and #512. Issue #84 is deferred and
is not a Sprint 8 completion gate.

## Child dispositions

| Issue | Disposition | Integration evidence |
| --- | --- | --- |
| #51 | Closed as the podcast coordination parent. Repository work is complete; remaining provider submissions/public launch are explicitly routed to backlog issue #671. | Coordination closeout on #51; no independent closing PR. |
| #261 | Closed and merged. | PR #611; PR head `78af9095a16886f8c0876e139113620dca806984`; merge `7f04298f87e2bde5b90eb174d9d7758067d348d0`, ancestral to `origin/main`. |
| #262 | Closed and merged. | PR #618; PR head `842b7d57d09b66a6f9ea9a43a4e5d02be43aa3bb`; merge `6e01e2bbe40915e814e43e54f84dfb61c84601e3`, ancestral to `origin/main`. |
| #263 | Closed and merged. | PR #626; PR head `a77e8e6c8e6e3c59330a5c15ce45924985735b7c`; merge `e13b5db0b49f9bc6772ca2765634a852abdf1ed2`, ancestral to `origin/main`. |
| #264 | Closed and merged. This delivered repository-side preparation only; it did not claim public launch or provider submission. | PR #649; PR head `a285a690b86f95f6fc3ea3dd150ded76b32c469b`; merge `bdbf8aa32620da0e277bf3e2ed5f272354021744`, ancestral to `origin/main`. |
| #342 | Closed and merged. | PR #586; PR head `822c81e9d0ad15e479960de542d419e64c80e1f9`; merge `b381edce8020567c4ac5af03f5df062157f55a16`, ancestral to `origin/main`. |
| #511 | Closed as absorbed into #512, with its requirements retained in the #512 implementation/review boundary. | Explicit closeout note on #511; no duplicate implementation PR. |
| #512 | **Pending.** Active Observatory implementation must be completed, independently reviewed, green, merged, and ancestral to `main`. | Insert exact PR, reviewed head, check result, merge commit, and ancestry after merge. |

## Final closeout gate

After #512 merges:

1. Record its exact PR number, reviewed head, required-check result, and merge
   commit in this table.
2. Verify that merge commit and the five recorded child merge commits are
   ancestors of current `origin/main`.
3. Run one bounded independent review of this aggregate closeout record.
4. Publish the #536 closeout through the typed lifecycle route with
   `Closes #536`.
5. Treat typed finish and worktree cleanup as asynchronous bookkeeping after
   merge; no child issue depends on that bookkeeping.

Sprint 8 closeout must not claim that the podcast has launched publicly or
that provider submissions occurred. Those external actions remain in #671.
