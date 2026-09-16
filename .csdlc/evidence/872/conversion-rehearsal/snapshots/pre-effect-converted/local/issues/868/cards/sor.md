# v0922-installed-command-contract

Canonical Template Source: `docs/templates/prompts/1.0.5/sor.md`

Authority notice: C-SDLC v3 is operational after V3-F/#505 and merged PR #591.
Authority requires the authenticated canonical native selector and reconciliation
receipt; missing or stale proof suspends authority. Retained typed v2 requires
explicit issue-scoped rollback or remediation approval.
Legacy `pr` editor routes are historical/retired compatibility orientation,
not current lifecycle authority.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-0868
Run ID: issue-0868
Version: 0.92.2
Title: [v0.92.2][SIM-02] One current installed command contract
Branch: codex/868-v0922-installed-command-contract
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T05:43:51.468228+00:00

Execution:
- Actor: `execute_868_command_contract with parallel installed-test, docs and independent advisory reviewers; Planning #4.4 coordinates`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

PR958 ready for review at6425ba9bbce4cc1f46789c2e3ac019adf86238b8, base main. Independent and operator-supplied reviews report no actionable findings.270localtests and276LinuxCItests pass;strictClippy,formatting,toolingcontracts,adl-ci pass. Coverage skipped byfocusedpathpolicy. Await explicit merge authorization; not merged or terminally closed.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `pending final candidate proof; implementation is in progress`

## Issue Metrics Truth
- Expected runtime class: `pending final candidate proof; implementation is in progress`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `no operator token budget assigned`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `in_progress`
- Issue goal ref: `Sprint #866 child #868 goal created by execute_868_command_contract after native bind/readiness; parent sprint goal active`
- Sprint goal ref: `issue-866`
- Goal metrics rollup ref: `.csdlc/evidence/868/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/868/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No measured execution or estimate pair exists`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/868/cards/sor.md`
- Tracked implementation artifacts: `Committed descriptor/result contracts, dispatcher glue, ephemeral remote result effects, test/PVF fixtures and current generated manuals at 6425ba9bbce4cc1f46789c2e3ac019adf86238b8`
- Additional proof artifacts: `.csdlc/evidence/868/execution/final-6425-validation.json;independent-review-6425.json;operator-provided-review-6425.json;ci/summary.json;pr-ready-result.json;pr-final-body-result.json`

## Actions taken
- `Native doctor and bind passed; child issue goal created before implementation. Existing README ownership preserved.`
- `Implemented descriptor/input variants, additive envelope and current documentation; advisory findings corrected with regression tests.`
- `Final source270tests pass,0failed/ignored.25attempt corpus21completed/3blocked/1intentionalinterruption.29actualenvelopes pass fullschema;22invalidcases rejected per schema run.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; primarymain inspection-only, coordination in resolvedGitmetadata`
- Worktree-only paths remaining: `Local cards, raw logs, retained candidates/provenance and review/publication receipts; tracked source committed and pushed`
- Integration state: `in_progress`
- Verification scope: `not_run`
- Integration method used: `Nativev3 github-pr operational create with authenticated reconciliation; separate publish --observe-github readiness readback`
- Verification performed:
  - `Native review --execute; github-pr --execute; publish --observe-github; gh pr view958 read-only base/head/state check`
    `Native PRcreate remote mutation and local receipts; publication observation read-only. No merge or activeownerreplacement.`
- Result: `pr_open_ready;CI_passed;awaiting_merge_authorization`

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout BRANCH -- PATH` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- Integration state describes lifecycle state of the integrated artifact set, not where verification happened.
- Verification scope describes where the verification commands were run.
- worktree_only means at least one required path still exists only outside the main repository path.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By native v3 `csdlc finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `Focused command_manifest, operational_cli_commands, installed_command_contract, proof_parity_install_commands, result_envelope; installed extended journey and shepherd matrix; tests/support/validate_result_schema.py`
    `270localtests,276LinuxCItests andtoolingcontracts passed. Coverage skip reason standalone_csdlc_v3_surface_requires_only_its_independent_focused_suite; no coverage proof claimed.`
- Results:
  - `passed_local_and_CI;coverage_skipped_by_path_policy`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: not_run
    checks_run:
      - "not_run"
  determinism:
    status: not_run
    replay_verified: not_run
    ordering_guarantees_verified: not_run
  security_privacy:
    status: not_run
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: not_run
    absolute_path_leakage_detected: not_run
  artifacts:
    status: not_run
    required_artifacts_present: not_run
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `pending final candidate proof; implementation is in progress`
- Fixtures or scripts used: `pending final candidate proof; implementation is in progress`
- Replay verification (same inputs -> same artifacts/order): `pending final candidate proof; implementation is in progress`
- Ordering guarantees (sorting / tie-break rules used): `pending final candidate proof; implementation is in progress`
- Artifact stability notes: `pending final candidate proof; implementation is in progress`

## Security / Privacy Checks
- Secret leakage scan performed: `pending final candidate proof; implementation is in progress`
- Prompt / tool argument redaction verified: `pending final candidate proof; implementation is in progress`
- Absolute path leakage check: `pending final candidate proof; implementation is in progress`
- Sandbox / policy invariants preserved: `pending final candidate proof; implementation is in progress`

## Replay Artifacts
- Trace bundle path(s): `pending final candidate proof; implementation is in progress`
- Run artifact root: `.csdlc/evidence/868 (planned)`
- Replay command used for verification: `pending final candidate proof; implementation is in progress`
- Replay result: `pending final candidate proof; implementation is in progress`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/868/execution/final-6425-validation.json`
- Required artifacts present: `pending final candidate proof; implementation is in progress`
- Artifact schema/version checks: `pending final candidate proof; implementation is in progress`
- Hash/byte-stability checks: `pending final candidate proof; implementation is in progress`
- Missing/optional artifacts and rationale: `CI, merge, terminal receipt and cleanup absent because publication has not occurred`

## Decisions / Deviations
- `Accepted merged #867 and native terminal closeout released #868. Preserve README ownership; additive ephemeral remote effect metadata admitted after comparing #849 linkage diff, with merge effects explicitly unknown.`
- `Preparation does not authorize a live transition or replace the active binary`

## Follow-ups / Deferred work
- `Await explicit operator authorization to merge PR958 exact6425 into main`
- `isolated issue 872 authenticated fence control`
