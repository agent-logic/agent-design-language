# dynamic-agent-health-task-failures

Canonical Template Source: `docs/templates/prompts/1.0.4/sor.md`

Authority notice: V3-F/#505 is the pending tooling changeover decision; until
that operator-reviewed cutover is approved, merged, and terminally reconciled,
C-SDLC v2 remains live authority.
Legacy `pr` editor routes are historical/retired compatibility orientation,
not current lifecycle authority.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-0759
Run ID: issue-0759
Version: 1.0.4
Title: [v0.92.1][TAIL-06.03][runtime] Isolate dynamic-agent health task failures
Branch: codex/759-dynamic-agent-health-task-failures
Card Status: ready
Status: implemented_current_main_resynced_validation_passed_review_pending
Generated: <timestamp>

Execution:
- Actor: `codex`
- Model: `gpt-5-codex`
- Provider: `openai`
- Start Time: `2026-09-09T00:00:00-07:00`
- End Time: `not_finished_review_pending_after_ci_repair`

## Summary

Implemented the #759 dynamic-agent health sweep repair and repaired CI-discovered integration drift. After PR #779 run 34381946883 failed on the synthetic PR merge because current main restored the stable resident shepherd runtime id contract, the branch was resynced through origin/main 11a3fe88e0bc30e46a9fd3a7ff39f01807e3477f. Reviewer P2 on stale/misplaced SNTP evidence was fixed, and final r15 validation passed at source head 23a5b82e6a353eaf4963357f72b536939430cab5.

## PVF Lane Truth
- Initial PVF lane: `runtime-focused`
- Planned PVF lane: `runtime-focused-defect-regression`
- Final PVF lane: `runtime-focused-defect-regression`
- Lane change reason: `No lane change; issue #759 is a narrow Runtime async health-sweep defect.`

## Issue Metrics Truth
- Expected runtime class: `Rust runtime crate`
- Estimated elapsed seconds: `3600`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `1800`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `not_started`
- Actual CI wait seconds: `not_started`
- Budget source: `SPP/VPP estimates and live Codex goal`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `not_collected`
- Data-source confidence: `medium`
- Estimate error percent: `unknown`
- Completion state: `post_ci_red_current_main_resynced_validation_passed_review_pending`
- Issue goal ref: `Codex goal: Issue #759 dynamic-agent health sweep remediation`
- Sprint goal ref: `v0.92.1 closeout tail runtime defect lane`
- Goal metrics rollup ref: `v0.92.1`
- Validation planning prompt: `.csdlc/issues/759/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable_pre_closeout`
- Variance category: `not_applicable`
- Variance note: `Closeout metrics remain pending until PR/CI/finish complete.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/759/cards/sor.md`
- Tracked implementation artifacts: `adl-runtime-kernel/src/control.rs; .csdlc/evidence/759/*.log; .csdlc/issues/759/cards/*.md and *.values.json`
- Additional proof artifacts: `.csdlc/evidence/759/validation-r15.md; .csdlc/evidence/759/runtime-v3-fast-full-r15.log sha256 fa55d84b9444f3ae96702cbb8be01f2fc30fb03d8afea9827561b1440ba123e2; .csdlc/evidence/759/focused-dynamic-agent-health-r15.log sha256 66f30d3138fff8335a83c398e23ebd5dfc9800aace35819783ea983fe40956cb; .csdlc/evidence/759/focused-resident-shepherd-id-r15.log sha256 4fbbfc368908ba0a2a9551d89c61a8ec7448daea0b15962db404e9e39577abea; .csdlc/evidence/759/assembly-sntp-isolated-r15.log sha256 124680c06b464b1874737dbef05cec70a2f8bc1c261df9c1bd1031cc4125fff9; .csdlc/evidence/759/assembly-shutdown-grace-isolated-r15.log sha256 c8eaadc6b557198b4f08609e3db459e358c2c29d536f704cff04f13e8b6cd1cc; .csdlc/evidence/759/observability-s3-outage-isolated-r15.log sha256 eace6c62dce52168b62b59548db8b1eaa49cfd48a0571be9ec29fb416501cc1f; .csdlc/evidence/759/strict-clippy-r15.log sha256 11d208621432c4d2f122eb7b232efea5b902d5794a303b860be68af4593c310a; .csdlc/evidence/759/html-observatory-proof-r15.log sha256 e6333af7915d7b7e24cda4ec9362d1c0039a4fd56b89f98c07acd1b284e69587; .csdlc/evidence/759/fmt-check-r15.log sha256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855; .csdlc/evidence/759/diff-check-r15.log sha256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855; diagnostic red logs retained for r8/r10/r14 scheduler-sensitive classifications`

## Actions taken
- `Replaced the JoinSet loop that matched only successful joins with a loop over every join result.`
- `Stored a stable task-id to dynamic-agent declaration map so task panic/cancel failures project against the correct agent identity.`
- `Added deterministic panic and cancellation regressions proving peer successes remain healthy and failed tasks surface as failed; after current-main resync, preserved the stable resident shepherd runtime id contract through resident_shepherd_runtime_id while retaining configured canonical/display projection.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `None yet; work is committed on bound branch codex/759-dynamic-agent-health-task-failures.`
- Worktree-only paths remaining: `Branch worktree contains implementation/evidence pending review and publication.`
- Integration state: `worktree_branch_committed_resynced_validation_passed_review_pending`
- Verification scope: `Bound FastWork worktree on branch codex/759-dynamic-agent-health-task-failures.`
- Integration method used: `native C-SDLC v3 bind plus git commit in issue worktree`
- Verification performed:
  - `git status --short --branch; git diff --check HEAD`
    `Confirmed current-main resync and no whitespace errors at source SHA 23a5b82e6a353eaf4963357f72b536939430cab5.`
- Result: `Local branch implementation exists; PR #779 previously published and red on stale remote run 34381946883; branch is repaired locally at source SHA 23a5b82e6a353eaf4963357f72b536939430cab5 and awaits fresh review, republish, and CI.`

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout BRANCH -- PATH` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- Integration state describes lifecycle state of the integrated artifact set, not where verification happened.
- Verification scope describes where the verification commands were run.
- worktree_only means at least one required path still exists only outside the main repository path.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By typed `csdlc-finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `cargo test --manifest-path adl-runtime-kernel/Cargo.toml; cargo test --manifest-path adl-runtime-kernel/Cargo.toml dynamic_agent_health_sweep_drains_after_task -- --nocapture; cargo test --manifest-path adl-runtime-kernel/Cargo.toml resident_shepherd_construction_uses_configured_canonical_name_and_truthful_counts --test agent_roster -- --nocapture; cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check; cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings; bash adl/tools/test_v0917_html_observatory_integrated_proof.sh; git diff --check HEAD`
    `Full runtime command reproduces the hosted adl-runtime-v3-fast lane geometry; focused regression proves panic/cancel task isolation and peer projection retention; resident shepherd focused check proves the current-main stable id contract; HTML Observatory proof, fmt, clippy, and diff prove integration and hygiene.`
- Results:
  - `PASS at source SHA 23a5b82e6a353eaf4963357f72b536939430cab5; all recorded r15 command statuses are 0. Hosted red run 34381946883 classified as current-main ancestry drift and repaired by resync. Local r8/r10/r14 broad-run failures are retained and classified as scheduler/timing-sensitive with isolated PASS evidence; the SNTP startup harness budget is now on the actual startup wait and the Vector S3 outage proof deadline is widened without production config changes.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed_local
    checks_run:
      - "full runtime suite passed; focused dynamic-agent health panic/cancel regression 2/2 passed; resident shepherd stable id check passed"
  determinism:
    status: passed_focused
    replay_verified: not_applicable_no_replay_artifact
    ordering_guarantees_verified: stable task-id mapping verified by deterministic regressions
  security_privacy:
    status: passed_no_secret_surface
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: present_local
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `dynamic_agent_health_sweep_drains_after_task_panic and dynamic_agent_health_sweep_drains_after_task_cancellation`
- Fixtures or scripts used: `In-crate mock Ollama server and cfg(test) forced task failure hook.`
- Replay verification (same inputs -> same artifacts/order): `not applicable; no replay artifact generated.`
- Ordering guarantees (sorting / tie-break rules used): `JoinSet outcomes are unordered but every outcome is drained; task id maps each failure to the correct declaration.`
- Artifact stability notes: `validation-r15.md records exact source SHA, argv, status, and SHA-256 for every r15 log; fmt and diff logs are zero-byte success outputs. HTML proof regenerated #5789 shared certificate evidence, which was restored to tracked bytes after proof.`

## Security / Privacy Checks
- Secret leakage scan performed: `not_applicable_no_secret_surface`
- Prompt / tool argument redaction verified: `not_applicable_no_prompt_or_tool_arg_artifact`
- Absolute path leakage check: `Evidence paths are repo-relative in SOR; local log headers do not require publication of host paths.`
- Sandbox / policy invariants preserved: `Implementation confined to bound issue branch/worktree.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable`
- Run artifact root: `.csdlc/evidence/759`
- Replay command used for verification: `not_applicable`
- Replay result: `not_applicable`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/759/validation-r15.md`
- Required artifacts present: `true`
- Artifact schema/version checks: `No schema changes.`
- Hash/byte-stability checks: `SHA256 hashes recorded for full runtime, focused dynamic health, focused resident shepherd, SNTP isolated harness, restored shutdown-grace harness, Vector S3 outage isolated harness, strict clippy, HTML Observatory proof, fmt, and diff logs.`
- Missing/optional artifacts and rationale: `Hosted CI rerun is pending fresh review and republish after current-main resync.`

## Decisions / Deviations
- `Native-v3 card validation currently reports `card_structure_invalid` because of a known validator schema-path defect; doctor still confirms lifecycle digest and binding.`
- `No provider health semantics were changed.`

## Follow-ups / Deferred work
- `Run fresh review on the repaired current-main-resynced source head 23a5b82e6a353eaf4963357f72b536939430cab5, then republish PR #779.`
- `Watch CI, finish through native C-SDLC v3 if merge authority is satisfied, and clean separately after merge.`
