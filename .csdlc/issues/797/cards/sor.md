# issue-metadata

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

Task ID: issue-0797
Run ID: issue-0797
Version: v0.92.1
Title: [C-SDLC v3] Support existing-issue label and milestone updates
Branch: codex/797-issue-metadata
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-09T18:56:10.035633+00:00

Execution:
- Actor: `planning-5`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `not_collected`
- End Time: `not_finished`

## Summary

Implement label add/remove/replace, explicit milestone set/clear and assignee replacement with preserved omissions and durable exact reconciliation.

## PVF Lane Truth
- Initial PVF lane: `owner_binary`
- Planned PVF lane: `owner_binary`
- Final PVF lane: `owner_binary`
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
- Goal metrics source ref: `Current issue goal; no per-phase metrics snapshot`
- Data-source confidence: `unknown`
- Estimate error percent: `not_collected`
- Completion state: `implementation_validated_pending_publication`
- Issue goal ref: `Issue #797 session goal in current Codex task`
- Sprint goal ref: `not_applicable: standalone defect`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/797/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_collected`
- Variance note: `No estimated/actual pair collected; no precision inferred.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/797/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3/src/commands/remote/{mod,tests}.rs; src/main.rs; docs/csdlc-v3/CONTRACT.md and issue-edit.schema.json`
- Additional proof artifacts: `.csdlc/evidence/797/validation.json`

## Actions taken
- `Extended typed IssueEdit fields and explicit metadata request schema.`
- `Retained authenticated resolved target in intent before writes; exact readback and no edit replay on mismatch.`
- `Added fake-transport tests and corrected reviewer-identified identity/schema gaps.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none before merge`
- Worktree-only paths remaining: `Issue branch changes await PR integration.`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree`
- Integration method used: `PR publication pending`
- Verification performed:
  - `git diff --check`
    `Checked changed whitespace; no main integration claim.`
- Result: `Not merged`

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
  - `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::tests; cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands; cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --lib --bin csdlc -- -D warnings; native six-card validation`
    `Proves deterministic metadata semantics, exact readback and durable uncertainty handling, plus CLI integration.`
- Results:
  - `After merging current main: 38 remote-owner tests, 12 operational CLI tests, 13 publication/help tests and clippy passed. Native six-card validation follows template registry refresh.`

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
      - "63 focused native tests and clippy; no live metadata mutation proof"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: false
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
      approved: true
```

## Determinism Evidence
- Determinism tests executed: `Native remote metadata fake-transport tests.`
- Fixtures or scripts used: `csdlc-v3/src/commands/remote/tests.rs`
- Replay verification (same inputs -> same artifacts/order): `Uncertain response then exact read-only reconciliation and receipt replay tested.`
- Ordering guarantees (sorting / tie-break rules used): `No ordering behavior changed.`
- Artifact stability notes: `Original request digest remains stable when metadata fields are omitted.`

## Security / Privacy Checks
- Secret leakage scan performed: `Inspect diff and retained artifacts; no credentials retained.`
- Prompt / tool argument redaction verified: `Existing scoped credential resolution remains unchanged; fake transports only for mutation tests.`
- Absolute path leakage check: `Implementation/proof use repo-relative paths; native binding retains required worktree identity.`
- Sandbox / policy invariants preserved: `Bound FastWork worktree; no primary issue artifacts.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable: no runtime trace`
- Run artifact root: `.csdlc/evidence/797`
- Replay command used for verification: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::tests; cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands; cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --lib --bin csdlc -- -D warnings; native six-card validation`
- Replay result: `Retained target read-only reconciliation, receipt replay and drift rejection passed.`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/797/validation.json`
- Required artifacts present: `true`
- Artifact schema/version checks: `Typed JSON request and explicit operation tests; native cards validate.`
- Hash/byte-stability checks: `Legacy request serialization assertion preserves existing operation digests.`
- Missing/optional artifacts and rationale: `No live GitHub mutation or runtime proof required.`

## Decisions / Deviations
- `Explicit tagged operations distinguish omission and clear.`
- `Issue edits never replay PATCH after uncertain mismatch, even with explicit absence recovery.`

## Follow-ups / Deferred work
- `Final review and required CI before handoff.`
- `Merge and postmerge closeout remain asynchronous.`
