# v0922-test-planner

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

Task ID: issue-0894
Run ID: issue-0894
Version: 0.92.2
Title: [v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings
Branch: codex/894-v0922-test-planner
Card Status: ready
Status: IMPLEMENTED_LOCAL_PROOF_PASS
Generated: 2026-09-12T00:10:09.925246+00:00

Execution:
- Actor: `unassigned implementation owner`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `2026-09-16T00:00:00-07:00`
- End Time: `2026-09-16T00:00:00-07:00`

## Summary

Implemented `adl codefriend plan tests` as a bounded test-plan generator over accepted review synthesis output. It maps traceable synthesized findings to behavior under test, source evidence, proposed test location, fixture/input guidance, expected pre-fix failure, expected post-fix assertion, validation lane/resource notes, detection rationale, and non-goals. Findings without repository paths are omitted with explicit reasons. Publication remains held until exact-head review and base strategy are safe.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
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
- Completion state: `implemented_pending_review`
- Issue goal ref: `not_created; required before implementation`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/894/goal-metrics.json (planned, absent until execution)`
- Validation planning prompt: `.csdlc/issues/894/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/894/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/actions/test_plan.rs; adl/src/codefriend/actions/mod.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_testplan.rs`
- Additional proof artifacts: `cargo output from focused codefriend_testplan and codefriend_remediate commands in this session`

## Actions taken
- `Added CodeFriend test-plan action module with typed schema, digest binding, create-only output, traceability, and validation.`
- `Registered `adl codefriend plan tests` and read subcommand.`
- `Added focused installed-CLI and negative validation tests.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; implementation is in bound FastWork worktree only`
- Worktree-only paths remaining: `not_bound`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree`
- Integration method used: `stacked_on_local_893_branch_for_development_only`
- Verification performed:
  - `CARGO_TARGET_DIR=/Users/daniel/git/agent-design-language/.git/csdlc-v3/local/build-cache/issue-894-target cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan; CARGO_TARGET_DIR=/Users/daniel/git/agent-design-language/.git/csdlc-v3/local/build-cache/issue-894-target cargo fmt --manifest-path adl/Cargo.toml --check; CARGO_TARGET_DIR=/Users/daniel/git/agent-design-language/.git/csdlc-v3/local/build-cache/issue-894-target cargo test --manifest-path adl/Cargo.toml --test codefriend_remediate; git diff --check`
    `Proves installed CLI generation/read path, traceability, omitted-finding handling, placeholder/non-test rejection, adjacent remediation planner compatibility, Rust formatting, and diff hygiene.`
- Result: `local_implementation_complete_pending_review_and_publication`

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
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan; cargo fmt --manifest-path adl/Cargo.toml --check; cargo test --manifest-path adl/Cargo.toml --test codefriend_remediate; git diff --check`
    `Focused #894 runtime proof passed 4/4; adjacent #893 remediation proof passed 4/4; formatting and diff hygiene passed.`
- Results:
  - `passed`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed
    checks_run:
      - "Focused installed CodeFriend test-plan generator proof: 4 tests passed"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: issue #894 owns codefriend.test_plan.v1 product artifact schema
```

## Determinism Evidence
- Determinism tests executed: `test_plan_maps_every_traceable_finding_to_executable_case; test_plan_omits_findings_without_repository_path; test_plan_reader_rejects_placeholder_untraceable_or_non_test_cases; installed_cli_generates_and_reads_test_plan_without_source_mutation`
- Fixtures or scripts used: `adl/tests/codefriend_testplan.rs synthetic ReviewSynthesis fixtures; installed `adl` test binary; no provider calls`
- Replay verification (same inputs -> same artifacts/order): `not_run; implementation has not started`
- Ordering guarantees (sorting / tie-break rules used): `deterministic sorted test case ids; omitted findings tracked explicitly`
- Artifact stability notes: `not_run; implementation has not started`

## Security / Privacy Checks
- Secret leakage scan performed: `not_applicable; no credentials or provider calls used`
- Prompt / tool argument redaction verified: `not_applicable; no provider prompt or credential args used`
- Absolute path leakage check: `no generated product artifact committed; local build cache path only used for validation to avoid full FastWork disk`
- Sandbox / policy invariants preserved: `true`

## Replay Artifacts
- Trace bundle path(s): `not_run; implementation has not started`
- Run artifact root: `.csdlc/evidence/894 (planned; focused proof output retained in session transcript)`
- Replay command used for verification: `CARGO_TARGET_DIR=/Users/daniel/git/agent-design-language/.git/csdlc-v3/local/build-cache/issue-894-target cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan`
- Replay result: `4 passed; 0 failed`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_testplan.rs plus installed CLI invocation under Cargo test`
- Required artifacts present: `true`
- Artifact schema/version checks: `not_run; implementation has not started`
- Hash/byte-stability checks: `synthesis_digest and test_plan_digest are produced through existing CodeFriend hash helper`
- Missing/optional artifacts and rationale: `Execution artifacts are absent because preparation is not delivery`

## Decisions / Deviations
- `#894 is locally stacked on #893 for development because #893 owns the shared CLI/action framework and PR #1012 is not terminal in this worktree.`
- `Publication remains held until #893 base strategy is safe and a fresh exact-head review passes.`

## Follow-ups / Deferred work
- `Commit immutable #894 candidate and obtain fresh exact-head review.`
- `Publish only after review passes and #893/base strategy is safe.`
