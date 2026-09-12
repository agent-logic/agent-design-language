# 909-gcp-move-in

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

Task ID: issue-0909
Run ID: issue-0909
Version: v0.92.2
Title: [v0.92.2][OPS-GCP] Produce one apply-ready company GCP move-in execution packet
Branch: codex/909-gcp-move-in
Card Status: completed
Status: IN_PROGRESS
Generated: <timestamp>

Execution:
- Actor: `<execution_actor>`
- Model: `<model>`
- Provider: `<provider>`
- Start Time: `<start_time>`
- End Time: `<end_time>`

## Summary

Complete reviewed#909 companyGCP planning packet published as PR#949. Bootstrap2noops, platform20noops from approved reconstructedprivatecandidate, organization5creates. Futurecustody/adoption/freshplan/application approvals separate; no cloud or remote-statechanges.

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
- Completion state: `implementation_complete`
- Issue goal ref: `Issue #909 active session goal created before implementation`
- Sprint goal ref: `Sprint 8 umbrella #934`
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
- Tracked implementation artifacts: `.csdlc/evidence/909/MOVE_IN_PACKET.md; APPLICATION_CHECKLIST.md; platform-plan-summary.json; local-recovery-receipt.json; bootstrap-plan-summary.json; organization-plan-summary.json; source-inventory.json; live-inventory.json; foundation-controls.json; effective-policies.json; VALIDATION.json; historicalrecoveryproposal/gap.`
- Additional proof artifacts: `<additional_proof_artifacts>`

## Actions taken
- `Reused unchanged source/locks; verified company organization/project/billing plus32census/20effectivepolicy observations and113quotas per project.`
- `Produced2bootstrapnoops and5organizationcreates; approved isolated recovery serial33→36 produced20platformnoops; no cloud or remote state changes.`
- `Native review, PR#949 creation and authenticated publication reconciliation passed. Hosted check and merge state remain live PRtruth; no apply/merge performed.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `.csdlc/evidence/909; .csdlc/issues/909; .csdlc/transactions/completed/909`
- Integration state: `pr_open`
- Verification scope: `bound issue worktree and approved company identity readback attempt`
- Integration method used: `<integration_method_used>`
- Verification performed:
  - `<integration_verification_command>`
    `<integration_verification_effect>`
- Result: `Published native PR#949 targeting main with Closes#909. Native authenticated publication/readback passed. Current CI and merge status must be read live from PR#949; no merge or terminal closeout claimed.`

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
  - `Terraform9staticchecks; companyinventory/policy/quotareads; bootstrap/organization savedplans; approvedexact3localimports;20resourcepre/postreadbacks; realplatformsavedplan; sourcehash/JSON/link/privacychecks; independentactualproofandpacketreview.`
    `Proves actual company inventory and all3package plans:2bootstrapnoops,20platformnoops,5organizationcreates. Platform candidate reconstructed under explicit approval; future adoption/application separate.`
- Results:
  - `Nine static checks and all company readbacks passed. Bootstrap2noops, platform20noops, organization5creates. Exactly3approvedlocalimports; pre/postvaluesunchanged. Independent final acceptance review PASS after precisionfixes.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed_local_and_independent_review
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
