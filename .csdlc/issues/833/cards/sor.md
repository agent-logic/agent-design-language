# external-review-immutable-candidate

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

Task ID: issue-0833
Run ID: issue-0833
Version: v0.92.1
Title: [v0.92.1][TAIL-06.18][review] Re-run external review at immutable candidate
Branch: codex/833-external-review-immutable-candidate
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-11T17:24:00Z

Execution:
- Actor: `codex:/root`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `2026-09-11T17:24:00Z`
- End Time: `not_finished`

## Summary

Immutable-candidate handoff drafted; validation and review pending.

## PVF Lane Truth
- Initial PVF lane: `documentation`
- Planned PVF lane: `documentation`
- Final PVF lane: `documentation`
- Lane change reason: `No lane change`

## Issue Metrics Truth
- Expected runtime class: `small`
- Estimated elapsed seconds: `not_collected`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `not_collected`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `not_collected`
- Actual CI wait seconds: `not_collected`
- Budget source: `No explicit token budget`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `not_collected`
- Data-source confidence: `unknown`
- Estimate error percent: `not_collected`
- Completion state: `in_progress`
- Issue goal ref: `Issue #833 session goal`
- Sprint goal ref: `#522`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/833/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_collected`
- Variance note: `Ongoing.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/833/cards/sor.md`
- Tracked implementation artifacts: `V0921_EXTERNAL_REVIEW_HANDOFF.md`
- Additional proof artifacts: `pending`

## Actions taken
- `Bound issue #833 in its FastWork worktree.`
- `Pinned canonical candidate 9c7e57d412d61898bd44ab00d53e31afbb779e5c.`
- `Corrected #522 closure dependency through authenticated native v3.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `issue #833 handoff and lifecycle files`
- Integration state: `worktree_only`
- Verification scope: `pending`
- Integration method used: `not_started`
- Verification performed:
  - `not_run`
    `not_run`
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
  - `not_run`
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
    status: in_progress
    checks_run:
      - "focused handoff validator pending rerun"
  determinism:
    status: in_progress
    replay_verified: not_applicable
    ordering_guarantees_verified: not_applicable
  security_privacy:
    status: in_progress
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: in_progress
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `Focused deterministic handoff validator.`
- Fixtures or scripts used: `.csdlc/prepared/issues/833/validate-external-review-handoff.rb`
- Replay verification (same inputs -> same artifacts/order): `not applicable to documentation handoff`
- Ordering guarantees (sorting / tie-break rules used): `not applicable`
- Artifact stability notes: `Candidate SHA is literal and immutable.`

## Security / Privacy Checks
- Secret leakage scan performed: `pending final scan`
- Prompt / tool argument redaction verified: `not applicable`
- Absolute path leakage check: `pending final scan`
- Sandbox / policy invariants preserved: `yes`

## Replay Artifacts
- Trace bundle path(s): `not applicable`
- Run artifact root: `.csdlc/prepared/issues/833`
- Replay command used for verification: `not applicable`
- Replay result: `not applicable`

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_HANDOFF.md`
- Required artifacts present: `handoff present; external report pending`
- Artifact schema/version checks: `native six-card validation passed`
- Hash/byte-stability checks: `candidate commit resolves locally`
- Missing/optional artifacts and rationale: `External report is a later phase of #833.`

## Decisions / Deviations
- `#522 stays open through #833 and resulting remediation.`
- `Candidate frozen before administrative #522 closure; remediation state and ledger closure are distinct.`

## Follow-ups / Deferred work
- `Obtain independent external review.`
- `Route every returned finding to #522.`
