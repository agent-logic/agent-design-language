# codefriend-result-integrity

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

Task ID: issue-1162
Run ID: issue-1162
Version: 0.92.2
Title: [v0.92.2][TAIL-06][P1] Repair CodeFriend result integrity and website interoperability
Branch: codex/1162-codefriend-result-integrity
Card Status: draft
Status: NOT_STARTED
Generated: 2026-09-23T17:29:32.826544+00:00

Execution:
- Actor: `codex`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

No implementation or validation execution yet. This is the prepared output scaffold for Group B only.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `not_run`
- Lane change reason: `not_started`

## Issue Metrics Truth
- Expected runtime class: `not_started`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `No token budget requested; no paid operations authorized by this plan`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `not_started`
- Issue goal ref: `issue-1162`
- Sprint goal ref: `issue-921`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/1161/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `not_started`
- Variance analysis completed: `not_started`
- Variance category: `not_started`
- Variance note: `not_started`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/1161/cards/sor.md`
- Tracked implementation artifacts: `none`
- Additional proof artifacts: `none`

## Actions taken
- `not_started`
- `not_started`
- `not_started`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `none; not bound`
- Integration state: `not_started`
- Verification scope: `none; preparation only`
- Integration method used: `none`
- Verification performed:
  - `not_started`
    `not_started`
- Result: `not_started`

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
  - `none; see VPP for planned commands`
    `not_run`
- Results:
  - `not_run`

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
      - "No validation executed."
  determinism:
    status: not_run
    replay_verified: false
    ordering_guarantees_verified: false
  security_privacy:
    status: not_run
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: not_run
    required_artifacts_present: false
    schema_changes:
      present: false
      approved: false
```

## Determinism Evidence
- Determinism tests executed: `not_started`
- Fixtures or scripts used: `not_started`
- Replay verification (same inputs -> same artifacts/order): `not_started`
- Ordering guarantees (sorting / tie-break rules used): `not_started`
- Artifact stability notes: `not_started`

## Security / Privacy Checks
- Secret leakage scan performed: `not_started`
- Prompt / tool argument redaction verified: `not_started`
- Absolute path leakage check: `not_started`
- Sandbox / policy invariants preserved: `not_started`

## Replay Artifacts
- Trace bundle path(s): `not_started`
- Run artifact root: `.csdlc/evidence/1162`
- Replay command used for verification: `not_started`
- Replay result: `not_started`

## Artifact Verification
- Primary proof surface: `pending focused crash recovery and adapter regressions`
- Required artifacts present: `not_started`
- Artifact schema/version checks: `not_started`
- Hash/byte-stability checks: `not_started`
- Missing/optional artifacts and rationale: `not_started`

## Decisions / Deviations
- `Retain exactly seven assigned findings in one aggregate issue with two repository components.`
- `Preserve frozen review artifacts; reconcile current source before implementation.`

## Follow-ups / Deferred work
- `Bind #1162, implement both repository components, and retain the cross-repository disposition map.`
- `Run focused validation, obtain independent exact-head review, and publish without merge or deployment.`
