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
- Model: `llama3.2:3b; blob sha256 dde5aa3fc5ffc17176b5e8bdc82f587b24b2678c6c66101bf7da77af9f7ccdff`
- Provider: `ollama through NVIDIA PAIR v0.1.1 loopback proxy`
- Start Time: `unknown; implementation began under active child goal`
- End Time: `in_progress`

## Summary

PAIR and the canonical Runtime route work on the approved local Apple M4 Pro node. After equal explicit prewarming, the small single-node comparison measured lower PAIR throughput: 0.9763x baseline at concurrency one and 0.7682x at concurrency two. This does not establish multi-node behavior. A second approved trusted node and controlled node-loss run are still required before the keep/repair/retire decision.

## PVF Lane Truth
- Initial PVF lane: `provider`
- Planned PVF lane: `provider`
- Final PVF lane: `provider`
- Lane change reason: `No lane change; executed only local deterministic subset of required provider experiment proof`

## Issue Metrics Truth
- Expected runtime class: `small CPU/filesystem and loopback transport; actual accelerator Runtime proof pending`
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
- Completion state: `in_progress`
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
- Tracked implementation artifacts: `adl/tools/pair_experiment.py; adl/tools/test_pair_experiment.py; adl/tests/pair_provider.rs; .csdlc/evidence/904/IMPLEMENTATION_STATUS.md; .csdlc/evidence/904/LOCAL_PREFLIGHT.json; .csdlc/evidence/904/local-runtime-single-node-c1.json; .csdlc/evidence/904/local-runtime-single-node-c2.json`
- Additional proof artifacts: `.csdlc/evidence/904/LOCAL_PREFLIGHT.json; .csdlc/evidence/904/local-runtime-single-node-c1.json; .csdlc/evidence/904/local-runtime-single-node-c2.json`

## Actions taken
- `Verified native bound context, current issue and accepted #876; created child #904 implementation goal under #932.`
- `Verified NVIDIA-signed PAIR v0.1.1, ran all workers, pinned Ollama 0.32.14 and llama3.2:3b, and completed same-corpus direct/raw PAIR measurements.`
- `Added and executed create-only content-redacted repeated Runtime live probes through canonical provider definitions at concurrency one and two; second-node pairing, node loss, resource proof and final decision remain pending.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation is in resolved Git metadata`
- Worktree-only paths remaining: `.csdlc/issues/904/cards; native bound setup only`
- Integration state: `worktree_only`
- Verification scope: `Actual single-node direct Ollama, raw PAIR and canonical Runtime-through-PAIR routes plus deterministic accounting and negative contracts; multi-node and node-loss gates remain pending`
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
  - `python3 adl/tools/test_pair_experiment.py; cargo test --manifest-path adl/Cargo.toml --test pair_provider pair_workflow_shape_is_bounded_and_concurrent -- --exact; cargo test --manifest-path adl/Cargo.toml --test pair_provider pair_actual_runtime_workflow -- --ignored --exact --nocapture with pinned ADL_PAIR inputs; cargo clippy --manifest-path adl/Cargo.toml --test pair_provider -- -D warnings; git diff --check`
    `Partial accounting and loopback transport correctness established; no actual PAIR/Runtime/hardware qualification.`
- Results:
  - `Sixteen deterministic Python tests passed. One deterministic Rust workflow-shape test passed. The ignored live Runtime test passed 12 of 12 exact-output requests across three batches each at concurrency one and two at source 29d71a537a0bbc4f08c329bf2e5b2f97d0d0ea48. Strict Clippy and diff checks passed. After equal explicit prewarming, the raw local preflight passed 24 of 24 requests and measured PAIR/baseline throughput ratios of 0.9763 at concurrency one and 0.7682 at concurrency two. Two-node routing, node loss, complete resource comparison, CI and final review remain unproved.`

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
    status: partial_local_tests_passed
    replay_verified: not_run
    ordering_guarantees_verified: not_run
  security_privacy:
    status: focused_local_cli_redaction_passed
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: not_run
    absolute_path_leakage_detected: not_run
  artifacts:
    status: partial_actual_single_node_and_runtime_proof_present
    required_artifacts_present: partial; hardware and Runtime experiment artifacts missing
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `14 tests including20receipt mutation subcases, matrix allocation bound, two-node/concurrency/context drift, raw loopback HTTP, deadline/body cap, explicit sampling/residency and redacted CLI failures.`
- Fixtures or scripts used: `python3 adl/tools/test_pair_experiment.py; cargo test --manifest-path adl/Cargo.toml --test pair_provider; operator-attended verified NVIDIA PAIR v0.1.1 service; exact two-request local corpus; three Runtime repetitions at concurrency one and two`
- Replay verification (same inputs -> same artifacts/order): `Local fixture only; no actual Runtime replay`
- Ordering guarantees (sorting / tie-break rules used): `not separately verified; partial implementation and local tests exist, full experiment remains unrun`
- Artifact stability notes: `not separately verified; partial implementation and local tests exist, full experiment remains unrun`

## Security / Privacy Checks
- Secret leakage scan performed: `not separately verified; partial implementation and local tests exist, full experiment remains unrun`
- Prompt / tool argument redaction verified: `Focused CLI failure test verifies sanitized stderr without supplied secret or absolute scratch path; no live-provider privacy proof.`
- Absolute path leakage check: `not separately verified; partial implementation and local tests exist, full experiment remains unrun`
- Sandbox / policy invariants preserved: `not separately verified; partial implementation and local tests exist, full experiment remains unrun`

## Replay Artifacts
- Trace bundle path(s): `.adl/runs/904/accounting-tests.log`
- Run artifact root: `.csdlc/evidence/904 (planned)`
- Replay command used for verification: `python3 adl/tools/test_pair_experiment.py`
- Replay result: `Deterministic summary equality fixture passes; actual Runtime replay not run`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/904 (planned)`
- Required artifacts present: `Local source/tests/checkpoint present; full experiment artifacts missing`
- Artifact schema/version checks: `Native generation8 six-card values/renders/structures/digest validation passed; further field normalization is revalidated natively.`
- Hash/byte-stability checks: `Plan/measurement digest and deterministic repeated-summary equality covered by local fixture; actual experiment stability not tested`
- Missing/optional artifacts and rationale: `Local source, focused test log and checkpoint evidence exist. Actual two-node/Runtime/node-loss traces, resource measurements, CI and disposition are missing because those executions have not occurred.`

## Decisions / Deviations
- `#876 CLOSED; PR #953 MERGED at b6d110c84e11253b392d0bb078f2fb33a36b9a0c, ancestor of selected main`
- `Planning #5 released903/904/905 setup ownership; native FastWork bind completed. Implementation, hardware execution, model loading/download and service mutations remain outside setup scope.`

## Follow-ups / Deferred work
- `Select two approved compatible nodes and PAIR/model/license/resource scope; establish fair bounded Runtime settings and actual collector integration.`
- `Complete actual raw/Runtime/node-loss experiment plus independent final review and requiredCI before closing publication.`
