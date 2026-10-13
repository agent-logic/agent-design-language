# <slug>

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

Task ID: issue-1266
Run ID: issue-1266
Version: 1.0.5
Title: [v0.93.1][C-SDLC v3][defect] Reconcile recordless merged closeout in external repositories
Branch: codex/1266-recordless-merged-closeout-recovery
Card Status: ready
Status: IN_PROGRESS
Generated: <timestamp>

Execution:
- Actor: `<execution_actor>`
- Model: `<model>`
- Provider: `<provider>`
- Start Time: `<start_time>`
- End Time: `<end_time>`

## Summary

Repair authenticated replay of historical external closeout after the consumer checkout advances, retain the complete v0.93.1 census, and reconcile only proven positive cases.

## PVF Lane Truth
- Initial PVF lane: `<initial_pvf_lane>`
- Planned PVF lane: `<planned_pvf_lane>`
- Final PVF lane: `<final_pvf_lane>`
- Lane change reason: `<lane_change_reason>`

## Issue Metrics Truth
- Expected runtime class: `<expected_runtime_class>`
- Estimated elapsed seconds: `<estimated_elapsed_seconds>`
- Actual elapsed seconds: `<actual_elapsed_seconds>`
- Actual active work seconds: `<actual_active_work_seconds>`
- Estimated total tokens: `<estimated_total_tokens>`
- Actual total tokens: `<actual_total_tokens>`
- Estimated validation seconds: `<estimated_validation_seconds>`
- Actual validation seconds: `<actual_validation_seconds>`
- Actual PR wait seconds: `<actual_pr_wait_seconds>`
- Actual CI wait seconds: `<actual_ci_wait_seconds>`
- Budget source: `<budget_source>`
- Goal metrics data source: `<actual_metrics_data_source>`
- Goal metrics source ref: `<actual_metrics_source_ref>`
- Data-source confidence: `<actual_metrics_confidence>`
- Estimate error percent: `<estimate_error_percent>`
- Completion state: `implementation_validated_pending_exact_head_review_and_publication`
- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`
- Validation planning prompt: `<vpp_card>`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `<variance_analysis_required>`
- Variance analysis completed: `<variance_analysis_completed>`
- Variance category: `<variance_category>`
- Variance note: `<variance_note>`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `<output_card>`
- Tracked implementation artifacts: `csdlc-v3/src/application/intent/terminal.rs; csdlc-v3/tests/installed_intent_commands.rs; .csdlc/evidence/1266/*.json`
- Additional proof artifacts: `.csdlc/evidence/1266/v0931-consumer-denominator.v1.json; .csdlc/evidence/1266/preexisting-terminal-receipts.v1.json; .csdlc/evidence/1266/consumer-authority-observations.v1.json; .csdlc/evidence/1266/infrastructure-reconciliation.v1.json`

## Actions taken
- `Retain the immutable closing PR head when replaying a valid legacy merged terminal receipt after the invoking checkout advances.`
- `Cross-check receipt schema, repository, issue, PR, head, publication repository and disposition against the digest-bound terminal state before authenticated replay.`
- `Reconciled both eligible Infrastructure issues canonically and preserved all 35 pre-existing terminal receipts byte-identically.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `All #1266 implementation and evidence changes await PR integration.`
- Integration state: `worktree_only`
- Verification scope: `<verification_scope>`
- Integration method used: `PR publication pending`
- Verification performed:
  - `git diff --check origin/main...HEAD`
    `Checks branch diff hygiene without claiming merge.`
- Result: `Not merged`

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
  - `cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_intent_commands external_pr_finish -- --nocapture; cargo check --manifest-path csdlc-v3/Cargo.toml --all-targets; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; git diff --check`
    `Proves external recordless closeout admission, fail-closed conflicts and receipt/state identity tamper rejection, replay after checkout advance, compilation, lint and diff hygiene.`
- Results:
  - `Focused external-pr tests passed 2/2; formatting, strict clippy, JSON validation and diff hygiene passed after complete-denominator evidence was recorded.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed
    checks_run:
      - "<verification_check_1>"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: <verification_ordering_guarantees_verified>
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: <verification_schema_changes_present>
      approved: <verification_schema_changes_approved>
```

## Determinism Evidence
- Determinism tests executed: `<determinism_tests_executed>`
- Fixtures or scripts used: `<fixtures_or_scripts_used>`
- Replay verification (same inputs -> same artifacts/order): `Synthetic installed external-consumer replay advances the checkout and requires expected_noop with byte-identical inventory.`
- Ordering guarantees (sorting / tie-break rules used): `<ordering_guarantees>`
- Artifact stability notes: `<artifact_stability_notes>`

## Security / Privacy Checks
- Secret leakage scan performed: `<secret_leakage_scan_performed>`
- Prompt / tool argument redaction verified: `<prompt_tool_arg_redaction_verified>`
- Absolute path leakage check: `<absolute_path_leakage_check>`
- Sandbox / policy invariants preserved: `<sandbox_policy_invariants_preserved>`

## Replay Artifacts
- Trace bundle path(s): `<trace_bundle_paths>`
- Run artifact root: `<run_artifact_root>`
- Replay command used for verification: `<replay_command>`
- Replay result: `<replay_result>`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/1266/v0931-consumer-denominator.v1.json`
- Required artifacts present: `true`
- Artifact schema/version checks: `<artifact_schema_checks>`
- Hash/byte-stability checks: `<hash_byte_stability_checks>`
- Missing/optional artifacts and rationale: `<missing_optional_artifacts_rationale>`

## Decisions / Deviations
- `<decision_or_deviation_1>`
- `<decision_or_deviation_2>`

## Follow-ups / Deferred work
- `Obtain fresh exact-head independent review and record its disposition through the native review route.`
- `Publish the reviewed exact head and require green CI; do not merge without separate authority.`
