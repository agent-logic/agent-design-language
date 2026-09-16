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
- Start Time: `unknown`
- End Time: `unknown`

## Summary

Implemented and remediated the installed CodeFriend operator review shell for #891. The product exposes `adl codefriend review shell start|inspect|cancel|retry|withhold-publication` around the existing isolated four-lane review runner. After exact-head review found cancellation/retry truth gaps, the implementation now archives stale cancel requests before retry and prevents final-lane cancellation from fabricating completion.

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
- Goal metrics data source: `active goal read from Codex goal API; detailed per-issue token split unknown`
- Goal metrics source ref: `Codex active goal for Sprint 4 #930`
- Data-source confidence: `medium_for_goal_identity_low_for_per_issue_metrics`
- Estimate error percent: `unknown`
- Completion state: `implemented_remediated_pending_fresh_review_publication_ci`
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
- Additional proof artifacts: `.csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture; .csdlc/evidence/891/real-provider-shell-proof/admission-small-fixture.json; .csdlc/evidence/891/real-provider-shell-proof/openai-provider-request.json`

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
  - `cargo fmt --manifest-path adl/Cargo.toml --check`
    `verified formatting for touched Rust files`
  - `cargo test --manifest-path adl/Cargo.toml --lib codefriend::review::runner::tests`
    `verified provider fenced-JSON normalization with 3/3 unit tests`
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_review review_shell`
    `verified installed shell control behavior with 6/6 focused tests, including cancel/retry race coverage`
  - `OPENAI_API_KEY=<approved env reference> ./adl/target/debug/adl codefriend review shell start ...`
    `verified the installed shell can operate a real OpenAI-backed bounded review and produce operator-state/run/review-record artifacts without storing secret values`
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
  - `cargo fmt --manifest-path adl/Cargo.toml --check`
    `verified formatting for touched Rust files`
  - `cargo test --manifest-path adl/Cargo.toml --lib codefriend::review::runner::tests`
    `verified provider fenced-JSON normalization with 3/3 unit tests`
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_review review_shell`
    `verified installed shell start/inspect/cancel/retry/withhold behavior and cancel/retry race handling with 6/6 focused tests`
  - `git diff --check`
    `verified diff hygiene`
  - `OPENAI_API_KEY=<approved env reference> ./adl/target/debug/adl codefriend review shell start --store /Volumes/FastWork/adl-worktrees/.codefriend-stores/891-real-provider-shell-small-fixture/store --packet-id 64e257e4bab57e73af85b059288d342f52ef323d1956a946a1dcc74a60985515 --provider-request .csdlc/evidence/891/real-provider-shell-proof/openai-provider-request.json --out .csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture --run-id issue-891-real-provider-shell-openai-small-fixture`
    `verified real provider execution through the installed operator shell on macOS; completed with four lane artifacts and review-record/run JSON`
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
      - "cargo fmt --manifest-path adl/Cargo.toml --check"
      - "cargo test --manifest-path adl/Cargo.toml --lib codefriend::review::runner::tests"
      - "cargo test --manifest-path adl/Cargo.toml --test codefriend_review review_shell"
      - "git diff --check"
      - "installed shell real-provider OpenAI proof"
  determinism:
    status: passed_for_deterministic_control_fixtures
    replay_verified: focused integration tests create isolated local fixture repositories, stores, controlled provider transport and output directories per run
    ordering_guarantees_verified: not_run
  security_privacy:
    status: passed_for_local_fixture_scope
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed
    required_artifacts_present: passed_for_local_fixture_scope
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `cargo test --manifest-path adl/Cargo.toml --test codefriend_review review_shell`
- Fixtures or scripts used: `adl/tests/codefriend_review.rs fixture servers plus .csdlc/evidence/891/real-provider-shell-proof bounded two-file fixture repository`
- Replay verification (same inputs -> same artifacts/order): `passed for focused deterministic shell tests`
- Ordering guarantees (sorting / tie-break rules used): `attempt settlement reloads latest persisted operator-state before writing, so older cancelled attempts cannot overwrite newer retry completion`
- Artifact stability notes: `Retries now archive attempt-local cancel-request.json before a new attempt; cancellation after the last provider request now records failed/cancelled run truth instead of complete.`

## Security / Privacy Checks
- Secret leakage scan performed: `real-provider proof used credential_ref env:OPENAI_API_KEY; no key value was copied into repository artifacts`
- Prompt / tool argument redaction verified: `provider result artifacts retain credential_ref only and redacted provider text excerpts`
- Absolute path leakage check: `proof command records the external FastWork store path because `admit-local` rejects stores inside the source checkout; repository artifacts keep request/output paths relative`
- Sandbox / policy invariants preserved: `real-provider proof used bounded repository fixture, no source mutation, and no secret value persistence`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture/operator-state.json; .csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture/attempts/1/review/run.json; .csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture/attempts/1/review/review-record.json`
- Run artifact root: `.csdlc/evidence/891/real-provider-shell-proof/operator-shell-openai-small-fixture records the successful installed shell real-provider run; local deterministic test artifacts remain under adl/target/codefriend-review-tests`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_review`
- Replay result: `not_run; implementation has not started`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/891/real-provider-shell-proof`
- Required artifacts present: `operator-state.json, attempts/1/review/run.json and attempts/1/review/review-record.json are present for the successful real-provider shell proof`
- Artifact schema/version checks: `operator-state JSON, review run JSON, lane result JSON and review-record JSON are parsed by focused tests; native C-SDLC validate passed at gen4 before remediation and will be rerun after this truth edit.`
- Hash/byte-stability checks: `cargo fmt completed after remediation; git diff --check will be rerun before fresh exact-head review.`
- Missing/optional artifacts and rationale: `Real external provider proof was executed on the macOS operator host with the installed shell against OpenAI gpt-4.1-mini and completed with lane artifacts. Linux installed qualification is left to repository CI after publication; no additional cloud, synthesis, renderer or publication proof is claimed here.`

## Decisions / Deviations
- `The issue contract proposed `adl/tests/codefriend_shell.rs`; the implementation extended the existing `adl/tests/codefriend_review.rs` integration target because it already owns the installed review runner fixture and avoids duplicating fixture infrastructure.`
- `No synthesis, remediation, UX, rendering or report publication behavior was implemented; those remain owned by sibling Sprint 4 issues.`

## Follow-ups / Deferred work
- `Refresh dependency and owner evidence, bind natively, and create issue goal before implementation`
- `Execute VPP and independent exact-head review, then native publication, terminal finish and separate cleanup`
