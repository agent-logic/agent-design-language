# v0922-speculative-decoding-retest

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

Task ID: issue-0905
Run ID: issue-0905
Version: 0.92.2
Title: [v0.92.2][SPEC-RETEST] Speculative-decoding requalification
Branch: codex/905-v0922-speculative-decoding-retest
Card Status: draft
Status: partial_implementation_environment_gated
Generated: 2026-09-12T00:18:14.558347+00:00

Execution:
- Actor: `Planning #7 / sprint8_909; bounded #905 implementation under #932`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

Partial retest accounting/output identity/attempt preservation implemented and independently reviewed at ec4c1c3ba2e8fa005a476f575280c0ca3cf867cc. Thirteen deterministic tests pass. Actual current Runtime baseline/speculative comparison and draft failure/fallback remain unexecuted; no keep/repair/retire conclusion or PR.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `deterministic local accounting and correctness negatives; real Runtime/hardware lane pending`
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
- Budget source: `no operator token budget assigned`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `incomplete_actual_runtime_proof_pending`
- Issue goal ref: `Active #905 full implementation and executed requalification goal under Sprint 6 #932`
- Sprint goal ref: `issue-932; Sprint 6 setup and coordination`
- Goal metrics rollup ref: `.csdlc/evidence/905/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/905/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No measured execution or estimate pair exists`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/905/cards/sor.md`
- Tracked implementation artifacts: `adl/tools/vllm_qwen_speculative_decoding_benchmark.py; adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; .csdlc/evidence/905/RETEST_STATUS.md`
- Additional proof artifacts: `none; acceptance proof not started`

## Actions taken
- `Verified bound native doctor and created child905 full goal; mapped actual kernel Ollama-only generation route and missing speculative controls`
- `Implemented fail-closed counters/summary/pair comparison and output identities; preserved all observed setup/generation attempts without raw exception text`
- `Independent source review found historical output overwrite; exclusive result/journal reservation corrected and13tests independently pass`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation remains in resolved Git metadata`
- Worktree-only paths remaining: `.csdlc/issues/905/cards; native bound setup only`
- Integration state: `worktree_only`
- Verification scope: `Bounded deterministic accounting/correctness, complete declared grid, output identity, exclusive artifact preservation and redacted failure journaling; raw fake engine tests do not prove inference.`
- Integration method used: `not_run; implementation has not started`
- Verification performed:
  - `not_run; implementation has not started`
    `not_run; implementation has not started`
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
  - `python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; git diff --check; native validate`
    `No implementation proof attempted`
- Results:
  - `13 deterministic tests passed locally and independently; diff hygiene passed. No hardware/engine/Runtime/CI proof.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: not_run
    checks_run:
      - "not_run"
  determinism:
    status: not_run
    replay_verified: not_run
    ordering_guarantees_verified: not_run
  security_privacy:
    status: not_run
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: not_run
    absolute_path_leakage_detected: not_run
  artifacts:
    status: not_run
    required_artifacts_present: not_run
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `not_run; implementation has not started`
- Fixtures or scripts used: `not_run; implementation has not started`
- Replay verification (same inputs -> same artifacts/order): `not_run; implementation has not started`
- Ordering guarantees (sorting / tie-break rules used): `not_run; implementation has not started`
- Artifact stability notes: `not_run; implementation has not started`

## Security / Privacy Checks
- Secret leakage scan performed: `not_run; implementation has not started`
- Prompt / tool argument redaction verified: `not_run; implementation has not started`
- Absolute path leakage check: `not_run; implementation has not started`
- Sandbox / policy invariants preserved: `Bound worktree only; no cloud/service/model/credential mutation; source changes confined to retest harness/tests`

## Replay Artifacts
- Trace bundle path(s): `not_run; implementation has not started`
- Run artifact root: `.adl/runs/905`
- Replay command used for verification: `not_run; implementation has not started`
- Replay result: `not_run; implementation has not started`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/905/RETEST_STATUS.md`
- Required artifacts present: `not_run; implementation has not started`
- Artifact schema/version checks: `not_run; implementation has not started`
- Hash/byte-stability checks: `not_run; implementation has not started`
- Missing/optional artifacts and rationale: `Actual Runtime comparison, hardware resource/cost and fallback evidence are required but unavailable; not optional or waived`

## Decisions / Deviations
- `#864 CLOSED; PR #865 MERGED at f1c4e2a915c215797f0d2708cb8b0568f2b80b32, ancestor of selected main`
- `Planning #5 released903/904/905 setup ownership; native FastWork bind completed. Implementation, hardware execution, model loading/download and service mutations remain outside setup scope.`

## Follow-ups / Deferred work
- `Select approved engine/model/tokenizer/hardware and actual current Runtime route for paired modes; no provider integration or acquisition silently authorized`
- `Execute comparable real routes and controlled draft failure/fallback before final disposition/closingPR`
