# greeting-recovery-local-model-boundary

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

Task ID: issue-0814
Run ID: issue-0814
Version: 1.0.5
Title: [v0.92.1][TAIL-06.09][runtime] Repair greeting recovery identity and local-model boundary
Branch: codex/814-greeting-recovery-local-model-boundary
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-09

Execution:
- Actor: `codex:fix_814_runtime`
- Model: `Codex`
- Provider: `OpenAI`
- Start Time: `not_collected`
- End Time: `not_collected`

## Summary

Implemented all three #520 findings owned by #814 and resolved both first-review gaps. Interrupted final greeting claims exhaust durably without redispatch; DomainWork/result identity stays logical and stable while adapter attempts use an explicit private execution identity; structural loopback origin validation and an executed two-endpoint fixture prove redirect denial.

## PVF Lane Truth
- Initial PVF lane: `runtime_focused`
- Planned PVF lane: `runtime_focused`
- Final PVF lane: `runtime_focused`
- Lane change reason: `none`

## Issue Metrics Truth
- Expected runtime class: `medium`
- Estimated elapsed seconds: `1800`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `12000`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `900`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `not_collected`
- Actual CI wait seconds: `not_collected`
- Budget source: `issue-local SPP estimate`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `not_collected`
- Data-source confidence: `not_collected`
- Estimate error percent: `not_collected`
- Completion state: `implementation_complete_review_pending`
- Issue goal ref: `codex-goal:issue-814`
- Sprint goal ref: `issue-520-review-remediation`
- Goal metrics rollup ref: `issue-522`
- Validation planning prompt: `.csdlc/issues/814/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown_metrics`
- Variance analysis completed: `not_applicable`
- Variance category: `not_collected`
- Variance note: `Actual elapsed and token telemetry were not collected; unknown values are not treated as zero variance.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/814/cards/sor.md`
- Tracked implementation artifacts: `adl-runtime-kernel/src/control.rs; adl-runtime-kernel/src/ingress.rs; adl-runtime/tests/shepherd_local_model.rs`
- Additional proof artifacts: `Typed #814 lifecycle cards and exact command results retained in this execution session.`

## Actions taken
- `Added a fail-closed max-attempt recovery boundary that persists terminal failure rather than claiming another greeting attempt after restart.`
- `Separated immutable logical DomainWork/result identity from private adapter-attempt execution identity and proved the completed ingress ledger retains only the stable logical key.`
- `Replaced prefix URL checks with structural loopback authority validation and executed the real Python runner against a local redirect source and target, proving denial before the target is reached.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; implementation is committed only after review preparation`
- Worktree-only paths remaining: `adl-runtime-kernel/src/control.rs; adl-runtime/tests/shepherd_local_model.rs; .csdlc/issues/814`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree`
- Integration method used: `pending reviewed PR publication`
- Verification performed:
  - `git status --short --branch; git diff --check`
    `Confirms the bounded issue branch and clean patch formatting before commit.`
- Result: `implementation complete in bound worktree; PR publication pending independent review`

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
  - `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib; cargo test --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model -- --include-ignored --skip real_local_model_smoke; cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib --tests -- -D warnings; cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --test shepherd_local_model -- -D warnings; bash adl/tools/run_owner_validation_lane.sh runtime; cargo fmt --check; git diff --check`
    `Proves the complete kernel unit denominator, local-model authority negatives, warning-free compilation, proportional owner contract, formatting, and patch hygiene.`
- Results:
  - `passed: 218 kernel tests, 2 Shepherd boundary tests including executed redirect denial, both Clippy targets, Runtime owner lane, formatting, and diff hygiene`

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
      - "218 kernel tests plus 2 Shepherd boundary tests and warning-denied Clippy"
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
- Determinism tests executed: `All 218 adl-runtime-kernel library tests and focused retry/URL negative fixtures.`
- Fixtures or scripts used: `In-memory Runtime provider fixture, durable dynamic-agent store fixture, and structural URL table fixtures.`
- Replay verification (same inputs -> same artifacts/order): `Restart fixture reloads the max-attempt record, persists terminal state, and proves no provider redispatch.`
- Ordering guarantees (sorting / tie-break rules used): `One logical greeting identity remains fixed while attempts advance monotonically and stop at the configured maximum.`
- Artifact stability notes: `No public schema changed; internal execution identity is separate from all logical identity projections.`

## Security / Privacy Checks
- Secret leakage scan performed: `Bounded diff inspection found no credentials or secret values.`
- Prompt / tool argument redaction verified: `No prompt or tool arguments are retained by the changed paths.`
- Absolute path leakage check: `Recorded cards and source diff use repository-relative artifact paths.`
- Sandbox / policy invariants preserved: `All tracked changes are confined to the bound issue worktree.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable; deterministic tests are the proof surface`
- Run artifact root: `not_applicable`
- Replay command used for verification: `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --lib admission_greeting`
- Replay result: `8 admission-greeting tests passed, including restart exhaustion and identity-drift rejection`

## Artifact Verification
- Primary proof surface: `adl-runtime-kernel/src/control.rs unit regressions and adl-runtime/tests/shepherd_local_model.rs boundary fixtures`
- Required artifacts present: `yes`
- Artifact schema/version checks: `No schema changes; existing durable store deserialization and six-card lifecycle validation pass.`
- Hash/byte-stability checks: `Stable idempotency/work/conversation/turn/correlation bindings are asserted across retry and restart.`
- Missing/optional artifacts and rationale: `Live Ollama GPU smoke remains intentionally ignored because #814 changes validation boundaries, not provider availability.`

## Decisions / Deviations
- `Internal execution attempts receive separate adapter identities so cached retryable failures cannot block a retry, while the logical work identity never changes.`
- `All HTTP redirects are denied in the local proof instead of attempting to classify redirect targets.`

## Follow-ups / Deferred work
- `Obtain fresh independent exact-head rereview of the two resolved findings and the full bounded diff.`
- `Publish a truthful PR with Closes #814 and observe required CI.`
