# v0922-provider-recovery-qualification

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

Task ID: issue-0901
Run ID: issue-0901
Version: 0.92.2
Title: [v0.92.2][QUAL-PROVIDER] Execute real provider failure and recovery qualification
Branch: codex/901-v0922-provider-recovery-qualification
Card Status: draft
Status: NOT_STARTED
Generated: 2026-09-12T00:15:14.473149+00:00

Execution:
- Actor: `unassigned implementation owner`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

Preparation only for [v0.92.2][QUAL-PROVIDER] Execute real provider failure and recovery qualification. No implementation, acceptance proof, implementation review, publication, merge or closeout has run.

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
- Issue goal ref: `thread-goal 01a0924d-fbc0-7d21-b1ec-965c8a9562a4; active Sprint #931 objective explicitly includes child #901 qualification`
- Sprint goal ref: `Sprint 5 umbrella #931; child #901 provider failure and recovery qualification`
- Goal metrics rollup ref: `.csdlc/evidence/901/goal-metrics.json (planned, absent until execution)`
- Validation planning prompt: `.csdlc/issues/901/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/901/cards/sor.md`
- Tracked implementation artifacts: `none; implementation not started`
- Additional proof artifacts: `none; acceptance proof not started`

## Actions taken
- `Live source issue inspected for bounded preparation`
- `Six-card values prepared from registry 1.0.5`
- `Dependencies and local resource authority refreshed; binding and execution remain pending`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; native preparation is in resolved Git metadata`
- Worktree-only paths remaining: `not_bound`
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
- Sandbox / policy invariants preserved: `not_run; implementation has not started`

## Replay Artifacts
- Trace bundle path(s): `not_run; implementation has not started`
- Run artifact root: `.csdlc/evidence/901 (planned)`
- Replay command used for verification: `not_run; implementation has not started`
- Replay result: `not_run; implementation has not started`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/901 (planned)`
- Required artifacts present: `not_run; implementation has not started`
- Artifact schema/version checks: `not_run; implementation has not started`
- Hash/byte-stability checks: `not_run; implementation has not started`
- Missing/optional artifacts and rationale: `Execution artifacts are absent because preparation is not delivery`

## Decisions / Deviations
- `#855 is satisfied: PR #964 merged and issue #855 closed, delivering the registered-provider lifecycle. #852 is also accepted through merged PR #963 for any WSS failure-event evidence consumed. #851 remains dirty and unmerged in its separate registered worktree; its bytes are preserved and its paths are excluded.`
- `Use only task-owned local subprocesses and new #901 paths; preserve #851 and do not mutate shared provider services`

## Follow-ups / Deferred work
- `Bind natively, establish the child issue goal, then implement the bounded task-owned qualification harness`
- `Implement complete source contract, execute VPP, obtain independent exact-head review and use native publication/finish/clean`
