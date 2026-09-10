# pr-ready-reconciliation

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

Task ID: issue-0824
Run ID: issue-0824
Version: v0.92.1
Title: [v0.92.1][tooling] Reconcile native pull-request ready mutations
Branch: codex/824-pr-ready-reconciliation
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-10T00:00:00Z

Execution:
- Actor: `codex:/root/fix_815_cloud_auth`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `2026-09-10T00:00:00Z`
- End Time: `2026-09-10T22:41:24Z`

## Summary

Repaired native pull-request ready mutation routing with a narrow GraphQL operation, authenticated exact-target resolution, exact response/readback validation, and durable one-shot recovery consumption.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
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
- Estimate error percent: `unknown`
- Completion state: `implementation_validated_pending_publication`
- Issue goal ref: `Issue #824 session goal`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/824/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_collected`
- Variance note: `No estimated/actual metric pair was collected; variance is not inferred.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/824/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3 remote mutation state machine, operational adapter, focused unit and CLI tests, and issue #824 redacted regression packet/validator.`
- Additional proof artifacts: `.csdlc/prepared/issues/824/814-pull-request-ready-reconciliation.json and validate-pr-ready-reconciliation.sh.`

## Actions taken
- `Replaced the invalid ready REST endpoint with the typed markPullRequestReadyForReview GraphQL mutation.`
- `Resolved and retained exact PR node identity/head/draft state, validated the mutation response, and reconciled authenticated ready state.`
- `Prepared every deterministic pre-dispatch input before durably consuming the sole recovery, then added stale-head, wrong-PR, still-draft, already-ready, rejection, uncertain-transport, pre-dispatch-failure, success, and repeated-recovery tests.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; work remains on issue branch`
- Worktree-only paths remaining: `all issue #824 implementation and lifecycle paths`
- Integration state: `worktree_only`
- Verification scope: `Full csdlc-v3 all-target test suite plus strict clippy, formatting, issue validator, operational CLI fake-transport proof, and diff hygiene.`
- Integration method used: `pending native publication`
- Verification performed:
  - `bash .csdlc/prepared/issues/824/validate-pr-ready-reconciliation.sh; cargo test --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 --all-targets --no-fail-fast; cargo clippy --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml --all -- --check; git diff --check`
    `Validates the complete issue-branch artifact set locally; native publication and hosted CI remain pending.`
- Result: `PASS in the bound issue worktree; native publication and hosted CI pending`

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
  - `bash .csdlc/prepared/issues/824/validate-pr-ready-reconciliation.sh; cargo test --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 --all-targets --no-fail-fast; cargo clippy --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml --all -- --check; git diff --check`
    `Exercises the exact positive transition and every required negative without network or paid mutation, and checks the complete C-SDLC v3 target for regressions.`
- Results:
  - `PASS: 219 all-target tests, 3 focused ready tests, 1 adapter test, 1 operational CLI proof, strict clippy, formatting, and diff hygiene.`

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
      - "Full C-SDLC v3 tests, focused ready matrix, strict clippy, formatting, native card validation, and diff hygiene passed."
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: not_applicable
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
- Determinism tests executed: `Ready success, provider rejection, GraphQL error, stale head, wrong PR, still draft, already ready, uncertain transport, missing credential before dispatch, recovery success, and repeated recovery denial.`
- Fixtures or scripts used: `.csdlc/prepared/issues/824/validate-pr-ready-reconciliation.sh; local fake-curl operational CLI fixture; SequencedProcessAdapter unit fixtures.`
- Replay verification (same inputs -> same artifacts/order): `Repeated focused validator executions produce the same positive and negative classifications.`
- Ordering guarantees (sorting / tie-break rules used): `Not applicable; the state machine binds one exact PR target and serializes recovery consumption with create-new persistence.`
- Artifact stability notes: `Durable intent, recovery, mutation, and reconciliation records are bound by stable operation and intent digests.`

## Security / Privacy Checks
- Secret leakage scan performed: `Yes; the retained regression validator rejects secret-shaped fields and excludes credentials and raw response bodies.`
- Prompt / tool argument redaction verified: `No prompt payload is published; the fake CLI uses a fixture-only token and retains no credential value.`
- Absolute path leakage check: `Passed for published proof artifacts; the required lifecycle worktree binding remains the only machine-local path.`
- Sandbox / policy invariants preserved: `Yes; validation uses fake transport and performs no live provider or paid mutation.`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/prepared/issues/824/814-pull-request-ready-reconciliation.json`
- Run artifact root: `.csdlc/prepared/issues/824`
- Replay command used for verification: `bash .csdlc/prepared/issues/824/validate-pr-ready-reconciliation.sh`
- Replay result: `passed`

## Artifact Verification
- Primary proof surface: `.csdlc/prepared/issues/824/validate-pr-ready-reconciliation.sh and focused C-SDLC v3 tests`
- Required artifacts present: `true`
- Artifact schema/version checks: `Native six-card validation passed; retained regression JSON parses and its required operation/attempt/readback fields validate.`
- Hash/byte-stability checks: `Stable operation, intent, response, readback, reconciliation, and recovery digests bind durable records to the exact request and head.`
- Missing/optional artifacts and rationale: `No live GitHub mutation receipt is required because issue validation is explicitly offline and mutation-free.`

## Decisions / Deviations
- `Use GitHub's typed GraphQL ready mutation because no supported REST ready endpoint exists in the bounded adapter.`
- `Treat only curl exit 22 and GraphQL errors as provider rejection; all uncertain transport outcomes require authenticated reconciliation.`

## Follow-ups / Deferred work
- `Publish only after fresh independent exact-head PASS.`
- `Shepherd required hosted checks green before declaring the issue goal complete.`
