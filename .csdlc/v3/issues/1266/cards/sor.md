# recordless-merged-closeout-recovery

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

Task ID: issue-1266
Run ID: issue-1266
Version: 1.0.5
Title: [v0.93.1][C-SDLC v3][defect] Reconcile recordless merged closeout in external repositories
Branch: codex/1266-recordless-merged-closeout-recovery
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-10-13

Execution:
- Actor: `Worker #7`
- Model: `Codex`
- Provider: `OpenAI`
- Start Time: `not_collected`
- End Time: `in_progress`

## Summary

Repair authenticated replay of historical external closeout after the consumer checkout advances, retain the complete v0.93.1 census, and reconcile only proven positive cases.

## PVF Lane Truth
- Initial PVF lane: `csdlc-package-admission-and-live-closeout`
- Planned PVF lane: `csdlc-package-admission-and-live-closeout`
- Final PVF lane: `csdlc-package-admission-and-live-closeout`
- Lane change reason: `No lane change.`

## Issue Metrics Truth
- Expected runtime class: `focused Rust tests plus authenticated read-only consumer observations`
- Estimated elapsed seconds: `not_collected`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `not_collected`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `not_started`
- Actual CI wait seconds: `not_started`
- Budget source: `no explicit budget`
- Goal metrics data source: `session observations; exact timing and token totals were not retained`
- Goal metrics source ref: `issue #1266 session`
- Data-source confidence: `low`
- Estimate error percent: `not_applicable`
- Completion state: `implementation_validated_with_live_acceptance_blockers_pending_repair`
- Issue goal ref: `issue #1266 authenticated closeout recovery`
- Sprint goal ref: `v0.93.1 closeout`
- Goal metrics rollup ref: `active issue #1266 goal`
- Validation planning prompt: `.csdlc/v3/issues/1266/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `false`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No estimate baseline was retained.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/v3/issues/1266/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3/src/application/intent/terminal.rs; csdlc-v3/tests/installed_intent_commands.rs; .csdlc/evidence/1266/*.json`
- Additional proof artifacts: `.csdlc/evidence/1266/v0931-consumer-denominator.v1.json; .csdlc/evidence/1266/preexisting-terminal-receipts.v1.json; .csdlc/evidence/1266/consumer-authority-observations.v1.json; .csdlc/evidence/1266/complete-denominator-preview.v1.json; .csdlc/evidence/1266/infrastructure-reconciliation.v1.json`

## Actions taken
- `Retain the immutable closing PR head when replaying a valid legacy merged terminal receipt after the invoking checkout advances.`
- `Cross-check receipt schema, repository, issue, PR, head, publication repository and disposition against the digest-bound terminal state before authenticated replay.`
- `Reconciled both eligible Infrastructure issues canonically and preserved all 35 pre-existing terminal receipts byte-identically.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `All #1266 implementation and evidence changes await PR integration.`
- Integration state: `worktree_only`
- Verification scope: `origin/main...HEAD plus five sibling-repository v0.93.1 consumers`
- Integration method used: `PR publication pending`
- Verification performed:
  - `git diff --check origin/main...HEAD`
    `Checks branch diff hygiene without claiming merge.`
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
  - `cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_intent_commands external_pr_finish -- --nocapture; cargo check --manifest-path csdlc-v3/Cargo.toml --all-targets; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; git diff --check`
    `Proves external recordless closeout admission, fail-closed conflicts and receipt/state identity tamper rejection, replay after checkout advance, compilation, lint and diff hygiene.`
- Results:
  - `Focused external-pr tests passed 2/2 and the distinct external-publication first-close regression passed 1/1; formatting, strict clippy, JSON validation and diff hygiene passed. Native issue validation remains blocked by legacy SRP projection drift, and the complete live preview found retained receipt replay blockers.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: blocked
    checks_run:
      - "Focused external_pr_finish tests 2/2; external-publication first-close regression 1/1; cargo fmt check; strict cargo clippy; jq evidence parse; git diff check"
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
    status: partial
    required_artifacts_present: false
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `Synthetic terminal replay, receipt/state tamper matrix, remote-head drift rejection, and first external-close differing-head regression.`
- Fixtures or scripts used: `installed_intent_commands external_pr_finish fixture and issue_1092 external-publication fixture`
- Replay verification (same inputs -> same artifacts/order): `Synthetic installed external-consumer replay advances the checkout and requires expected_noop with byte-identical inventory.`
- Ordering guarantees (sorting / tie-break rules used): `Retained receipt classification and immutable head derive from the same authoritative receipt read before remote verification.`
- Artifact stability notes: `Existing receipt bytes remain unchanged; the preview failure is retained as adverse evidence rather than repaired by hand.`

## Security / Privacy Checks
- Secret leakage scan performed: `Focused diff and evidence inspection; no credential contents captured.`
- Prompt / tool argument redaction verified: `true`
- Absolute path leakage check: `Evidence uses repository identities and logical Git-common scope; lifecycle cards retain the bound worktree by schema.`
- Sandbox / policy invariants preserved: `true`

## Replay Artifacts
- Trace bundle path(s): `not_applicable`
- Run artifact root: `.csdlc/evidence/1266`
- Replay command used for verification: `cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_intent_commands external_pr_finish; cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_intent_commands issue_1092_installed_finish_accepts_explicit_authenticated_legacy_target`
- Replay result: `Complete live read-only preview blocked across 37 terminal-record cases: 36 intent_terminal_receipt_mismatch results and Infrastructure #27 intent_bound_checkout_mismatch; two no-closing-PR cases remain excluded.`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/1266/v0931-consumer-denominator.v1.json`
- Required artifacts present: `false`
- Artifact schema/version checks: `All five issue #1266 evidence JSON files parse; semantic card projection reports healthy after typed edits.`
- Hash/byte-stability checks: `All 35 pre-existing terminal receipt hashes match the pre-reconciliation ledger; no receipt was rewritten.`
- Missing/optional artifacts and rationale: `No CI or PR artifacts exist because native review/publication authority remains blocked.`

## Decisions / Deviations
- `Preserved all pre-existing terminal receipt bytes despite live replay failures.`
- `Did not touch the unrelated pre-existing issue #872 deletion, so native proof remains clean-candidate blocked.`

## Follow-ups / Deferred work
- `Obtain fresh exact-head independent review and record its disposition through the native review route.`
- `Publish the reviewed exact head and require green CI; do not merge without separate authority.`
