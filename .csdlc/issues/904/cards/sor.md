# v0922-pair-multinode-experiment

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

Task ID: issue-0904
Run ID: issue-0904
Version: 0.92.2
Title: [v0.92.2][PLAT-PAIR] NVIDIA PAIR multi-node local-inference experiment
Branch: codex/904-v0922-pair-multinode-experiment
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:18:16.017452+00:00

Execution:
- Actor: `Planning #7 / sprint8_720`
- Model: `phi4-mini:latest; manifest sha256 78fad5d182a7c33065e153a5f8ba210754207ba9d91973f57dffa7f487363753; model blob sha256 3c168af1dea0a414299c7d9077e100ac763370e5a98b3c53801a958a47f0a5db; MIT license`
- Provider: `ollama through NVIDIA PAIR v0.1.1 loopback proxy and canonical provider reload sidecar`
- Start Time: `unknown; implementation began under active child goal`
- End Time: `implementation and first review repairs complete; renewed review and CI pending`

## Summary

PAIR works as a two-node request router. Decision REPAIR: the heterogeneous PAIR-cluster deployment materially outperformed the Mac-only baseline on larger/concurrent healthy work, failover and recovery succeeded, and the result does not isolate router overhead from RTX hardware. Startup/shared-host outliers and operational gaps remain.

## PVF Lane Truth
- Initial PVF lane: `provider`
- Planned PVF lane: `provider`
- Final PVF lane: `provider`
- Lane change reason: `No lane change; deterministic accounting plus actual two-node provider hardware integration were executed.`

## Issue Metrics Truth
- Expected runtime class: `Small deterministic CPU/filesystem checks plus bounded two-node local accelerator Runtime execution.`
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
- Completion state: `implementation_complete_renewed_review_pending`
- Issue goal ref: `Sprint #932 child #904 active implementation goal`
- Sprint goal ref: `issue-932; Sprint 6 setup and coordination`
- Goal metrics rollup ref: `.csdlc/evidence/904/goal-metrics.json (planned, absent until execution)`
- Validation planning prompt: `.csdlc/issues/904/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/904/cards/sor.md`
- Tracked implementation artifacts: `adl/tools/pair_experiment.py; adl/tools/test_pair_experiment.py; adl/tests/pair_provider.rs; .csdlc/evidence/904/PLAN.json; .csdlc/evidence/904/RESOURCE_SAMPLES.json; .csdlc/evidence/904/MEASUREMENTS.json; .csdlc/evidence/904/RESULTS.json; .csdlc/evidence/904/SUPPLEMENTAL_RESULTS.json; .csdlc/evidence/904/EXPERIMENT_REPORT.md; .csdlc/evidence/904/DECISION.md; .csdlc/evidence/904/IMPLEMENTATION_STATUS.md; .csdlc/evidence/904/providers.yaml; .csdlc/evidence/904/OBSERVED_RESOURCE_SAMPLE.json`
- Additional proof artifacts: `.csdlc/evidence/904/PLAN.json; .csdlc/evidence/904/RESOURCE_SAMPLES.json; .csdlc/evidence/904/MEASUREMENTS.json; .csdlc/evidence/904/RESULTS.json; .csdlc/evidence/904/SUPPLEMENTAL_RESULTS.json; .csdlc/evidence/904/EXPERIMENT_REPORT.md; .csdlc/evidence/904/DECISION.md; .csdlc/evidence/904/providers.yaml; .csdlc/evidence/904/OBSERVED_RESOURCE_SAMPLE.json`

## Actions taken
- `Verified native bound context, current issue and accepted #876; created child #904 implementation goal under #932.`
- `Verified signed PAIR v0.1.1 on Apple M4 Pro and RTX 3090 nodes with identical resident Phi-4 model bytes, canonical Ollama provider configuration, bounded resources and zero cloud spend.`
- `Executed the complete raw and Runtime matrix, actual remote-node loss and recovery, retained negative results and private-input hashes, and authored the REPAIR decision plus AWS fleet mapping.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation is in resolved Git metadata`
- Worktree-only paths remaining: `.csdlc/issues/904/cards; native bound setup only`
- Integration state: `worktree_only`
- Verification scope: `Actual two-node direct Ollama baseline, raw PAIR and canonical Runtime-through-PAIR routes; concurrency 1 and 2; controlled node loss and recovery; deterministic accounting and negative contracts`
- Integration method used: `Local commits in exact bound #904 worktree; no push or PR`
- Verification performed:
  - `git status --short --branch; git rev-parse HEAD`
    `Bound worktree source committed locally; no merged delivery`
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
  - `python3 adl/tools/test_pair_experiment.py; python3 adl/tools/pair_experiment.py --plan .csdlc/evidence/904/PLAN.json --measurements .csdlc/evidence/904/MEASUREMENTS.json --provider-definitions .csdlc/evidence/904/providers.yaml --resource-samples .csdlc/evidence/904/RESOURCE_SAMPLES.json; cargo test --manifest-path adl/Cargo.toml --test pair_provider pair_workflow_shape_is_bounded_and_concurrent -- --exact; cargo clippy --manifest-path adl/Cargo.toml --test pair_provider -- -D warnings; git diff --check`
    `Complete local two-node PAIR/Runtime qualification and deterministic replay established; independent renewed review and hosted CI remain pending.`
- Results:
  - `19 deterministic Python tests passed, including valid-but-wrong resource digest rejection; the complete 72-request accounting replay is byte-identical from tracked inputs; one exact Rust workflow-shape test passed; strict Clippy, JSON parsing and diff checks passed. Four actual Runtime receipts passed 24 of 24 outputs across healthy and node-loss states. Raw baseline/PAIR runs passed 48 of 48 Phi-4 outputs. Broker attribution proved healthy RTX selection, Mac failover and RTX recovery. First exact-head findings are repaired; renewed review and CI remain pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: local_complete_review_and_ci_pending
    checks_run:
      - "passed"
  determinism:
    status: local_accounting_replay_passed
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: tracked_packet_redacted_and_hash_bound
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: complete_local_experiment_packet_first_review_findings_repaired
    required_artifacts_present: yes; review and CI are lifecycle gates rather than missing experiment artifacts
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `18 Python tests including failover topology, concurrency, drift, incorrect output, failure, resource bound, collector deadline/body cap and CLI redaction cases`
- Fixtures or scripts used: `Deterministic Python accounting; production Rust Runtime live gate; official signed PAIR v0.1.1 Mac and Windows binaries; two exact-output prompts; three repetitions at concurrency 1 and 2; actual remote-node shutdown and restart`
- Replay verification (same inputs -> same artifacts/order): `Same plan, measurements and provider bytes reproduce the tracked accounting result`
- Ordering guarantees (sorting / tie-break rules used): `Matrix identities are unique and complete; normalized monotonic offsets preserve observed batch concurrency and node-loss ordering; summaries iterate fixed route/scenario/concurrency order`
- Artifact stability notes: `Tracked plan and measurement digests bind provider, corpus, model, node events and resource sample bytes; ignored private evidence is bound by SHA-256 in SUPPLEMENTAL_RESULTS.json`

## Security / Privacy Checks
- Secret leakage scan performed: `Tracked evidence scanned for private host names, addresses, raw node IDs and absolute user paths; none detected`
- Prompt / tool argument redaction verified: `Focused CLI failure test verifies sanitized stderr without supplied secret or absolute scratch path; no live-provider privacy proof.`
- Absolute path leakage check: `passed for tracked evidence packet`
- Sandbox / policy invariants preserved: `yes; two approved owned nodes, trusted LAN, bounded concurrency and zero cloud cost`

## Replay Artifacts
- Trace bundle path(s): `.adl/runs/904/accounting-tests.log`
- Run artifact root: `.csdlc/evidence/904`
- Replay command used for verification: `python3 adl/tools/pair_experiment.py --plan .csdlc/evidence/904/PLAN.json --measurements .csdlc/evidence/904/MEASUREMENTS.json --provider-definitions .csdlc/evidence/904/providers.yaml --resource-samples .csdlc/evidence/904/RESOURCE_SAMPLES.json`
- Replay result: `72 records admitted; deterministic summaries and comparisons emitted`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/904`
- Required artifacts present: `complete local experiment packet present`
- Artifact schema/version checks: `Native generation8 six-card values/renders/structures/digest validation passed; further field normalization is revalidated natively.`
- Hash/byte-stability checks: `Plan, tracked provider, tracked resource and ignored private-input digests checked; accounting rejects a valid-format wrong resource digest.`
- Missing/optional artifacts and rationale: `A bounded post-run residency/utilization snapshot is retained. No per-request power or utilization time series was collected, so no energy-efficiency claim is made.`

## Decisions / Deviations
- `#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main`
- `Decision REPAIR: retain PAIR for further development; require model residency, health/drain controls, security, observability and equivalent settings before production.`

## Follow-ups / Deferred work
- `Design a private AWS fleet experiment with model-preloaded GPU images, node admission/draining, interruption handling, request attribution and explicit autoscaling cost ceilings.`
- `Add reproducible context/generation controls and a reasoning-model response contract before qualifying DeepSeek through Runtime.`
