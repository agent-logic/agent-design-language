# issue-844-native-pr-merge

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

Task ID: issue-0844
Run ID: issue-0844
Version: v0.92.1
Title: [v0.92.1][TAIL-06.24][csdlc] Add a first-class native v3 pull-request merge operation
Branch: codex/844-native-pr-merge
Card Status: ready
Status: in_progress
Generated: 2026-09-11T15:54:41.182312+00:00

Execution:
- Actor: `Planning #7`
- Model: `GPT-6`
- Provider: `OpenAI`
- Start Time: `unknown`
- End Time: `unknown`

## Summary

Implemented typed merge-only github-pr mutation, exact review and authenticated rule/check admission, durable intent/PR lock, reconciliation-only retry, and existing finish compatibility. No live merge performed.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `unchanged`

## Issue Metrics Truth
- Expected runtime class: `small`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `issue goal; no explicit token budget`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implementation validated; independent review and publication pending`
- Issue goal ref: `Planning #7 active issue844 session goal`
- Sprint goal ref: `not_applicable; issue-local goal`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/844/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `false`
- Variance category: `unknown`
- Variance note: `Complete actual telemetry not collected; no invented metrics.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/844/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3/src/commands/remote/merge.rs; adapter/CLI routes and regressions; docs/csdlc-v3/PULL_REQUEST_MERGE.md`
- Additional proof artifacts: `.csdlc/evidence/844`

## Actions taken
- `Implemented explicit merge-only request and authenticated complete eligibility.`
- `Bound durable intent and two-parent merged readback; excluded concurrent or repeated mutation.`
- `Repaired all four independent findings and passed targeted regressions; refreshed full suite and rereview pending.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; root main remains clean`
- Worktree-only paths remaining: `Issue844 tracked work awaiting PR publication`
- Integration state: `not_published`
- Verification scope: `bound issue844 worktree`
- Integration method used: `bound branch commits; merge awaits explicit target authorization`
- Verification performed:
  - `git status --short --branch; git diff --check`
    `Checked branch isolation and whitespace.`
- Result: `not merged`

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
  - `cargo test --manifest-path csdlc-v3/Cargo.toml --all-targets -- --test-threads=1; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check`
    `Tests prove successful merge,25 eligibility negatives,review/method/routes,locking,drift,replay and finish; clippy/fmt check native owner.`
- Results:
  - `231 native tests and strict clippy passed before final metadata; hosted CI and independent review pending.`

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
      - "native owner suite and focused merge tests"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: reviewed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: present
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: false
```

## Determinism Evidence
- Determinism tests executed: `csdlc-v3/src/commands/remote/tests/merge_cases.rs`
- Fixtures or scripts used: `csdlc-v3/src/commands/remote/tests/merge_cases.rs`
- Replay verification (same inputs -> same artifacts/order): `Uncertain and already-merged replay perform zero additional mutations.`
- Ordering guarantees (sorting / tie-break rules used): `fsynced intent precedes PUT; exact two ordered parents required.`
- Artifact stability notes: `Create-only receipts and stable typed digests; existing operation digests preserved.`

## Security / Privacy Checks
- Secret leakage scan performed: `Reviewed request/query/receipt fields; no credentials or provider payloads retained.`
- Prompt / tool argument redaction verified: `Credentials supplied only through shared child resolver.`
- Absolute path leakage check: `Proof paths repository-relative; generated worktree binding is explicit identity.`
- Sandbox / policy invariants preserved: `Edits only in bound worktree; native lifecycle; no live merge.`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/844`
- Run artifact root: `.csdlc/evidence/844`
- Replay command used for verification: `cargo test --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::tests::merge_cases -- --test-threads=1`
- Replay result: `PASS`

## Artifact Verification
- Primary proof surface: `csdlc-v3/src/commands/remote/merge.rs`
- Required artifacts present: `true`
- Artifact schema/version checks: `Typed serde request/method and native card validation`
- Hash/byte-stability checks: `Intent and reconciliation compare exact identities and existing payload digests.`
- Missing/optional artifacts and rationale: `Live merge intentionally not performed; deterministic transport proves operation.`

## Decisions / Deviations
- `Support only explicit merge commits; reject unsupported methods,queues or rules.`
- `REST has head CAS only; base race detected by poststate and never called reconciled success.`

## Follow-ups / Deferred work
- `Independent exact-head review, native publication and required CI.`
- `Merge authorization, terminal finish and cleanup remain separate after publication.`
