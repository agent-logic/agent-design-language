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
Status: READY_FOR_PUBLICATION
Generated: 2026-09-11T17:24:00Z

Execution:
- Actor: `codex:/root`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `2026-09-11T17:24:00Z`
- End Time: `2026-09-11T18:24:00Z`

## Summary

Immutable-candidate external-review report, executable-review addendum, and
finding-disposition remediation record are retained. PR #853 is open for #833
publication and intentionally leaves #522 open for the complete TAIL-06 final
ledger.

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
- Completion state: `completed_with_follow_on`
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
- Variance note: `No precise session metrics were collected; unknown values are preserved instead of estimated.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Issue-local lifecycle cards under `.csdlc/issues/833/cards/`
- Tracked implementation artifacts: `V0921_EXTERNAL_REVIEW_HANDOFF.md`, `V0921_EXTERNAL_REVIEW_REPORT.md`, `V0921_EXECUTABLE_REVIEW_ADDENDUM.md`, `ISSUE_834_CANDIDATE_REVALIDATION.md`, and `V0921_EXTERNAL_REVIEW_REMEDIATION.md`
- Additional proof artifacts: `.csdlc/prepared/issues/833/validate-external-review-handoff.rb`, `.csdlc/prepared/issues/833/issue-818-candidate-current-supersession.json`, `.csdlc/prepared/issues/833/validate-issue-818-candidate-current-supersession.rb`, and `.csdlc/prepared/issues/833/test-issue-818-candidate-current-supersession.rb`

## Actions taken
- `Bound issue #833 in its FastWork worktree.`
- `Pinned canonical candidate 9c7e57d412d61898bd44ab00d53e31afbb779e5c.`
- `Corrected #522 closure dependency through authenticated native v3.`
- `Retained the failed external report as non-proving evidence and added executable-review/remediation disposition records.`
- `Preserved #522 as the final ledger and release-readiness gate.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `.csdlc/issues/833/**`, `.csdlc/prepared/issues/833/**`, `docs/milestones/v0.92.1/evidence/release/tail-05/**`
- Worktree-only paths remaining: `none after PR #853 merge; PR remains open before merge`
- Integration state: `pr_open`
- Verification scope: `issue worktree at PR #853 head`
- Integration method used: `committed branch publication through PR #853`
- Verification performed:
  - `ruby .csdlc/prepared/issues/833/validate-external-review-handoff.rb --all`
    `Proves the handoff/report candidate binding and seven negative cases.`
  - `ISSUE818_SUPERSESSION=.csdlc/prepared/issues/833/issue-818-candidate-current-supersession.json ruby .csdlc/prepared/issues/833/validate-issue-818-candidate-current-supersession.rb`
    `Proves #818's exact 17-row proposal set was approved by operator merge at PR #832, remains unchanged at the candidate, and does not close #522/#833.`
- Result: `pass`

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
  - `ruby -c .csdlc/prepared/issues/833/validate-external-review-handoff.rb`
    `Proves the focused validator syntax.`
  - `ruby .csdlc/prepared/issues/833/validate-external-review-handoff.rb --all`
    `Proves the handoff/report candidate binding and seven negative cases.`
  - `ISSUE818_SUPERSESSION=.csdlc/prepared/issues/833/issue-818-candidate-current-supersession.json ruby .csdlc/prepared/issues/833/validate-issue-818-candidate-current-supersession.rb`
    `Proves the #818 supersession receipt against live PR #832 / issue #818 truth and exact candidate ancestry.`
  - `ruby test-issue-818-candidate-current-supersession.rb`
    `Proves eight malformed, forged, or scope-widened #818 supersession receipts are rejected.`
- Results:
  - `pass`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: pass
    checks_run:
      - "focused handoff validator passed"
      - "#818 supersession validator passed"
      - "#818 supersession negative fixtures passed"
  determinism:
    status: pass
    replay_verified: not_applicable
    ordering_guarantees_verified: not_applicable
  security_privacy:
    status: pass
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: pass
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `Focused deterministic handoff validator and #818 supersession negative-fixture suite.`
- Fixtures or scripts used: `.csdlc/prepared/issues/833/validate-external-review-handoff.rb`; `.csdlc/prepared/issues/833/test-issue-818-candidate-current-supersession.rb`
- Replay verification (same inputs -> same artifacts/order): `not applicable to documentation handoff`
- Ordering guarantees (sorting / tie-break rules used): `not applicable`
- Artifact stability notes: `Candidate SHA is literal and immutable.`

## Security / Privacy Checks
- Secret leakage scan performed: `focused artifact review; no matched secret value is retained by validator output`
- Prompt / tool argument redaction verified: `not applicable`
- Absolute path leakage check: `focused artifact review; no unjustified absolute path added by this repair`
- Sandbox / policy invariants preserved: `yes`

## Replay Artifacts
- Trace bundle path(s): `not applicable`
- Run artifact root: `.csdlc/prepared/issues/833`
- Replay command used for verification: `not applicable`
- Replay result: `not applicable`

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.92.1/evidence/release/tail-05/V0921_EXTERNAL_REVIEW_HANDOFF.md`
- Required artifacts present: `handoff, failed report, executable addendum, #834 candidate revalidation, #818 supersession receipt, and remediation disposition records present`
- Artifact schema/version checks: `native six-card validation passed`
- Hash/byte-stability checks: `candidate commit resolves locally`
- Missing/optional artifacts and rationale: `none for #833 publication; #522 final ledger remains a separate open parent gate.`

## Decisions / Deviations
- `#522 stays open through #833 and resulting remediation.`
- `Candidate frozen before administrative #522 closure; remediation state and ledger closure are distinct.`
- `#833 publication may close #833 but must not close #522.`

## Follow-ups / Deferred work
- `Merge PR #853 after operator approval/review settlement to close #833.`
- `Continue #522 final finding-disposition ledger after #833 is settled.`
