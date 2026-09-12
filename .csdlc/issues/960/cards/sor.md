# 960-shutdown-barrier

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

Task ID: issue-0960
Run ID: issue-0960
Version: 1.0.5
Title: Fix Runtime shutdown barrier acknowledgment race (v0.92.2)
Branch: codex/960-shutdown-barrier
Card Status: ready
Status: LOCAL_VALIDATION_PASSED
Generated: 2026-09-12T06:12:47.245031+00:00

Execution:
- Actor: `Worker #10 after explicit handoff from Planning #5`
- Model: `unknown`
- Provider: `OpenAI`
- Start Time: `See retained native issue960 preparation and diagnostic timestamps`
- End Time: `PR #962 open; CI and merge are not claimed complete`

## Summary

Implemented and locally validated event-specific local sink acknowledgment, fail-closed shutdown consumer, and bounded daemon stderr/exit diagnostics.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `none`

## Issue Metrics Truth
- Expected runtime class: `focused local runtime and CLI proof`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `Worker #10 issue-bound goal created after operator handoff; earlier implementation used the preserved Sprint 2 goal. No explicit token cap.`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `not_collected`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `published_required_ci_and_merge_pending`
- Issue goal ref: `Worker #10 task 01a0924e-4813-7101-ac5c-a1a9ac80f4f8 issue #960 goal: reviewed repair, focused proof, native publication and required CI; merge not authorized.`
- Sprint goal ref: `Sprint2 umbrella928`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/960/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `false`
- Variance category: `unknown`
- Variance note: `No invented aggregate metrics; per-command outcomes are retained separately.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/960/cards/sor.md`
- Tracked implementation artifacts: `adl/src/cli/observability.rs; adl/src/long_lived_agent.rs; adl/src/long_lived_agent/tests.rs; adl/tests/cli_smoke/agent.rs`
- Additional proof artifacts: `.csdlc/evidence/960/VALIDATION.md; LOCAL_PROOF.json; REPRODUCTION.json; sanitized logs/`

## Actions taken
- `unknown`
- `unknown`
- `unknown`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; bound worktree only`
- Worktree-only paths remaining: `All issue960 implementation and proof records`
- Integration state: `pr_open`
- Verification scope: `Focused local proof and independent source/record review; GitHub CI recorded separately for the actual head`
- Integration method used: `Native github-pr publication and authenticated publish readback after independent exact-head review.`
- Verification performed:
  - `git status --short --branch; git rev-parse HEAD`
    `PR base main, exact reviewed head and closing issue #960 confirmed by live readback.`
- Result: `PR #962 published through authenticated native github-pr; base main, Closes #960 verified. Not merged. Current CI observations are retained separately in native readback and local PR status evidence.`

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
  - `cargo test --manifest-path adl/Cargo.toml --lib observability::tests; focused shutdown consumer and cli_smoke tests; scoped cargo clippy`
    `Proved exact event acknowledgment, fail-closed sink handling, existing redaction and normal plus1ms-heartbeat CLI notice/disposition`
- Results:
  - `17 observability tests;1 actual barrier consumer test;3 CLI executions across normal,1ms-heartbeat and continuity/publication failure cases; scoped Clippy passed. See LOCAL_PROOF.json.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: pass
    checks_run:
      - "Focused local proof passed; current-head GitHub integration results must be inspected separately."
  determinism:
    status: pass
    replay_verified: false
    ordering_guarantees_verified: true
  security_privacy:
    status: pass
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: pass
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: false
```

## Determinism Evidence
- Determinism tests executed: `Explicit concurrent-writer interleaving; failed status path despite successful JSONL then repaired sink`
- Fixtures or scripts used: `Existing local runtime context and loopback CLI smoke fixtures; directory-at-file-path sink failure`
- Replay verification (same inputs -> same artifacts/order): `Not claimed; mutable monitor status and timestamps intentionally vary`
- Ordering guarantees (sorting / tie-break rules used): `Receipt reflects its own synchronous write under existing status lock; later heartbeat may replace snapshot`
- Artifact stability notes: `No unbounded history, durable remote delivery or power-loss guarantee claimed`

## Security / Privacy Checks
- Secret leakage scan performed: `Scoped source/record diff reviewed; existing observability secret-redaction tests passed; no credentials used`
- Prompt / tool argument redaction verified: `Existing observability redaction tests pass; no new event payload fields`
- Absolute path leakage check: `New proof uses repo-relative paths; bound-worktree identity is intentional native metadata`
- Sandbox / policy invariants preserved: `Bound worktree; no primary issue artifacts; no paid calls`

## Replay Artifacts
- Trace bundle path(s): `.adl/960-diagnosis; .adl/960-*.log`
- Run artifact root: `.adl`
- Replay command used for verification: `Focused commands in VALIDATION.md`
- Replay result: `Original CI cause unknown; supported-heartbeat diagnostic failure preserved; repaired heartbeat CLI passes`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/960/VALIDATION.md`
- Required artifacts present: `Sanitized reproduction, local proof logs, native cards and independent review present; live CI/terminal evidence tracked separately`
- Artifact schema/version checks: `Native six-card validation required after this update; no production schema changes`
- Hash/byte-stability checks: `Four source hashes and seven retained log hashes verified by independent review; unchanged source after publication records update.`
- Missing/optional artifacts and rationale: `No remote OTLP delivery or local broad coverage proof claimed; required CI remains a separate gate`

## Decisions / Deviations
- `unknown`
- `unknown`

## Follow-ups / Deferred work
- `unknown`
- `unknown`
