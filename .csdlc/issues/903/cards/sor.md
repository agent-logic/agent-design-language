# v0922-mlx-metal-provider

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

Task ID: issue-0903
Run ID: issue-0903
Version: 0.92.2
Title: [v0.92.2][PLAT-MLX] Bounded MLX and Apple Metal provider adapter
Branch: codex/903-v0922-mlx-metal-provider
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:18:14.454312+00:00

Execution:
- Actor: `Planning #7: #903 implementation; sibling908 bounded design review and workflow test authoring`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not recorded; session is in progress`
- End Time: `in_progress`

## Summary

Bounded MLX adapter and actual canonical production-workflow Metal smoke passed. Independent source/evidence review found no new source defects; staleSOR/SRP corrected. Supplemental successful-review comparison not established, trials paused with failures retained. RequiredCI, publication, operator merge and native closeout remain.

## PVF Lane Truth
- Initial PVF lane: `provider`
- Planned PVF lane: `provider`
- Final PVF lane: `provider`
- Lane change reason: `unchanged`

## Issue Metrics Truth
- Expected runtime class: `local CPU fixtures plus observational Apple Metal GPU smoke`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `no operator token budget assigned`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `in_progress`
- Issue goal ref: `Active issue903 implementation goal under Sprint6 #932, created before source edits`
- Sprint goal ref: `issue-932; Sprint 6 setup and coordination`
- Goal metrics rollup ref: `active issue903 goal; no fabricated per-phase accounting`
- Validation planning prompt: `.csdlc/issues/903/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No comparable estimate/actual timing pair recorded for the complete issue`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/903/cards/sor.md`
- Tracked implementation artifacts: `adl/src/provider/mlx.rs; adl/src/provider/mlx/tests.rs; provider/substrate/profile/dispatch registration; adl/tests/mlx_provider.rs; .csdlc/evidence/903`
- Additional proof artifacts: `.csdlc/evidence/903/actual-hardware.json; local-validation.json; source-review.json`

## Actions taken
- `Implemented mlx adapter and narrow canonical dispatch/profile/substrate registration in903worktree; coordinated #855 extraction seam.`
- `12 protocol negatives,2 public/workflow fixtures,1 inherited reload regression passed; explicit real Metal workflow test also passed1/1 at3374315d3 with immutable local output/sidecar hashes. Strict Clippy passed.`
- `Bounded publication review passed at9e9803b66; unchanged source/hardware proof verified. Supplemental model trials paused after failed matched qualification; singleengine Qwen3.6 discovery qualifiedwithlimits. No paired-review acceleration claim.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation remains in resolved Git metadata`
- Worktree-only paths remaining: `All903 implementation, cards and evidence; not integrated`
- Integration state: `worktree_only`
- Verification scope: `bounded adapter and canonical production-workflow route`
- Integration method used: `pending native publication and operator merge`
- Verification performed:
  - `pending PR and operator merge`
    `not integrated`
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
  - `See .csdlc/evidence/903/local-validation.json for focused Rust and explicit actual Metal commands`
    `Local protocol, profile, reload and actual production-workflow Metal behavior proved; CI and successful-review benchmark outstanding.`
- Results:
  - `partial`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: partial
    checks_run:
      - "not_run"
  determinism:
    status: partial
    replay_verified: not_run
    ordering_guarantees_verified: not_run
  security_privacy:
    status: not_run
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: not_run
    absolute_path_leakage_detected: not_run
  artifacts:
    status: partial
    required_artifacts_present: not_run
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `12 deterministic protocol negatives,2 public/workflow fixtures,1 inherited reload regression; observational live smoke separately passed1/1`
- Fixtures or scripts used: `adl/src/provider/mlx/tests.rs; adl/tests/mlx_provider.rs; .csdlc/evidence/903/start_mlx_smoke_server.py`
- Replay verification (same inputs -> same artifacts/order): `not independently verified as replay proof`
- Ordering guarantees (sorting / tie-break rules used): `Canonical reload snapshot consumed before execute_sequential_with_provider_reload_handle invocation; no broader concurrency claim`
- Artifact stability notes: `Private original hardware receipt SHA256 retained; no deterministic generation claim`

## Security / Privacy Checks
- Secret leakage scan performed: `No broad repository scan; credential-free loopback smoke`
- Prompt / tool argument redaction verified: `Adapter protocol tests cover bounded generic errors; no broad repository leakage audit claimed`
- Absolute path leakage check: `Sanitized public hardware receipt replaces private absolute model path with pinned repository revision`
- Sandbox / policy invariants preserved: `Source changes remain in native bound issue worktree. Real Metal needed explicit sandbox escalation; isolated owned servers used bounded lifetime.`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/903/actual-hardware.json; private .adl/runs/903/actual-smoke receipt`
- Run artifact root: `.csdlc/evidence/903`
- Replay command used for verification: `Explicit ignored real-hardware test with approved local server and pinned ADL_MLX inputs; see adl/tests/mlx_provider.rs`
- Replay result: `single real production-workflow smoke passed; repeat/restart replay not separately claimed`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/903`
- Required artifacts present: `implementation and local evidence present; CI/publication pending`
- Artifact schema/version checks: `Native generation9 six-card values/render/structure/digest validation passed`
- Hash/byte-stability checks: `Actual receipt records exact sidecar, snapshot and output hashes; generation text is observational, not byte-stability proof`
- Missing/optional artifacts and rationale: `CI/PR not yet run; successful-review comparison unresolved. Actual Metal smoke now proved locally.`

## Decisions / Deviations
- `#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main`
- `Operator approved local execution and matching-model download. Real Metal workflow smoke passed. Additional successful-review comparison remains in progress; failed model reviews are non-proving for useful-review acceleration.`

## Follow-ups / Deferred work
- `Current finalmetadata/source review and native publication, then requiredCI. Supplemental comparison remains unresolved; do not hide failure or claim success.`
- `Updated independent exact-head review, CI, operator merge, native finish and exact cleanup`
