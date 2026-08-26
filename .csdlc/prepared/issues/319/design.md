# #319 WP-30 Release Ceremony Design

## Outcome

Produce one exact-candidate v0.92 release-ceremony receipt after the reviewed-green #318 merge, without making asynchronous typed finish or worktree cleanup a predecessor gate.

## Authority and sequence

1. Bind the ceremony candidate to clean `main`, exact Git revision, v0.92 notes, plan, checklist, and the canonical #307-#319 issue graph.
2. Validate each predecessor according to its canonical disposition: closing-PR identity, checks, review, merge and ancestry for merge-backed rows; retained exact external-review evidence for #314; and retained replacement/retrospective authority for recordless #310. Local `closed_out` projections and worktree removal are supporting bookkeeping only.
3. Run `adl/tools/test_release_ceremony.sh` and the real `adl/tools/release_ceremony.sh --version v0.92 --target-branch codex/319-v092-wp30-release-ceremony` preflight from the clean exact candidate worktree.
4. Retain a reviewed pre-merge ceremony evidence packet containing candidate, intended tag/release identity, validation results, non-claims, and the authorized #268 qualification disposition. The final immutable post-merge receipt is generated from clean exact `main` after #319 merges and before separately authorized release mutation.
5. Publish #319 for exact-head review and merge before any tag or GitHub release mutation. Tag/release mutation remains an explicit operator-authorized post-merge step.

## Implementation boundary

- Repair the generic ceremony gate so dependency truth is merge-based and manifest-driven rather than requiring every historical same-version typed record to be `closed_out`.
- Add an exact v0.92 ceremony manifest and validator with disposition-specific predecessor proof under issue-owned paths.
- Update only final v0.92 release notes, plan, checklist, and ceremony packet where required for truthful candidate state.
- Do not implement product features, activate v0.93, execute #268, or create v0.92.1 issues.

## Failure and recovery

The preflight fails closed on a dirty/wrong branch, stale candidate, missing or non-ancestral merge, red check, unresolved review finding, conflicting tag/release identity, malformed manifest, or unsupported claim. No network mutation occurs during check-only execution. A later authorized split-step mutation must verify identity before every retry and preserve a published release rather than deleting history.

## Proof

- Ceremony tests prove dirty, wrong-branch, duplicate, partial-state, and merge-manifest denial.
- The issue-local validator proves the exact #308-#318 denominator using each row's canonical disposition and proves #318 merge ancestry.
- The real ceremony script completes pre-merge check-only proof from the clean exact candidate; after merge, the final receipt is generated from clean exact `main`.
- Typed `csdlc-review` must retain a successful exact-revision reviewer record before publication; diff hygiene is supporting proof only.
