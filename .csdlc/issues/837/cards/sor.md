# active-boot-paths-control-plane-guidance

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

Task ID: issue-0837
Run ID: issue-0837
Version: v0.92.1
Title: [v0.92.1][TAIL-06.22][architecture] Publish active boot paths and retire stale control-plane guidance
Branch: codex/837-active-boot-paths-control-plane-guidance
Card Status: ready
Status: READY
Generated: 2026-09-10T23:00:00Z

Execution:
- Actor: `codex`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `2026-09-10T23:00:00Z`
- End Time: `2026-09-10T23:18:00Z`

## Summary

Published a source-backed subsystem boot-path inventory, moved the active editor handoff to native v3, repaired stale current tooling guidance, and added focused enforcement that preserves explicit rollback and historical contexts.

## PVF Lane Truth
- Initial PVF lane: `review_docs`
- Planned PVF lane: `review_docs`
- Final PVF lane: `review_docs`
- Lane change reason: `No lane change`

## Issue Metrics Truth
- Expected runtime class: `small`
- Estimated elapsed seconds: `1800`
- Actual elapsed seconds: `1080`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `180`
- Actual validation seconds: `8`
- Actual PR wait seconds: `not_collected`
- Actual CI wait seconds: `not_collected`
- Budget source: `No explicit token budget`
- Goal metrics data source: `Observed focused validation command`
- Goal metrics source ref: `.csdlc/prepared/issues/837/validate-active-boot-paths.sh`
- Data-source confidence: `high`
- Estimate error percent: `not_collected`
- Completion state: `implementation_complete_pending_exact_head_review`
- Issue goal ref: `Active issue #837 execution goal`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/837/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `true`
- Variance analysis completed: `complete`
- Variance category: `faster_than_estimate`
- Variance note: `Warm focused validation completed below the conservative estimate.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/837/cards/sor.md`
- Tracked implementation artifacts: `docs/tooling/ACTIVE_BOOT_PATHS.md; active tooling/editor guidance repairs; adl/tools/editor_action.sh; adl/tools/test_editor_action.sh; csdlc-v3/tests/command_manifest.rs; .csdlc/prepared/issues/837/validate-active-boot-paths.sh`
- Additional proof artifacts: `Native v3 lifecycle cards and transaction records for issue #837.`

## Actions taken
- `Derived and published an eight-row subsystem table from executable, source, selector, configuration, and historical boundaries.`
- `Replaced stale ordinary v2 guidance in active tooling, card, validator, and editor documentation and changed the copy-only editor adapter to emit native v3 commands.`
- `Added source-existence, single-entrypoint, active-doc, actual-help, and negative stale-guidance enforcement.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; publication pending`
- Worktree-only paths remaining: `Issue #837 lifecycle, inventory, documentation, adapter, test, and validator changes.`
- Integration state: `worktree_only`
- Verification scope: `Bound issue #837 FastWork worktree`
- Integration method used: `not_started; typed publication pending exact-head review`
- Verification performed:
  - `git status --short --branch`
    `Confirms changes remain isolated on codex/837-active-boot-paths-control-plane-guidance.`
- Result: `pending_publication`

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
  - `bash .csdlc/prepared/issues/837/validate-active-boot-paths.sh`
    `Runs 10 native v3 command-manifest tests, validates actual native command help and source paths, exercises editor command output, rejects stale v2-current fixtures, and checks diff hygiene.`
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
      - "Exactly one ordinary lifecycle anchor; all table source paths exist; scoped active docs contain no unbounded v2 route."
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
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `10 command-manifest tests plus editor adapter contract tests`
- Fixtures or scripts used: `.csdlc/prepared/issues/837/validate-active-boot-paths.sh`
- Replay verification (same inputs -> same artifacts/order): `Focused validator reran successfully.`
- Ordering guarantees (sorting / tie-break rules used): `Source inventory and guidance repair precede final contract validation.`
- Artifact stability notes: `The table uses repository-relative source paths and a single machine-readable lifecycle anchor.`

## Security / Privacy Checks
- Secret leakage scan performed: `not_applicable; no credentials or provider data involved`
- Prompt / tool argument redaction verified: `not_applicable`
- Absolute path leakage check: `passed for tracked implementation artifacts`
- Sandbox / policy invariants preserved: `All tracked writes are confined to the bound issue worktree.`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/prepared/issues/837/validate-active-boot-paths.sh`
- Run artifact root: `.csdlc/evidence/837`
- Replay command used for verification: `bash .csdlc/prepared/issues/837/validate-active-boot-paths.sh`
- Replay result: `passed`

## Artifact Verification
- Primary proof surface: `csdlc-v3/tests/command_manifest.rs`
- Required artifacts present: `true`
- Artifact schema/version checks: `Native six-card validation runs after this typed edit.`
- Hash/byte-stability checks: `Canonical selector and authenticated receipt are validated by the command-manifest suite.`
- Missing/optional artifacts and rationale: `No live Runtime or cloud proof is required for this documentation/control-plane contract.`

## Decisions / Deviations
- `Retained v2 source remains untouched.`
- `Historical evidence remains outside current-guidance enforcement.`

## Follow-ups / Deferred work
- `Obtain independent exact-head review.`
- `Publish with Closes #837 and Part of #522 after review passes.`
