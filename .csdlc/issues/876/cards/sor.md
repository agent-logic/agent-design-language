# issue-876-provider-definitions

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

Task ID: issue-0876
Run ID: issue-0876
Version: v0.92.2
Title: [v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions
Branch: codex/876-provider-definitions
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-12T02:15:09.413701+00:00

Execution:
- Actor: `fix_941_ci`
- Model: `GPT-6`
- Provider: `OpenAI`
- Start Time: `not_collected`
- End Time: `not_finished`

## Summary

Editable definitions expand profiles and validate concrete adapters before atomic promotion. Recursive neutral nested credentials are rejected; bounded diagnostics preserve last-known-good state. Twenty-five focused tests, mutation proof, clippy and formatting pass. Production review passed at e0dc293dca; card P2 corrected here, pending metadata review and PR CI. No merge or closeout claim.

## PVF Lane Truth
- Initial PVF lane: `provider`
- Planned PVF lane: `provider`
- Final PVF lane: `provider`
- Lane change reason: `none; bounded local provider integration and contract proof`

## Issue Metrics Truth
- Expected runtime class: `focused-production-integration`
- Estimated elapsed seconds: `not_collected`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `not_collected`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `not_collected`
- Actual CI wait seconds: `not_collected`
- Budget source: `No explicit token budget requested`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `Issue-bound goal service; no issue metrics export collected`
- Data-source confidence: `unknown`
- Estimate error percent: `not_collected`
- Completion state: `implementation_reviewed_card_review_pending`
- Issue goal ref: `Active #876 passing reviewed PR goal`
- Sprint goal ref: `Sprint 2 #928 execution goal`
- Goal metrics rollup ref: `not_collected; parent sprint goal owns rollup`
- Validation planning prompt: `.csdlc/issues/876/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `false`
- Variance category: `not_applicable`
- Variance note: `No complete estimated/actual metric pair; no invented precision.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/876/cards/sor.md`
- Tracked implementation artifacts: `adl/src/provider/reload.rs; adl/src/execute/tests.rs; adl/src/execute/tests/provider_definitions.rs; docs/providers/provider-profile-hot-loading.md`
- Additional proof artifacts: `.csdlc/evidence/876/IMPLEMENTATION_PROOF.md`

## Actions taken
- `Reused the existing reload owner, immutable snapshots and production runner.`
- `Moved adapter admission after profile expansion and before promotion; preserved credential-reference and shadow-path behavior.`
- `Executed production loopback/concurrency/LKG proof, negative matrix, compatibility tests and mutation check.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `None; primary main remains clean and inspection-only.`
- Worktree-only paths remaining: `All #876 implementation and card/evidence paths until PR merge.`
- Integration state: `worktree_only`
- Verification scope: `Bound issue worktree and controlled loopback endpoints.`
- Integration method used: `Issue branch commit, followed by native reviewed PR publication; merge pending.`
- Verification performed:
  - `git status --short --branch; git merge-base --is-ancestor for prerequisite merges`
    `Confirmed bound branch, clean primary main and accepted dependency ancestry.`
- Result: `Worktree implementation committed; PR not yet published.`

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
  - `cargo test --manifest-path adl/Cargo.toml --lib <filter>; filters provider_definitions, provider_reload, execute_sequential_retains_starting_provider_snapshot, provider_mod_profile; cargo clippy --manifest-path adl/Cargo.toml --lib -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml -- --check; git diff --check`
    `Proved real dispatch, invalid-input/LKG/redaction, reference admission, existing reload/in-flight/profile behavior and lint/format correctness.`
- Results:
  - `25 focused tests passed (3+7+1+14); clippy, formatting and whitespace passed. Current-head hosted CI pending.`

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
      - "25 focused tests and mutation rejection; see .csdlc/evidence/876/IMPLEMENTATION_PROOF.md"
  determinism:
    status: passed
    replay_verified: false
    ordering_guarantees_verified: true
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: false
```

## Determinism Evidence
- Determinism tests executed: `Three definition tests, seven reload tests, one in-flight and fourteen profile tests.`
- Fixtures or scripts used: `Production ProviderReloadOwner and runner with loopback HTTP request/release channels; provider reload invalid-input matrix.`
- Replay verification (same inputs -> same artifacts/order): `Artifact replay not claimed; immutable snapshot/digest retention and original-defect mutation were executed.`
- Ordering guarantees (sorting / tie-break rules used): `Old request arrival barrier precedes reload; new dispatch follows observed snapshot publication. Concurrent readers compare both providers.`
- Artifact stability notes: `Snapshot digest remains equal after invalid replacement; existing profile stability tests pass.`

## Security / Privacy Checks
- Secret leakage scan performed: `Synthetic credential marker absence asserted in initial-loader errors and watcher diagnostics; no real credential values loaded.`
- Prompt / tool argument redaction verified: `Candidate input details excluded from rejection text; no broad provider-prompt logging change claimed.`
- Absolute path leakage check: `SOR uses repository-relative references. Raw local compiler/backtrace logs retain host paths as diagnostic provenance, not provider output.`
- Sandbox / policy invariants preserved: `Bound FastWork worktree only; no paid provider calls, process launches, deployment or main writes.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable; no replay trace-bundle claim`
- Run artifact root: `.csdlc/evidence/876`
- Replay command used for verification: `not_applicable; mutation regression proof is not artifact replay`
- Replay result: `not_claimed`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/876/IMPLEMENTATION_PROOF.md`
- Required artifacts present: `Implementation, tests, docs, all six native cards and preserved proof logs present.`
- Artifact schema/version checks: `Existing sidecar schema unchanged; native six-card values/render/structure validation passed.`
- Hash/byte-stability checks: `Invalid reload retains generation and digest; existing profile projection stability tests pass.`
- Missing/optional artifacts and rationale: `No hosted qualification, paid inference or replay trace required for this local contract.`

## Decisions / Deviations
- `Used existing adapter constructors for admission; verified they retain references without completing inference or writing shadow evidence.`
- `Preserved initial macOS fixture failure, clippy warning and shadow-filename rejection with corrections; preserved card P2.`

## Follow-ups / Deferred work
- `Obtain metadata-only exact-head review, native publication and required CI.`
- `Dynamic agent lifecycle remains separately owned by #855; no implementation in this issue.`
