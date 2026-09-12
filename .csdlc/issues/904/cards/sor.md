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
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

Partial accounting and bounded raw Ollama collector implemented and independently reviewed. All3review findings corrected; hardware, current Runtime comparison and final experiment disposition remain incomplete.

## PVF Lane Truth
- Initial PVF lane: `provider`
- Planned PVF lane: `provider`
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
- Completion state: `not_started`
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
- Tracked implementation artifacts: `adl/tools/pair_experiment.py; adl/tools/test_pair_experiment.py; .csdlc/evidence/904/IMPLEMENTATION_STATUS.md`
- Additional proof artifacts: `.csdlc/evidence/904/IMPLEMENTATION_STATUS.md`

## Actions taken
- `Verified native bound context, current issue and accepted #876; created child #904 implementation goal under #932.`
- `Implemented pair_experiment.py accounting component and focused deterministic tests; verified upstream PAIR0.1.1 identity and macOS support.`
- `Real collector integration, two approved nodes, model/license/resource choices and experiment remain incomplete.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation is in resolved Git metadata`
- Worktree-only paths remaining: `.csdlc/issues/904/cards; native bound setup only`
- Integration state: `not_started`
- Verification scope: `not_run`
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
  - `not_run`
    `No implementation proof attempted`
- Results:
  - `At8356bf4ee:14 local deterministic accounting and loopback transport tests passed; reviewer independently reran14 PASS. Native cards validate. No actual PAIR/Runtime/two-node/node-loss experiment, server cancellation proof or final disposition.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: local_partial_gate_passed_full_experiment_not_run
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
- Determinism tests executed: `14 tests including20receipt mutation subcases, matrix allocation bound, two-node/concurrency/context drift, raw loopback HTTP, deadline/body cap, explicit sampling/residency and redacted CLI failures.`
- Fixtures or scripts used: `not_run; implementation has not started`
- Replay verification (same inputs -> same artifacts/order): `not_run; implementation has not started`
- Ordering guarantees (sorting / tie-break rules used): `not_run; implementation has not started`
- Artifact stability notes: `not_run; implementation has not started`

## Security / Privacy Checks
- Secret leakage scan performed: `not_run; implementation has not started`
- Prompt / tool argument redaction verified: `not_run; implementation has not started`
- Absolute path leakage check: `not_run; implementation has not started`
- Sandbox / policy invariants preserved: `not_run; implementation has not started`

## Replay Artifacts
- Trace bundle path(s): `.adl/runs/904/accounting-tests.log`
- Run artifact root: `.csdlc/evidence/904 (planned)`
- Replay command used for verification: `not_run; implementation has not started`
- Replay result: `not_run; implementation has not started`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/904 (planned)`
- Required artifacts present: `not_run; implementation has not started`
- Artifact schema/version checks: `not_run; implementation has not started`
- Hash/byte-stability checks: `not_run; implementation has not started`
- Missing/optional artifacts and rationale: `No implementation or hardware run has started, so execution artifacts and metrics are not available.`

## Decisions / Deviations
- `#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main`
- `Planning #5 released903/904/905 setup ownership; native FastWork bind completed. Implementation, hardware execution, model loading/download and service mutations remain outside setup scope.`

## Follow-ups / Deferred work
- `Select two approved compatible nodes and PAIR/model/license/resource scope; establish fair bounded Runtime settings and actual collector integration.`
- `Complete actual raw/Runtime/node-loss experiment plus independent final review and requiredCI before closing publication.`
