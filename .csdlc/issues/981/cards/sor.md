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

Task ID: issue-0981
Run ID: issue-0981
Version: 1.0.5
Title: [v0.92.2][C-SDLC v3][defect] Do not misclassify repository-scoped issue creation receipts
Branch: codex/981-repository-scoped-issue-creation-receipt
Card Status: ready
Status: in_progress
Generated: <timestamp>

Execution:
- Actor: `<execution_actor>`
- Model: `<model>`
- Provider: `<provider>`
- Start Time: `<start_time>`
- End Time: `<end_time>`

## Summary

Implemented exact repository-scoped issue-creation receipt classification and repaired the prepare/proof validator-admission mismatch exposed by the real #981 proof; independent review, publication, CI, merge, and terminal closeout remain pending.

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
- Completion state: `<completion_state>`
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
- Tracked implementation artifacts: `csdlc-v3/src/commands/remote/mod.rs; csdlc-v3/src/storage/semantic.rs; csdlc-v3/src/commands/proof/intent.rs; csdlc-v3/tests/transactions.rs`
- Additional proof artifacts: `.csdlc/evidence/981/validation.md`

## Actions taken
- `<actions_taken_line_1>`
- `<actions_taken_line_2>`
- `<actions_taken_line_3>`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `<main_repo_paths_updated>`
- Worktree-only paths remaining: `<worktree_only_paths_remaining>`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree`
- Integration method used: `<integration_method_used>`
- Verification performed:
  - `<integration_verification_command>`
    `<integration_verification_effect>`
- Result: `<integration_result>`

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
  - `cargo test --manifest-path csdlc-v3/Cargo.toml semantic_gate_a; native csdlc proof 981; cargo test --manifest-path csdlc-v3/Cargo.toml; cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets --all-features -- -D warnings; git diff --check`
    `Proves repository-scoped receipt classification, safe single-filter admission, final-binary dependency-record discovery, full C-SDLC v3 behavior, formatting, strict lint, and patch hygiene.`
- Results:
  - `The full component suite passed; native proof executed 21 semantic_gate_a tests with zero failures and unchanged inputs; one manual-only prepared-measurement test remained intentionally ignored.`

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
      - "Native 21-test proof, focused receipt and proof regressions, full csdlc-v3 suite, fmt, strict clippy, and diff hygiene"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: not_applicable
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `<determinism_tests_executed>`
- Fixtures or scripts used: `<fixtures_or_scripts_used>`
- Replay verification (same inputs -> same artifacts/order): `<replay_verification>`
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
- Primary proof surface: `<primary_proof_surface>`
- Required artifacts present: `<required_artifacts_present>`
- Artifact schema/version checks: `<artifact_schema_checks>`
- Hash/byte-stability checks: `<hash_byte_stability_checks>`
- Missing/optional artifacts and rationale: `<missing_optional_artifacts_rationale>`

## Decisions / Deviations
- `<decision_or_deviation_1>`
- `<decision_or_deviation_2>`

## Follow-ups / Deferred work
- `<follow_up_1>`
- `<follow_up_2>`
