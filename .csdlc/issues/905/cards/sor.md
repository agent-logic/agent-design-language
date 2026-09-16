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
Status: review_changes_applied_re_review_pending
Generated: 2026-09-12T00:18:14.558347+00:00

Execution:
- Actor: `Planning #7; bounded #905 implementation under Sprint 6 #932`
- Model: `Qwen3.5:9b resident immutable GGUF target; equal-length temporary arm aliases differ only by draft_num_predict`
- Provider: `Ollama 0.32.14 through current Runtime provider registry and compatibility fallback`
- Start Time: `2026-09-15 bounded execution session`
- End Time: `Third-round review remediation implemented and locally validated; exact-head re-review pending`

## Summary

Current Runtime correctness and recovery remain proved; performance remains repair_inconclusive. PR #1004 alias ownership and failure-evidence defects now include pre-create namespace ownership and a finally-guaranteed, durably recorded model-removal attempt; exact-head re-review is pending.

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
- Data-source confidence: `high that this declared run is statistically inconclusive; low for any keep/retire performance conclusion`
- Estimate error percent: `unknown`
- Completion state: `review_changes_applied_re_review_pending`
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
- Variance category: `performance_regime_shift`
- Variance note: `Aggregate suggested +12.98% end-to-end and +17.29% decode, but end-to-end block wins were 1/4 with -5.96% median and decode wins 2/4 with -11.33% median; robustness gate failed.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/905/cards/sor.md`
- Tracked implementation artifacts: `adl/tools/vllm_qwen_speculative_decoding_benchmark.py; adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; adl/tools/issue905_runtime_speculative_retest.py; adl/tools/test_issue905_runtime_speculative_retest.py; .csdlc/evidence/905`
- Additional proof artifacts: `.csdlc/evidence/905/RUNTIME_RETEST.json; .csdlc/evidence/905/ATTEMPT_REGISTER.json`

## Actions taken
- `Added a bounded current Runtime harness that creates and verifies same-blob baseline/speculative aliases and reuses the exact Runtime identity for both arms.`
- `Executed four counterbalanced blocks with symmetric direct preload and excluded Runtime prewarm; all eight measured output pairs matched exactly.`
- `Rejected invalid draft configuration, verified healthy operator-selected ordinary generation, claimed cryptographically run-scoped alias namespaces before create, and guaranteed a durably recorded model-removal attempt despite ambiguous create or unrelated resource cleanup failures.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation remains in resolved Git metadata`
- Worktree-only paths remaining: `tracked issue implementation/evidence/cards pending publication`
- Integration state: `worktree_only`
- Verification scope: `Current Runtime paired correctness and performance with immutable same-model/tokenizer identity, same Runtime identity, counterbalanced symmetric warm procedure, invalid draft rejection and operator-selected ordinary recovery.`
- Integration method used: `Existing PR #1004 update pending renewed exact-head independent review`
- Verification performed:
  - `Pending renewed hosted CI; merge explicitly withheld by operator`
    `No merge or terminal integration claim; conservative repair_inconclusive disposition unchanged.`
- Result: `PR #1004 is open; third-round review changes are local pending commit/push, renewed review and CI. No merge authorized.`

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
  - `cd adl/tools && python3 -m unittest -v test_issue905_runtime_speculative_retest.py test_vllm_qwen_speculative_decoding_benchmark.py; python3 -m py_compile issue905_runtime_speculative_retest.py test_issue905_runtime_speculative_retest.py; git diff --check; native validate`
    `Establishes same-model/tokenizer correctness and bounded invalid-draft recovery, and prevents aggregate-only speed claims by recording per-block distributions and a robustness gate.`
- Results:
  - `PASS: 13 deterministic benchmark/accounting tests plus 8 deterministic alias-ownership/failure-reporting regressions, Python compilation, immutable model/tokenizer identity checks, eight-pair current Runtime proof, counterbalanced symmetric warm procedure, controlled invalid-draft rejection, healthy operator-selected ordinary recovery and diff hygiene. Exact-head re-review and renewed CI pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: pass_reviewed_performance_inconclusive
    checks_run:
      - "Eight of eight paired outputs matched exactly; performance robustness gate failed and is recorded as repair_inconclusive."
  determinism:
    status: pass for declared exact-output fixed-marker corpus
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: pass
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: complete_review_changes_applied_re_review_pending
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `21 deterministic tests: 13 benchmark/accounting cases and 8 alias-ownership/failure-reporting regressions; plus eight exact expected-marker paired Runtime comparisons`
- Fixtures or scripts used: `adl/tools/test_vllm_qwen_speculative_decoding_benchmark.py; adl/tools/issue905_runtime_speculative_retest.py`
- Replay verification (same inputs -> same artifacts/order): `runtime-run-14 completed with eight exact output pairs, counterbalanced symmetric prewarm and same Runtime identity`
- Ordering guarantees (sorting / tie-break rules used): `Four blocks alternate A/B then B/A; each arm switch receives a symmetric direct preload and excluded Runtime prewarm before measurement.`
- Artifact stability notes: `Tracked evidence omits machine-local output paths, records immutable model/tokenizer hashes, and retains all failed/superseded attempt classifications.`

## Security / Privacy Checks
- Secret leakage scan performed: `true`
- Prompt / tool argument redaction verified: `true`
- Absolute path leakage check: `pass`
- Sandbox / policy invariants preserved: `Bound issue worktree; no cloud spend or model download; isolated transient ports/processes; temporary same-model aliases only.`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/905/RUNTIME_RETEST.json; .csdlc/evidence/905/ATTEMPT_REGISTER.json; .csdlc/evidence/905/RETEST_STATUS.md; .csdlc/evidence/905/PR1004_REVIEW_REMEDIATION.json`
- Run artifact root: `.adl/runs/905`
- Replay command used for verification: `Use the declared command in VPP; temporary aliases use the same resident Qwen3.5:9b model with draft_num_predict 0 and 4.`
- Replay result: `pass at .adl/runs/905/runtime-run-14; tracked sanitized result at .csdlc/evidence/905/RUNTIME_RETEST.json`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/905/RUNTIME_RETEST.json`
- Required artifacts present: `true`
- Artifact schema/version checks: `JSON parse and native card validation`
- Hash/byte-stability checks: `Eight exact response pairs; tracked evidence JSON parse`
- Missing/optional artifacts and rationale: `Accepted/proposed draft-token counters are unavailable. Performance qualification is withheld because block-level robustness also failed; neither limitation is waived.`

## Decisions / Deviations
- `#864 CLOSED; PR #865 MERGED at f1c4e2a915c215797f0d2708cb8b0568f2b80b32, ancestor of selected main`
- `Repair the qualification benchmark and leave speculative decoding unqualified. Add stationarity criteria, a larger paired denominator and robust aggregate/confidence rule in follow-on work. No feature decommissioning or model deletion.`

## Follow-ups / Deferred work
- `Complete renewed exact-head independent review and native review/publication update; do not merge without operator instruction.`
- `Follow-on benchmark repair: explicit stabilization, larger paired blocks, per-block distribution and robust confidence/qualification threshold.`
