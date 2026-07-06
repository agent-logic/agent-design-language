# v0.91.7 WP-03 Review Remediation (#4953)

Status: pre_pr_review_findings_fixed

Issue: #4953

Scope: repair the WP-03 review findings for lifecycle shepherd state semantics, closeout truth, stale worktree residue, and issue discoverability.

## Findings Addressed

| Finding | Disposition |
| --- | --- |
| Closed issue with merged PR was reported as `closed_no_pr` by the shepherd. | Fixed in the repo-native watcher classifier: closed/completed issues with linked merged PR validation now classify as `merged_pending_closeout`, which maps to `merged_needs_closeout`. |
| #4630 SRP remained draft/not-run after PR #4714 merged. | Repaired local SRP truth from the recorded #4630 SOR review evidence. |
| #4630 SOR still carried stale `pr_open` follow-up truth. | Repaired local SOR integration facts to `merged` and removed obsolete pending-PR follow-ups. |
| #4713 claimed its worktree was pruned while a stale `.worktrees/adl-wp-4713` directory remained. | Removed stale non-git residue containing only `.adl/runtime_environment.json`; `pr closeout 4713` now reports the worktree absent/prune not needed. |
| Related WP-03 issues were not consistently discoverable by label. | Added `wp:WP-03` to #4709, #4713, and #4721 with repo-native issue-edit commands while preserving their task/bug type labels. |

## WP-03 Truth Consumption

| Surface | Evidence / disposition |
| --- | --- |
| Lifecycle shepherd command | #4630 / PR #4714 merged; #4953 repairs merged-PR closeout state semantics. |
| Root-main issue edit safety | #4709 / PR #4811 merged; label now includes `wp:WP-03`. |
| Rust post-merge closeout watcher attach | #4713 / PR #4829 merged; stale local residue removed and closeout revalidated. |
| Repo-native label ensure/edit path | #4721 / PR #4813 merged; label now includes `wp:WP-03`. |
| Broader watcher closeout classifier gap | Already tracked separately by #4950; not duplicated here. |

## Validation Plan

- Focused Rust lifecycle shepherd/watch tests.
- Pre-PR subagent review.
- Repo-native `pr.sh shepherd 4630 --json` confirmation.
- Repo-native `pr.sh closeout 4630` confirmation.
- Repo-native `pr.sh closeout 4713` confirmation.
- `git diff --check`.

## Pre-PR Review Findings

| Finding | Disposition |
| --- | --- |
| #4953 SOR overclaimed `pr_open` before PR publication. | Fixed; SOR now records implementation-complete pre-publication truth with `Integration state: worktree_only`. |
| #4953 SRP still recorded review as `not_run`. | Fixed; SRP now records the pre-PR review findings and dispositions. |
| Regression only proved watch classification, not shepherd-facing state. | Fixed; the regression now asserts `merged_needs_closeout` through `build_issue_lifecycle_shepherd_report`. |

## Non-Claims

- This packet does not close #4953 by itself.
- This packet does not claim unrelated WP-06 or resilience work is complete.
- This packet does not supersede #4950.
