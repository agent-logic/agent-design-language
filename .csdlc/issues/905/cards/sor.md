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
Status: implementation_complete_review_pending
Generated: 2026-09-12T00:18:14.558347+00:00

Execution:
- Actor: `Planning #7 / sprint8_909; bounded #905 implementation under #932`
- Model: `Qwen3.5:9b resident target; same-model temporary baseline/speculative aliases`
- Provider: `Ollama 0.32.14 through current Runtime provider registry and compatibility fallback`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

Current Runtime requalification completed. Four exact-output baseline/speculative pairs passed, but speculative decoding was 18.07% slower cold-inclusive and 4.34% slower warm, with 15.81 versus 36.01 decode tokens/s. Invalid draft admission failed closed and ordinary fallback remained healthy. Recommendation: retire speculative decoding from the current Runtime qualification path; reconsider only with a newer engine/model and accepted/proposed-token telemetry.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `No lane change; the runtime lane now includes executed current Runtime comparison and fallback proof.`

## Issue Metrics Truth
- Expected runtime class: `local current Runtime integration`
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
- Goal metrics data source: `.csdlc/evidence/905/RUNTIME_RETEST.json`
- Goal metrics source ref: `.csdlc/evidence/905/RUNTIME_RETEST.json`
- Data-source confidence: `high for this declared local engine/model/hardware configuration`
- Estimate error percent: `unknown`
- Completion state: `implementation_complete_review_pending`
- Issue goal ref: `Active #905 full implementation and executed requalification goal under Sprint 6 #932`
- Sprint goal ref: `issue-932; Sprint 6 setup and coordination`
- Goal metrics rollup ref: `.csdlc/evidence/905/RUNTIME_RETEST.json`
- Validation planning prompt: `.csdlc/issues/905/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `true`
- Variance analysis completed: `true`
- Variance category: `measured_regression`
- Variance note: `Speculative arm 18.07% slower cold-inclusive and 4.34% slower warm; decode throughput 15.81 versus 36.01 tokens/s.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/905/cards/sor.md`
- Tracked implementation artifacts: `adl/tools/vllm_qwen_speculative_decoding_benchmark.py; adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; adl/tools/issue905_runtime_speculative_retest.py; .csdlc/evidence/905`
- Additional proof artifacts: `.csdlc/evidence/905/RUNTIME_RETEST.json; .csdlc/evidence/905/ATTEMPT_REGISTER.json`

## Actions taken
- `Added a bounded current Runtime integration harness using isolated Observatory/provider/guardian/kernel processes and resident Ollama model aliases.`
- `Executed four paired baseline/speculative conversations with deterministic expected markers and recorded latency, decode throughput, startup and attempt history.`
- `Rejected an invalid draft alias at admission and verified healthy ordinary fallback through the current Runtime path.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation remains in resolved Git metadata`
- Worktree-only paths remaining: `tracked issue implementation/evidence/cards pending publication`
- Integration state: `worktree_only`
- Verification scope: `Current Runtime paired correctness, latency, decode throughput, invalid draft admission and ordinary fallback; accepted/proposed counters unavailable and unclaimed.`
- Integration method used: `native PR publication after independent review`
- Verification performed:
  - `not_run; implementation has not started`
    `not_run; implementation has not started`
- Result: `not_integrated; independent review, PR, CI and merge pending`

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
  - `python3 adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; python3 -m py_compile adl/tools/issue905_runtime_speculative_retest.py; current Runtime command recorded in VPP; git diff --check; native validate`
    `Establishes current Runtime correctness, paired latency/throughput, bounded failure and fallback behavior, and an evidence-bound retire disposition.`
- Results:
  - `PASS: 13 deterministic accounting tests, Python compile, four-pair current Runtime proof, controlled invalid-draft rejection and healthy ordinary fallback; diff hygiene passed. Independent review and CI pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: pass_local_review_pending
    checks_run:
      - "Four of four paired outputs exactly matched expected markers."
  determinism:
    status: not_run
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: pass
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: complete_local_review_pending
    required_artifacts_present: true
    schema_changes:
      present: not_run
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `13 deterministic accounting tests plus exact expected-marker paired Runtime proof`
- Fixtures or scripts used: `adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; adl/tools/issue905_runtime_speculative_retest.py`
- Replay verification (same inputs -> same artifacts/order): `not_run; implementation has not started`
- Ordering guarantees (sorting / tie-break rules used): `Serialized baseline/speculative pairs with per-arm first-run warm classification`
- Artifact stability notes: `Tracked evidence omits machine-local absolute paths and preserves explicit engine/hardware/model identities and limitations.`

## Security / Privacy Checks
- Secret leakage scan performed: `true`
- Prompt / tool argument redaction verified: `true`
- Absolute path leakage check: `pass`
- Sandbox / policy invariants preserved: `Bound issue worktree; no cloud spend or model download; isolated transient ports/processes; temporary same-model aliases only.`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/905/RUNTIME_RETEST.json; .csdlc/evidence/905/ATTEMPT_REGISTER.json; .csdlc/evidence/905/RETEST_STATUS.md`
- Run artifact root: `.adl/runs/905`
- Replay command used for verification: `Use the declared command in VPP; temporary aliases use the same resident Qwen3.5:9b model with draft_num_predict 0 and 4.`
- Replay result: `pass at .adl/runs/905/runtime-run-09; tracked sanitized result at .csdlc/evidence/905/RUNTIME_RETEST.json`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/905/RUNTIME_RETEST.json`
- Required artifacts present: `true`
- Artifact schema/version checks: `JSON parse and native card validation`
- Hash/byte-stability checks: `Four exact response pairs; tracked evidence JSON parse`
- Missing/optional artifacts and rationale: `Ollama 0.32.14 does not expose accepted/proposed draft-token counters; the evidence makes no claim for them. All other required local proof is present.`

## Decisions / Deviations
- `#864 CLOSED; PR #865 MERGED at f1c4e2a915c215797f0d2708cb8b0568f2b80b32, ancestor of selected main`
- `Retire speculative decoding from the current Runtime qualification path because exact outputs were preserved but latency and decode throughput regressed. This is a qualification disposition, not service decommissioning or model deletion.`

## Follow-ups / Deferred work
- `Run independent exact-head review, native review, publication and CI; fix any actionable finding before merge.`
- `Any future reconsideration requires a separate issue using a newer engine/model combination with accepted/proposed draft-token telemetry.`
