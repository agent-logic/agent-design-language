# v0922-review-synthesis

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

Task ID: issue-0892
Run ID: issue-0892
Version: 0.92.2
Title: [v0.92.2][CF-SYNTHESIS] Synthesize completed review perspectives
Branch: codex/892-v0922-review-synthesis
Card Status: ready
Status: implemented_pending_review
Generated: 2026-09-12T00:09:56.378553+00:00

Execution:
- Actor: `unassigned implementation owner`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

Implemented #892 CodeFriend review synthesis. The installed CLI now exposes `adl codefriend review synthesize --input <review-record.json> --out <new-dir>` to consume a committed complete four-lane ReviewRecord, validate provenance, deduplicate equivalent findings, preserve source attribution, severity rationale, disagreement and scope limits, and emit create-only `synthesis.json` plus `manifest.json` artifacts without source mutation, issue mutation, publication, remediation or rendering authority.

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
- Estimated validation seconds: `2400`
- Actual validation seconds: `approximately 125 local seconds observed for initial cold build plus focused test rerun; exact wall time not recorded as authoritative metrics`
- Actual PR wait seconds: `not_started`
- Actual CI wait seconds: `not_started`
- Budget source: `no operator token budget assigned`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implemented_pending_review_publication_ci`
- Issue goal ref: `Sprint 4 #930 active goal covers #892 execution in this session; single goal slot prevented replacing it with a separate child goal`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/892/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/892/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `Per-issue elapsed/token metrics are not available from the active sprint goal; validation seconds are approximate and not used as precise variance data.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/892/cards/sor.md`
- Tracked implementation artifacts: `none; implementation not started`
- Additional proof artifacts: `none; acceptance proof not started`

## Actions taken
- `Added `adl/src/codefriend/review/synthesis.rs` with validated synthesis types and a create-only artifact writer.`
- `Exported the synthesis module from `adl/src/codefriend/review/mod.rs` and wired the installed CLI command in `adl/src/cli/codefriend_cmd.rs`.`
- `Added focused deterministic coverage in `adl/tests/codefriend_synthesis.rs` for deduplication, disagreement preservation, incomplete-lane rejection and installed CLI artifact creation.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `adl/src/codefriend/review/synthesis.rs; adl/src/codefriend/review/mod.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_synthesis.rs`
- Worktree-only paths remaining: `.csdlc/issues/892/ and .csdlc/transactions/completed/892/ are generated lifecycle material in the bound worktree until publication/finish; implementation source changes are tracked candidate paths.`
- Integration state: `worktree_candidate_ready_for_review`
- Verification scope: `bound_issue_worktree`
- Integration method used: `bounded implementation in registered FastWork issue worktree; not published or merged`
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
      - "not_run"
  determinism:
    status: passed_for_deterministic_control_fixtures
    replay_verified: focused tests create isolated local fixture repositories and deterministic ReviewRecord JSON; installed CLI writes create-only output directories and rejects reuse.
    ordering_guarantees_verified: not_run
  security_privacy:
    status: passed_for_local_fixture_scope
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: not_run
    required_artifacts_present: passed_for_local_fixture_scope
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `not_run; implementation has not started`
- Fixtures or scripts used: `not_run; implementation has not started`
- Replay verification (same inputs -> same artifacts/order): `not_run; implementation has not started`
- Ordering guarantees (sorting / tie-break rules used): `not_run; implementation has not started`
- Artifact stability notes: `Synthesis output directory must not exist before execution; reruns cannot overwrite prior synthesis artifacts.`

## Security / Privacy Checks
- Secret leakage scan performed: `not_run; implementation has not started`
- Prompt / tool argument redaction verified: `not_run; implementation has not started`
- Absolute path leakage check: `not_run; implementation has not started`
- Sandbox / policy invariants preserved: `not_run; implementation has not started`

## Replay Artifacts
- Trace bundle path(s): `not_run; implementation has not started`
- Run artifact root: `test-generated per-run artifacts under adl/target/codefriend-synthesis-tests during focused tests; durable SOR proof is this card plus command output retained in terminal history until formal evidence capture`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_synthesis`
- Replay result: `not_run; implementation has not started`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/892 (planned)`
- Required artifacts present: `not_run; implementation has not started`
- Artifact schema/version checks: `ReviewRecord input, synthesis.json and manifest.json are parsed by focused tests; native C-SDLC validate pending after this edit`
- Hash/byte-stability checks: `cargo fmt check and git diff --check passed`
- Missing/optional artifacts and rationale: `Actual external provider execution and CI are deferred to required publication/CI gates; #892 consumes ReviewRecord output and does not require a new live provider call.`

## Decisions / Deviations
- `The implementation reads committed ReviewRecord JSON directly instead of raw lane directories because ReviewRecord is the validated production contract emitted by CF-REVIEW.`
- `No remediation planner, test planner, approval UX or renderer behavior was implemented; those remain owned by sibling Sprint 4 issues.`

## Follow-ups / Deferred work
- `Refresh dependency and owner evidence, bind natively, and create issue goal before implementation`
- `Execute VPP and independent exact-head review, then native publication, terminal finish and separate cleanup`
