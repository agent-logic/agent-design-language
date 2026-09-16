# v0922-local-command-decomposition

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

Task ID: issue-0862
Run ID: issue-0862
Version: 0.92.2
Title: [v0.92.2][C-SDLC v3][refactor] Decompose the local command owner
Branch: codex/862-v0922-local-command-decomposition
Card Status: in_progress
Status: IN_PROGRESS
Generated: 2026-09-16T00:27:06.103043+00:00

Execution:
- Actor: `Codex Planning #5`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

Decomposed the C-SDLC v3 local owner into explicit acyclic responsibility modules while preserving the public facade and operational contracts. Independent review, CI, publication, merge and terminal closeout remain pending.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `not_run`
- Lane change reason: `not_run; implementation has not started`

## Issue Metrics Truth
- Expected runtime class: `not_run; implementation has not started`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `No operator issue token budget assigned; VPP estimates are planning only`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implementation_complete_review_pending`
- Issue goal ref: `active Codex goal for Sprint #933 / issue #862`
- Sprint goal ref: `v0.92.2 Sprint 7 coordination issue #933; descriptive only`
- Goal metrics rollup ref: `.csdlc/evidence/862/goal-metrics.json (planned, absent until execution)`
- Validation planning prompt: `.csdlc/issues/862/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No measured execution/estimate pair exists`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/862/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3/src/commands/local/*.rs; csdlc-v3/tests/local_module_decomposition.rs; .csdlc/evidence/862/decomposition-inventory.md`
- Additional proof artifacts: `The broader owner lane passed 22 authority tests and several guards before an unchanged test_card_prompt.sh 1.0.3 versus 1.0.5 baseline mismatch; no full-lane pass is claimed`

## Actions taken
- `Extracted planning, lifecycle, storage, worktree, transaction, context, card, issue, binding and routing owners from local/mod.rs`
- `Added a deterministic structural contract for the thin facade, explicit dependencies and one owner per responsibility`
- `Ran 151 focused tests, strict all-target clippy, formatting and diff checks successfully`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation is in resolved Git metadata`
- Worktree-only paths remaining: `all implementation remains on codex/862-v0922-local-command-decomposition until publication`
- Integration state: `review_pending`
- Verification scope: `issue #862 local command owner decomposition`
- Integration method used: `pending native review and publication`
- Verification performed:
  - `pending CI`
    `pending`
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
  - `Focused seven-suite cargo test invocation; strict all-target clippy; cargo fmt --check; git diff --check`
    `Proves local route, authority, CAS, digest, transaction, binding and terminal compatibility at the pre-review candidate`
- Results:
  - `151 passed, 0 failed; strict clippy, formatting and diff checks passed`

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
      - "not_run"
  determinism:
    status: passed
    replay_verified: not_run
    ordering_guarantees_verified: not_run
  security_privacy:
    status: not_run
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: not_run
    absolute_path_leakage_detected: not_run
  artifacts:
    status: local proof complete; independent proof pending
    required_artifacts_present: not_run
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `151 focused deterministic tests`
- Fixtures or scripts used: `not_run; implementation has not started`
- Replay verification (same inputs -> same artifacts/order): `not_run; implementation has not started`
- Ordering guarantees (sorting / tie-break rules used): `not_run; implementation has not started`
- Artifact stability notes: `Public exports and serialized types remain in local/mod.rs; focused behavioral suites are unchanged and green`

## Security / Privacy Checks
- Secret leakage scan performed: `not_run; implementation has not started`
- Prompt / tool argument redaction verified: `not_run; implementation has not started`
- Absolute path leakage check: `not_run; implementation has not started`
- Sandbox / policy invariants preserved: `not_run; implementation has not started`

## Replay Artifacts
- Trace bundle path(s): `not_run; implementation has not started`
- Run artifact root: `.csdlc/evidence/862`
- Replay command used for verification: `not_run; implementation has not started`
- Replay result: `not_run; implementation has not started`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/862/decomposition-inventory.md`
- Required artifacts present: `implementation and local evidence present; review and CI pending`
- Artifact schema/version checks: `local module structural contract passed`
- Hash/byte-stability checks: `not_run; implementation has not started`
- Missing/optional artifacts and rationale: `Execution artifacts are absent because preparation is not delivery`

## Decisions / Deviations
- `#864 (WP-01) is accepted via merged PR #865 at f1c4e2a915c215797f0d2708cb8b0568f2b80b32; live issue closure and PR merge were verified during preparation. All-69 creation/review gate is satisfied. Shared SIM ownership still requires reconciliation.`
- `No branch/worktree binding, shared binary replacement or live provider effect is authorized here`

## Follow-ups / Deferred work
- `Run independent exact-head review and resolve every actionable finding`
- `Publish through native C-SDLC v3, observe required CI, then await merge authority`
