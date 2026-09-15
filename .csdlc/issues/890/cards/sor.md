# v0922-four-perspective-review

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

Task ID: issue-0890
Run ID: issue-0890
Version: 0.92.2
Title: [v0.92.2][CF-REVIEW] Execute isolated four-perspective repository review
Branch: codex/890-v0922-four-perspective-review
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:09:56.378553+00:00

Execution:
- Actor: `codex:/root`
- Model: `gpt-5`
- Provider: `OpenAI`
- Start Time: `not_started`
- End Time: `in_progress`

## Summary

Implemented the bounded #890 CodeFriend four-perspective review runner and CLI dispatch in the bound FastWork worktree. Local controlled-provider validation passed; independent review, separately authorized registered OpenAI proof, PR publication, CI, merge and terminal finish remain pending.

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
- Budget source: `no operator token budget assigned`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implementation_local_controlled_provider_validation_passed_review_and_registered_openai_pending`
- Issue goal ref: `not_created; create issue-bound goal before implementation`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/890/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/890/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No measured execution or estimate pair exists`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/890/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/review/mod.rs; adl/src/codefriend/review/lanes.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/src/cli/codefriend_cmd.rs; adl/src/cli/usage.rs; adl/tests/codefriend_review.rs`
- Additional proof artifacts: `Provider fixture requests and run artifacts were written only under adl/target/codefriend-review-tests during test execution; no live provider credentials or external provider calls were used for the deterministic proof.`

## Actions taken
- `Source issue reviewed for native preparation`
- `Six-card values prepared through current native template fields`
- `Implementation and acceptance validation remain pending`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; primary main remains tracked clean and issue work is only in /Volumes/FastWork/adl-worktrees/adl-issue-890-v0922-four-perspective-review`
- Worktree-only paths remaining: `all #890 implementation/card changes remain in the bound issue worktree pending commit, review and publication`
- Integration state: `worktree_only`
- Verification scope: `not_run`
- Integration method used: `bound issue worktree local implementation`
- Verification performed:
  - `not_run; implementation has not started`
    `not_run; implementation has not started`
- Result: `local controlled-provider proof passed; not published`

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
  - `cargo fmt --manifest-path adl/Cargo.toml --check; cargo test --manifest-path adl/Cargo.toml --test codefriend_review --test codefriend_evidence --test codefriend_ingestion; git diff --check`
    `Covers installed CLI review run, four isolated provider-adapter calls, committed lane input/result artifacts, peer-input rejection, missing-evidence fail-closed behavior, durable evidence-store compatibility, and no source mutation in deterministic local fixtures.`
- Results:
  - `passed locally in the bound #890 worktree: codefriend_review 3/3, codefriend_evidence 11/11, codefriend_ingestion 10/10; fmt --check passed; git diff --check passed.`

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
      - "cargo test --manifest-path adl/Cargo.toml --test codefriend_review --test codefriend_evidence --test codefriend_ingestion passed"
  determinism:
    status: not_run
    replay_verified: true_for_local_controlled_provider
    ordering_guarantees_verified: not_run
  security_privacy:
    status: not_run
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: not_run
  artifacts:
    status: not_run
    required_artifacts_present: partial_prepublication
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
- Secret leakage scan performed: `local proof asserts persisted run/log/stdout artifacts do not retain the fixture credential; no real credential file was read`
- Prompt / tool argument redaction verified: `not_run; implementation has not started`
- Absolute path leakage check: `not_run; implementation has not started`
- Sandbox / policy invariants preserved: `not_run; implementation has not started`

## Replay Artifacts
- Trace bundle path(s): `not_run; implementation has not started`
- Run artifact root: `.csdlc/evidence/890 (planned)`
- Replay command used for verification: `not_run; implementation has not started`
- Replay result: `not_run; implementation has not started`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/890 (planned)`
- Required artifacts present: `true for deterministic local proof; registered OpenAI proof and independent exact-head review remain pending`
- Artifact schema/version checks: `not_run; implementation has not started`
- Hash/byte-stability checks: `not_run; implementation has not started`
- Missing/optional artifacts and rationale: `Registered OpenAI generated-review proof, independent review, PR, CI, merge and terminal receipts do not exist yet.`

## Decisions / Deviations
- `Wait for accepted merged output of #881, #855. Re-observe upstream closure, merged implementation PR and required contract/proof acceptance before binding; refresh this plan against those exact revisions. #864 WP-01 has accepted planning delivery via merged PR #865; other listed upstream issues remain open at preparation snapshot. Preparation is allowed; implementation is blocked.`
- `Preparation does not implement product behavior or bypass dependency gates`

## Follow-ups / Deferred work
- `Refresh dependency and owner evidence, bind natively, and create issue goal before implementation`
- `Execute VPP and independent exact-head review, then native publication, terminal finish and separate cleanup`
