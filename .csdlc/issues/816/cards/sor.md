# validation-integrity

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

Task ID: issue-0816
Run ID: issue-0816
Version: v0.92.1
Title: [v0.92.1][TAIL-06.11][quality] Repair redaction and hot-reload validation integrity
Branch: codex/816-validation-integrity
Card Status: in_progress
Status: IN_PROGRESS
Generated: 2026-09-09T20:50:00Z

Execution:
- Actor: `codex`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `2026-09-09T20:50:00Z`
- End Time: `not_finished`

## Summary

Replaced the vacuous OBS-B redaction grep with a bounded publication manifest, clean fixture, and eight negative classes; added read-only reload status and synchronized both cancellation tests on observed pending and cancelled transitions.

## PVF Lane Truth
- Initial PVF lane: `runtime_tests`
- Planned PVF lane: `runtime_tests`
- Final PVF lane: `runtime_tests`
- Lane change reason: `No lane change`

## Issue Metrics Truth
- Expected runtime class: `small`
- Estimated elapsed seconds: `not_collected`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `not_collected`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `not_collected`
- Actual CI wait seconds: `not_collected`
- Budget source: `No explicit token budget`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `not_collected`
- Data-source confidence: `unknown`
- Estimate error percent: `not_collected`
- Completion state: `implementation_validated_pending_review`
- Issue goal ref: `Issue #816 session goal pending bind`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/816/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_collected`
- Variance note: `No execution metrics yet.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/816/cards/sor.md`
- Tracked implementation artifacts: `.csdlc/prepared/issues/512/validate-obs-b-redaction.sh; .csdlc/prepared/issues/816; adl-runtime-kernel/src/config_reload.rs; adl-runtime/tests/config_reload.rs`
- Additional proof artifacts: `Terminal proof: 16 publication paths, 1 clean fixture, 8 negative fixtures; 8 config-reload integration tests; 2 kernel unit tests; 10 repeated redaction runs; 25 repeated runs of each cancellation test.`

## Actions taken
- `Defined an explicit Runtime/UI/evidence publication denominator and scanned the actual bytes for credential, path, private-key, bearer, provider-token, and unredacted payload classes.`
- `Added read-only candidate-observed, pending, and cancellation status to the hot-reload handle without changing reload decisions.`
- `Removed fixed sleeps from both cancellation tests and waited for the exact pending-to-cancelled transition.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `Issue branch changes await independent review and PR integration.`
- Integration state: `worktree_only`
- Verification scope: `Bound issue #816 worktree.`
- Integration method used: `PR publication pending`
- Verification performed:
  - `git diff --check`
    `No whitespace errors; no main integration claimed.`
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
  - `bash .csdlc/prepared/issues/512/validate-obs-b-redaction.sh; cargo test --locked --manifest-path adl-runtime/Cargo.toml --test config_reload; cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml config_reload --lib; repeated focused cancellation and redaction runs; cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test config_reload -- -D warnings; cargo fmt --check; git diff --check`
    `Proves non-vacuous redaction denominators and negative rejection plus causal cancellation after watcher receipt.`
- Results:
  - `Passed: 16 actual publication paths, 1 clean and 8 negative redaction fixtures, all 8 integration tests, 2 kernel unit tests, 10 redaction repetitions, and 25 repetitions for each cancellation case.`

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
      - "16 actual publication paths, 9 negative/positive fixture groups, 50 repeated cancellation runs"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: true
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
- Determinism tests executed: `25 repetitions each for revert and unreadable-file cancellation after explicit pending-state synchronization.`
- Fixtures or scripts used: `.csdlc/prepared/issues/816/obs-b-publication-paths.txt and fixtures/obs-b-publication-clean.json`
- Replay verification (same inputs -> same artifacts/order): `Repeated focused commands passed without fixed cancellation sleeps.`
- Ordering guarantees (sorting / tie-break rules used): `Tests observe the transient candidate as pending before performing the cancelling action and then observe the cancellation counter before asserting generation zero.`
- Artifact stability notes: `Manifest paths are repository-relative, unique, and denominator-checked.`

## Security / Privacy Checks
- Secret leakage scan performed: `Yes; validator reports only path and category on failure, never matching content.`
- Prompt / tool argument redaction verified: `Eight negative classes include provider credentials and unredacted provider payload fields.`
- Absolute path leakage check: `Manifest rejects absolute and parent-traversal paths; actual publication bytes are scanned for machine-local paths.`
- Sandbox / policy invariants preserved: `All tracked work is in the bound FastWork worktree.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable`
- Run artifact root: `.csdlc/evidence/816`
- Replay command used for verification: `bash .csdlc/prepared/issues/512/validate-obs-b-redaction.sh && cargo test --locked --manifest-path adl-runtime/Cargo.toml --test config_reload`
- Replay result: `pass`

## Artifact Verification
- Primary proof surface: `.csdlc/prepared/issues/512/validate-obs-b-redaction.sh and adl-runtime/tests/config_reload.rs`
- Required artifacts present: `true`
- Artifact schema/version checks: `Validator emits one machine-readable JSON summary on success.`
- Hash/byte-stability checks: `not_applicable: no immutable publication hash contract added`
- Missing/optional artifacts and rationale: `No live cloud or browser execution is required for these validation-integrity defects.`

## Decisions / Deviations
- `Used a read-only watcher status channel as the smallest deterministic testability seam.`
- `Kept the OBS-B validator in Bash and made its denominator explicit instead of adding another language runtime.`

## Follow-ups / Deferred work
- `Obtain independent exact-head review and fix every finding.`
- `Publish with Closes #816 and shepherd required CI.`
