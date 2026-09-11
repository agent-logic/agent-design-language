# legacy-ready-intent-recovery

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

Task ID: issue-0843
Run ID: issue-0843
Version: v0.92.1
Title: [v0.92.1][TAIL-06.23][tooling] Recover retained ready intents without target identity
Branch: codex/843-legacy-ready-intent-recovery
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-11T15:33:13Z

Execution:
- Actor: `codex:/root`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `2026-09-11T15:33:13Z`
- End Time: `not_applicable`

## Summary

Legacy ready recovery preserves intent and binds authenticated target before dispatch.225 locked tests and fresh independent V3-F proof pass.

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
- Completion state: `pending_publication`
- Issue goal ref: `Active #835 expanded goal includes #843 dependency repair.`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/843/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/843/cards/sor.md`
- Tracked implementation artifacts: `none yet`
- Additional proof artifacts: `.csdlc/prepared/issues/824/814-pull-request-ready-reconciliation.json and validate-pr-ready-reconciliation.sh.`

## Actions taken
- `Created and reconciled issue #843 through native C-SDLC v3.`
- `Prepared issue-specific cards and focused validation plan.`
- `Implementation pending in bound worktree.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `implementation not started`
- Integration state: `worktree_only`
- Verification scope: `Full csdlc-v3 all-target test suite plus strict clippy, formatting, issue validator, operational CLI fake-transport proof, and diff hygiene.`
- Integration method used: `not_started`
- Verification performed:
  - `bash .csdlc/prepared/issues/824/validate-pr-ready-reconciliation.sh; cargo test --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 --all-targets --no-fail-fast; cargo clippy --manifest-path csdlc-v3/Cargo.toml -p csdlc-v3 --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml --all -- --check; git diff --check`
    `Validates the complete issue-branch artifact set locally; native publication and hosted CI remain pending.`
- Result: `not_run`

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
  - `cargo test --locked --manifest-path csdlc-v3/Cargo.toml; strict Clippy; V3-F validate and --negative`
    `Proves current source recovery and V3-F scope; does not authorize release.`
- Results:
  - `225 tests pass;45 focused remote tests pass;Clippy passes;15 negative cases pass.`

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
      - "Full C-SDLC v3 tests, focused ready matrix, strict clippy, formatting, native card validation, and diff hygiene passed."
  determinism:
    status: not_run
    replay_verified: false
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
- Replay result: `not_run`

## Artifact Verification
- Primary proof surface: `.csdlc/prepared/issues/824/validate-pr-ready-reconciliation.sh and focused C-SDLC v3 tests`
- Required artifacts present: `false`
- Artifact schema/version checks: `Native six-card validation passed; retained regression JSON parses and its required operation/attempt/readback fields validate.`
- Hash/byte-stability checks: `Stable operation, intent, response, readback, reconciliation, and recovery digests bind durable records to the exact request and head.`
- Missing/optional artifacts and rationale: `No live GitHub mutation receipt is required because issue validation is explicitly offline and mutation-free.`

## Decisions / Deviations
- `Use GitHub's typed GraphQL ready mutation because no supported REST ready endpoint exists in the bounded adapter.`
- `Treat only curl exit 22 and GraphQL errors as provider rejection; all uncertain transport outcomes require authenticated reconciliation.`

## Follow-ups / Deferred work
- `Publish only after fresh independent exact-head PASS.`
- `Shepherd required hosted checks green before declaring the issue goal complete.`
