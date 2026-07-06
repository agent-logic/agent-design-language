# v0.91.7 WP-04 Closeout Lifecycle Repair

Issue: `#4946`

Status: repair complete, pending publication.

## Summary

This packet records the WP-04 sprint-review finding remediation performed after
the WP-04 implementation and closeout PRs had already merged. The repair did
not change WP-04 product behavior. It corrected lifecycle truth drift in
root-local ignored cards and preserved the remaining metric-accounting residual
as explicit release evidence.

## Findings Addressed

| Finding | Repair |
| --- | --- |
| Repo-native watch/closeout review still treated the WP-04 issue set as needing closeout after the closeout packet claimed child closeout validation. | Rechecked the WP-04 issue set and reconciled the tracked closeout packet with the actual merged PR/closed issue set. `pr.sh closeout` passed for `#4617`, `#4666`, `#4667`, `#4668`, `#4669`, `#4670`, `#4743`, and `#4631` after local card repair. The local `pr.sh watch --json` path still classifies closed/completed issues as `closeout_needed` with reason `issue_closed_completed`; that is recorded as watch-classifier tooling brittleness rather than WP-04 implementation incompleteness. |
| Closed WP-04 child cards retained stale `worktree_only`, `pr_open`, or null PR URL machine-readable SOR facts. | Normalized root-local ignored SOR records for `#4617`, `#4666`, `#4667`, `#4668`, `#4669`, `#4670`, `#4743`, and `#4631` to merged integration truth with the known merged PR URLs. |
| Closed WP-04 SRP records retained draft status. | Normalized root-local ignored SRP records for `#4667` and `#4631` to schema-valid approved review truth. |
| WP-04 nested per-issue accounting was at risk of being overclaimed. | Preserved the existing residual: automatic nested per-issue goal capture while an umbrella sprint goal is active is not fully proven. Unknown elapsed/token fields remain unknown rather than inferred. |

## Repaired Root-Local Lifecycle Records

These records are ignored local lifecycle state, so this tracked packet records
the repair for review:

- `.adl/v0.91.7/tasks/issue-4617__v0-91-7-tools-metrics-harvest-codex-session-telemetry-for-reporting-and-prediction/sor.md`
- `.adl/v0.91.7/tasks/issue-4666__v0-91-7-wp-04-goals-implement-nested-issue-and-sprint-goal-accounting/sor.md`
- `.adl/v0.91.7/tasks/issue-4667__v0-91-7-wp-04-metrics-implement-sor-time-token-resource-accounting/sor.md`
- `.adl/v0.91.7/tasks/issue-4667__v0-91-7-wp-04-metrics-implement-sor-time-token-resource-accounting/srp.md`
- `.adl/v0.91.7/tasks/issue-4668__v0-91-7-wp-04-telemetry-implement-codex-session-telemetry-harvesting/sor.md`
- `.adl/v0.91.7/tasks/issue-4669__v0-91-7-wp-04-backfill-execute-bounded-v0-91-6-metrics-backfill/sor.md`
- `.adl/v0.91.7/tasks/issue-4670__v0-91-7-wp-04-outliers-implement-execution-outlier-analysis/sor.md`
- `.adl/v0.91.7/tasks/issue-4743__v0-91-7-wp-04-prediction-implement-execution-metrics-prediction-engine/sor.md`
- `.adl/v0.91.7/tasks/issue-4631__v0-91-7-wp-04-goal-state-nested-goals-and-execution-metrics/sor.md`
- `.adl/v0.91.7/tasks/issue-4631__v0-91-7-wp-04-goal-state-nested-goals-and-execution-metrics/srp.md`

## Verification

- `rg` scan over the WP-04 root-local SOR/SRP set found no remaining stale
  terminal-state values: `state: worktree_only`, `state: pr_open`,
  `status: draft`, `Closeout state: not_started`, `PR state: not_open`,
  `Watcher disposition: not_started`, or `pr_url: null`.
- `pr.sh closeout` passed for `#4617`, `#4666`, `#4667`, `#4668`,
  `#4669`, `#4670`, `#4743`, and `#4631`, validating each issue's
  STP/SIP/SOR bundle and confirming the issue worktrees were already absent.
- `pr.sh watch --json` still returned `classification: closeout_needed` with
  `reason: issue_closed_completed` for the same closed issues. This packet
  treats that as a watch-classifier contract problem after closeout validation,
  not as evidence of unmerged WP-04 work.
- Tracked closeout evidence still records the residual nested-goal limitation
  without converting unknown metrics to zero or inferred values.
- WP-06 records and review surfaces were intentionally left untouched.

## Residual

WP-04 is sprint-closeout clean for the reviewed lifecycle truth findings. Two
bounded residuals remain outside this repair:

- Automatic nested per-issue goal capture under an active umbrella goal still
  needs a future implementation proof before ADL can claim complete automatic
  per-issue accounting in every sprint execution mode.
- Follow-up issue `#4950` tracks the repo-native watch classifier fix needed to
  distinguish "closed issue still needs closeout" from "closed issue has
  already passed closeout validation" so future sprint reviews do not treat a
  generic `issue_closed_completed` classification as stale lifecycle truth.
