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
Card Status: ready
Status: READY_FOR_PUBLICATION
Generated: 2026-09-16T00:27:06.103043+00:00

Execution:
- Actor: `Codex Planning #5`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `recorded in active issue session; exact elapsed metric unavailable`
- End Time: `2026-09-16T02:20:43.813294+00:00`

## Summary

Decomposed the C-SDLC v3 local owner into explicit acyclic responsibility modules while preserving public and serialized behavior. Local proof and independent exact-head review pass; publication, hosted CI, merge and terminal closeout remain pending.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `No lane change; tooling remains the selected and final local lane`

## Issue Metrics Truth
- Expected runtime class: `bounded_local`
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
- Completion state: `review_complete_publication_pending`
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
- Additional proof artifacts: `Independent final exact-head review at b0bcbe6cec: no actionable findings. The unchanged owner-lane template mismatch remains separately disclosed.`

## Actions taken
- `Extracted planning, lifecycle, storage, worktree, transaction, context, card, issue, binding and routing owners from local/mod.rs`
- `Added a deterministic structural contract for the thin facade, explicit dependencies and one owner per responsibility`
- `Ran 152 focused tests and strict checks, accepted three P2 review findings, and amended the candidate for re-review`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation is in resolved Git metadata`
- Worktree-only paths remaining: `all implementation remains on codex/862-v0922-local-command-decomposition until publication`
- Integration state: `publication_pending`
- Verification scope: `issue #862 local command owner decomposition`
- Integration method used: `native C-SDLC v3 review and publication`
- Verification performed:
  - `pending CI`
    `pending`
- Result: `not_integrated; publication pending`

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
  - `152 passed, 0 failed; strict clippy, formatting and diff checks passed`

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
      - "152 focused tests passed after all review fixes"
  determinism:
    status: passed
    replay_verified: structural rerun passed
    ordering_guarantees_verified: explicit dependency rank and transaction suites passed
  security_privacy:
    status: unchanged surface; no new exposure found in focused review
    secrets_leakage_detected: none observed
    prompt_or_tool_arg_leakage_detected: none observed
    absolute_path_leakage_detected: only the required bound-worktree lifecycle identity
  artifacts:
    status: local proof and independent review complete; hosted CI pending
    required_artifacts_present: implementation, lifecycle cards and decomposition inventory present; final review and CI pending
    schema_changes:
      present: false
      approved: not applicable: no schema change
```

## Determinism Evidence
- Determinism tests executed: `152 focused deterministic tests`
- Fixtures or scripts used: `Existing deterministic local command, CLI, transaction, foundation, binding and terminal fixtures; no extraction helper is retained`
- Replay verification (same inputs -> same artifacts/order): `Deterministic structural rerun passed with 2 tests`
- Ordering guarantees (sorting / tie-break rules used): `Explicit ranked module graph with canonical sibling-import enforcement; transaction ordering behavior remains covered by focused suites`
- Artifact stability notes: `Public exports and serialized types remain in local/mod.rs; focused behavioral suites are unchanged and green`

## Security / Privacy Checks
- Secret leakage scan performed: `No credential-bearing surface changed; diff and focused source review found no secret material`
- Prompt / tool argument redaction verified: `No prompt or credential handling changed; existing operational and transaction suites passed`
- Absolute path leakage check: `Reviewed: the only machine-local path is the canonical bound-worktree identity required by typed lifecycle records`
- Sandbox / policy invariants preserved: `All writes remained in the bound FastWork issue worktree through native v3 routes`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/862/decomposition-inventory.md and typed lifecycle records`
- Run artifact root: `.csdlc/evidence/862`
- Replay command used for verification: `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_module_decomposition`
- Replay result: `Structural contract rerun passed with alternate-import negatives; the full 152-test focused suite passed after fixes`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/862/decomposition-inventory.md`
- Required artifacts present: `implementation, lifecycle cards, decomposition inventory and independent review present; hosted CI and terminal artifacts pending`
- Artifact schema/version checks: `local module structural contract passed`
- Hash/byte-stability checks: `Unchanged contract suites cover lifecycle digests and serialized behavior; no separate exhaustive byte snapshot was added`
- Missing/optional artifacts and rationale: `Execution artifacts are absent because preparation is not delivery`

## Decisions / Deviations
- `#864 (WP-01) is accepted via merged PR #865 at f1c4e2a915c215797f0d2708cb8b0568f2b80b32; live issue closure and PR merge were verified during preparation. All-69 creation/review gate is satisfied. Shared SIM ownership still requires reconciliation.`
- `No branch/worktree binding, shared binary replacement or live provider effect is authorized here`

## Follow-ups / Deferred work
- `Run native review and publish a PR with Closes #862`
- `Observe required hosted CI, resolve any findings, then await explicit merge authority`
