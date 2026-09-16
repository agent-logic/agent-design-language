# v0922-operator-review-shell

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

Task ID: issue-0891
Run ID: issue-0891
Version: 0.92.2
Title: [v0.92.2][CF-SHELL] Operate a real repository review through the installed CodeFriend shell
Branch: codex/891-v0922-operator-review-shell
Card Status: ready
Status: implemented_pending_review
Generated: 2026-09-12T00:09:56.378553+00:00

Execution:
- Actor: `unassigned implementation owner`
- Model: `unknown`
- Provider: `openai:gpt-4.1-mini via credential_ref env:OPENAI_API_KEY; key value was not stored in repo artifacts`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

Implemented and remediated the installed CodeFriend operator review shell for #891. The product exposes `adl codefriend review shell start|inspect|cancel|retry|withhold-publication` around the existing isolated four-lane review runner. After exact-head review found cancellation/retry truth gaps, the implementation archives stale cancel requests before retry, prevents final-lane cancellation from fabricating completion, and records a runner-owned settlement marker so retry after pre-run failure is not permanently blocked when `run.json` was never produced. The retained real-provider artifact is a bounded two-file small-fixture smoke proof only; it is not the required pinned Vector ten-file qualification, and Linux qualification remains deferred.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `no lane change; runtime lane remained correct for an installed operator shell`

## Issue Metrics Truth
- Expected runtime class: `local_candidate_build_and_real_provider_shell_proof`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `2400`
- Actual validation seconds: `approximately 217 local seconds observed for cargo check plus focused test build/run; exact wall time not recorded as authoritative metrics`
- Actual PR wait seconds: `not_started`
- Actual CI wait seconds: `not_started`
- Budget source: `no operator token budget assigned`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implemented_remediated_pending_fresh_review_publication_ci_with_required_vector_and_linux_qualification_not_claimed`
- Issue goal ref: `Sprint 4 #930 active goal covers #891 execution in this session; single goal slot prevented replacing it with a separate child goal`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/891/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/891/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `Per-issue elapsed/token metrics are not available from the active sprint goal; validation seconds are approximate and not used as precise variance data.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/891/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/operator/mod.rs; adl/src/codefriend/mod.rs; adl/src/codefriend/review/runner.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_review.rs`
- Additional proof artifacts: `.csdlc/evidence/891/real-provider-shell-proof/store; .csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture; .csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture.stdout.json; .csdlc/evidence/891/real-provider-shell-proof/admission-small-fixture.json; .csdlc/evidence/891/real-provider-shell-proof/openai-provider-request.json`

## Actions taken
- `Added `adl/src/codefriend/operator/mod.rs` as a thin operator-control state layer over the existing CodeFriend review runner.`
- `Wired `adl codefriend review shell` commands in `adl/src/cli/codefriend_cmd.rs` for start, inspect, cancel, retry and withhold-publication.`
- `Added cancellation checkpoints to `adl/src/codefriend/review/runner.rs` and focused installed CLI scenarios in `adl/tests/codefriend_review.rs`.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `adl/src/codefriend/operator/mod.rs; adl/src/codefriend/mod.rs; adl/src/codefriend/review/runner.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_review.rs`
- Worktree-only paths remaining: `.csdlc/issues/891/ and .csdlc/transactions/completed/891/ are generated lifecycle material in the bound worktree until publication/finish; implementation source changes are tracked candidate paths.`
- Integration state: `worktree_candidate_ready_for_review`
- Verification scope: `bound_issue_worktree`
- Integration method used: `bounded implementation in registered FastWork issue worktree; not published or merged`
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_review review_shell`
    `verified in bound issue worktree; not yet published or merged`
- Result: `not_integrated`

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
  - `cargo fmt --all -- --check (from adl/); cargo test --manifest-path Cargo.toml --lib codefriend::review::runner::tests (from adl/); cargo test --manifest-path Cargo.toml --test codefriend_review review_shell (from adl/); python3 -m json.tool ../.csdlc/issues/891/cards/sor.values.json; git -C .. diff --check; cargo clippy --all-targets -- -D warnings (from adl/)`
    `Focused deterministic tests, runner parser tests, broad Rust fmt/clippy, SOR values JSON parsing and diff hygiene passed for the #891 operator shell remediation. The retained OpenAI-backed two-file small-fixture smoke proof remains recorded, but required pinned Vector ten-file and Linux qualification are not claimed.`
- Results:
  - `passed`

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
      - "installed shell real-provider proof completed"
  determinism:
    status: passed_for_deterministic_control_fixtures
    replay_verified: focused integration tests create isolated local fixture repositories, stores, controlled provider transport and output directories per run
    ordering_guarantees_verified: passed
  security_privacy:
    status: passed_for_local_fixture_scope
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed_for_two_file_small_fixture_only_required_vector_and_linux_qualification_not_claimed
    required_artifacts_present: passed_for_local_fixture_scope_only
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `cargo test --manifest-path adl/Cargo.toml --test codefriend_review review_shell`
- Fixtures or scripts used: `adl/tests/codefriend_review.rs fixture servers plus .csdlc/evidence/891/real-provider-shell-proof bounded two-file fixture repository`
- Replay verification (same inputs -> same artifacts/order): `focused integration tests create isolated local fixture repositories, stores, controlled provider transport and output directories per run`
- Ordering guarantees (sorting / tie-break rules used): `attempt settlement reloads latest persisted operator-state before writing, so older cancelled attempts cannot overwrite newer retry completion`
- Artifact stability notes: `Retries now archive attempt-local cancel-request.json before a new attempt; cancellation after the last provider request now records failed/cancelled run truth instead of complete.`

## Security / Privacy Checks
- Secret leakage scan performed: `reviewed recorded provider request/result artifacts for credential references; key value was supplied only through env:OPENAI_API_KEY and was not stored`
- Prompt / tool argument redaction verified: `provider request records credential_ref env:OPENAI_API_KEY only; command record uses <approved env reference> placeholder and the external store path is non-secret`
- Absolute path leakage check: `passed; final SOR references repo-relative proof artifacts and records provider credentials only as env:OPENAI_API_KEY`
- Sandbox / policy invariants preserved: `real-provider proof used bounded repository fixture, no source mutation, and no secret value persistence`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture/operator-state.json; .csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture/attempts/1/review/run.json; .csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture/attempts/1/review/review-record.json`
- Run artifact root: `.csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture records the successful installed shell real-provider run; local deterministic test artifacts remain under adl/target/codefriend-review-tests`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_review`
- Replay result: `passed for focused deterministic shell tests`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture`
- Required artifacts present: `passed for bounded two-file small-fixture smoke proof only; operator-state.json, run.json, review-record.json and four lane artifact bundles are present under .csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture. This does not satisfy the required pinned Vector ten-file scope or Linux installed qualification.`
- Artifact schema/version checks: `operator-state JSON, review run JSON, lane result JSON and review-record JSON are present in the committed small-fixture proof packet; focused tests parse the operator/review artifacts; SOR values JSON parses with python3 -m json.tool. Native card validation was not rerun because the generated native v3 csdlc binary was unavailable at the expected worktree path.`
- Hash/byte-stability checks: `cargo fmt --all -- --check, focused runner parser tests, focused shell integration tests, SOR values JSON parsing, git diff --check and cargo clippy --all-targets -- -D warnings passed after this remediation. Native card validation was not rerun because the generated native v3 csdlc binary was unavailable at the expected worktree path.`
- Missing/optional artifacts and rationale: `Real external provider proof was executed on the macOS operator host with the installed shell against OpenAI gpt-4.1-mini and completed with lane artifacts for a bounded two-file small-fixture repository. The required pinned Vector ten-file proof and Linux installed qualification were not executed in this SOR state and are not claimed. Repository CI remains a publication gate; no additional cloud, synthesis, renderer, full Vector, Linux or publication proof is claimed here.`

## Decisions / Deviations
- `The issue contract proposed `adl/tests/codefriend_shell.rs`; the implementation extended the existing `adl/tests/codefriend_review.rs` integration target because it already owns the installed review runner fixture and avoids duplicating fixture infrastructure.`
- `No synthesis, remediation, UX, rendering or report publication behavior was implemented; those remain owned by sibling Sprint 4 issues.`
- `First exact-head review at d78c5ba0b9 found two P2 cancellation truth defects: retry after cancel was blocked by stale cancel-request.json, and final-lane cancellation could still settle as complete. Both were remediated in source and covered by focused regressions.`
- `Second exact-head review at 926d66dff109 found that failed pre-run attempts could block retry when no run.json existed; remediation records attempts/<n>/settlement.json on runner errors and adds a focused retry-after-pre-run-failure regression.`
- `Second exact-head review also found a proof truth gap: the retained real-provider shell proof used a two-file fixture while #891 requires the pinned Vector ten-file scope and macOS/Linux qualification. This SOR correction preserves the small-fixture smoke proof but no longer claims full required qualification from it.`

## Follow-ups / Deferred work
- `Refresh dependency and owner evidence, bind natively, and create issue goal before implementation`
- `Execute VPP and independent exact-head review, then native publication, terminal finish and separate cleanup`
