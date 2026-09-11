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

Task ID: issue-0836
Run ID: issue-0836
Version: v0.92.1
Title: [v0.92.1][TAIL-06.21][quality] Publish recursive code-size and relocation evidence
Branch: codex/836-recursive-rust-size
Card Status: draft
Status: in_progress
Generated: <timestamp>

Execution:
- Actor: `<execution_actor>`
- Model: `<model>`
- Provider: `<provider>`
- Start Time: `<start_time>`
- End Time: `<end_time>`

## Summary

Implemented reproducible recursive Rust size/relocation evidence and corrected interpretation of facade reduction. Required issue goal created before implementation.

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
- Completion state: `implementation_complete_publication_pending`
- Issue goal ref: `issue-836-active-session-goal`
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
- Tracked implementation artifacts: `docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/**; docs/milestones/v0.92.1/features/RUST_RESILIENCE_REFACTORING_v0.92.1.md`
- Additional proof artifacts: `docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/document-audit.json; docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/validation.json`

## Actions taken
- `Measured exact PR547 first-parent baseline and merged candidate from recursive tracked Git blobs.`
- `Retained JSON/Markdown, ten-document hash/line audit, and negative report guardrails; linked correction from current feature doc.`
- `Ran focused fixture tests, independent recomputation, byte-stability checks and bounded independent review.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `All #836 changes pending publication`
- Integration state: `worktree_only`
- Verification scope: `<verification_scope>`
- Integration method used: `<integration_method_used>`
- Verification performed:
  - `<integration_verification_command>`
    `<integration_verification_effect>`
- Result: `not merged; publication pending`

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
  - `python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/test_measure.py; python3 docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measure.py --check docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measurement.json`
    `Validates recursive tracked scope, revision identity, gross deltas, rename and relocation evidence, rejected corruptions and generated claims.`
- Results:
  - `3 focused tests pass; retained-report check passes; two independent outputs byte-identical; independent source/relocation/document audit passes. Hosted CI and final exact-head review pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: local_proof_passed
    checks_run:
      - "<verification_check_1>"
  determinism:
    status: <verification_determinism_status>
    replay_verified: <verification_replay_verified>
    ordering_guarantees_verified: <verification_ordering_guarantees_verified>
  security_privacy:
    status: <verification_security_privacy_status>
    secrets_leakage_detected: <verification_secrets_leakage_detected>
    prompt_or_tool_arg_leakage_detected: <verification_prompt_or_tool_arg_leakage_detected>
    absolute_path_leakage_detected: <verification_absolute_path_leakage_detected>
  artifacts:
    status: <verification_artifacts_status>
    required_artifacts_present: <verification_required_artifacts_present>
    schema_changes:
      present: <verification_schema_changes_present>
      approved: <verification_schema_changes_approved>
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
- Primary proof surface: `docs/milestones/v0.92.1/evidence/refactoring/rust-01/issue-836/measurement.json`
- Required artifacts present: `<required_artifacts_present>`
- Artifact schema/version checks: `<artifact_schema_checks>`
- Hash/byte-stability checks: `<hash_byte_stability_checks>`
- Missing/optional artifacts and rationale: `<missing_optional_artifacts_rationale>`

## Decisions / Deviations
- `Resilience family grew 5278 to5995 lines; no code-reduction or behavior-proof claim. Historical #499 cards/validator preserved.`
- `Gross additions/deletions retain matching lines; possible relocation is separately labeled and cannot prove semantic movement.`

## Follow-ups / Deferred work
- `Obtain final exact-head review and publish Closes #836 PR.`
- `Hosted CI provides integration checks; merge remains asynchronous.`
