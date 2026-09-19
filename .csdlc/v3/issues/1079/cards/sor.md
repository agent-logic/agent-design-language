# deepseek-openrouter-reasoning

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

Task ID: issue-1079
Run ID: issue-1079
Version: 1.0.5
Title: [v0.92.2][Runtime][OpenRouter] Honor reasoning effort for DeepSeek review turns
Branch: codex/1079-deepseek-openrouter-reasoning
Card Status: draft
Status: IN_PROGRESS
Generated: <timestamp>

Execution:
- Actor: `codex/root`
- Model: `GPT-6`
- Provider: `OpenAI`
- Start Time: `2026-09-18T22:00:00Z`
- End Time: `in_progress`

## Summary

Implemented OpenRouter reasoning-effort serialization, conflicting-control rejection, safe typed Runtime provider failures, docs, and focused tests. Conversation-level regressions, installed DeepSeek profile, live proof, final review, PR, CI, and merge remain in progress.

## PVF Lane Truth
- Initial PVF lane: `runtime_provider_integration`
- Planned PVF lane: `focused_provider_and_runtime_integration_plus_bounded_live_qualification`
- Final PVF lane: `in_progress`
- Lane change reason: `No material lane change.`

## Issue Metrics Truth
- Expected runtime class: `bounded_local_plus_live_provider`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `not_applicable_prepublication`
- Actual CI wait seconds: `not_applicable_prepublication`
- Budget source: `not_collected`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `Issue #1079 active Codex goal`
- Data-source confidence: `low`
- Estimate error percent: `unknown`
- Completion state: `implementation_and_validation_in_progress`
- Issue goal ref: `Issue #1079 active Codex goal`
- Sprint goal ref: `not_assigned`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/1079/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `false`
- Variance category: `not_applicable`
- Variance note: `Unknown estimates cannot produce a truthful variance calculation.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/1079/cards/sor.md`
- Tracked implementation artifacts: `OpenRouter codec/adapter/tests, Runtime ingress/control/conversation tests, provider inference docs, and six native lifecycle cards on the issue branch.`
- Additional proof artifacts: `Focused local validation results; installed proof pending.`

## Actions taken
- `Added normalized reasoning-effort consumption and OpenRouter reasoning.effort request serialization.`
- `Added safe allowlisted provider failure projection at Runtime conversation boundaries and focused coverage.`
- `Documented the contract and prepared reviewed release binaries for a separate DeepSeek provider profile.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; issue branch is not merged`
- Worktree-only paths remaining: `all issue-branch changes pending PR merge`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree and installed local Runtime proof surface`
- Integration method used: `issue branch commit; PR pending`
- Verification performed:
  - `git status --short --branch; git rev-parse HEAD; git diff --check`
    `Verifies exact branch identity and patch hygiene.`
- Result: `In progress; not merged.`

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
  - `cargo test --manifest-path adl-provider-core/Cargo.toml; cargo clippy --manifest-path adl-provider-core/Cargo.toml --all-targets -- -D warnings; focused adl-runtime-kernel tests; native csdlc validate`
    `Verifies request controls, rejection semantics, provider suite integrity, typed Runtime failures, and lifecycle card structure.`
- Results:
  - `Provider-core 122 tests and strict Clippy passed; Runtime helper regression passed; conversation-level regression and installed proof in progress. Two unrelated broader Runtime test failures and existing Runtime Clippy warnings reproduce on clean main.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: in_progress
    checks_run:
      - "Exact-head independent review found three actionable gaps; remediation and re-review are in progress."
  determinism:
    status: passed_local
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: passed_local
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: in_progress
    required_artifacts_present: false
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `Focused request-capture and invalid-control tests use deterministic local capture fixtures.`
- Fixtures or scripts used: `adl-provider-core request-capture fixtures; Runtime authenticated conversation fixture; installed local proof scripts pending.`
- Replay verification (same inputs -> same artifacts/order): `Focused local tests replayed successfully.`
- Ordering guarantees (sorting / tie-break rules used): `Provider candidate validation occurs before dispatch; conversation terminal results remain sequenced and idempotently retained.`
- Artifact stability notes: `Tracked behavior is committed; installed proof artifacts remain pending.`

## Security / Privacy Checks
- Secret leakage scan performed: `Focused public-result tests assert no secret-bearing provider error crosses the boundary.`
- Prompt / tool argument redaction verified: `Yes for focused Runtime results; live proof pending.`
- Absolute path leakage check: `Tracked cards use repository-relative proof references; machine-local invocation state is untracked.`
- Sandbox / policy invariants preserved: `Yes; tracked edits remain in the bound FastWork worktree and main is inspection-only.`

## Replay Artifacts
- Trace bundle path(s): `not_available_preproof`
- Run artifact root: `.csdlc/evidence/1079`
- Replay command used for verification: `Focused Cargo test commands recorded above.`
- Replay result: `passed for completed local lanes`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/1079 (pending)`
- Required artifacts present: `false; installed review/A2A proof and passing re-review remain pending`
- Artifact schema/version checks: `Native card validation pending after this edit.`
- Hash/byte-stability checks: `Exact tracked commit identity will be recorded before final review.`
- Missing/optional artifacts and rationale: `PR, CI, merge, and terminal artifacts do not exist before publication.`

## Decisions / Deviations
- `Kept Nexus and Nemotron on the shared OpenRouter definition.`
- `Use a separate DeepSeek provider definition with low reasoning, 8192 output tokens, and 180-second timeout.`

## Follow-ups / Deferred work
- `Install the reviewed candidate generation and run full-issue plus governed A2A proof.`
- `Fix all exact-head review findings, re-review, publish a draft PR, and shepherd CI.`
