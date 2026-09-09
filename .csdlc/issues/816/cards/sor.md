# validation-integrity

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

Task ID: issue-0816
Run ID: issue-0816
Version: v0.92.1
Title: [v0.92.1][TAIL-06.11][quality] Repair redaction and hot-reload validation integrity
Branch: codex/816-validation-integrity
Card Status: in_progress
Status: IN_PROGRESS
Generated: 2026-09-09T20:50:00Z

Execution:
- Actor: `codex`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `2026-09-09T20:50:00Z`
- End Time: `not_finished`

## Summary

OBS-B executes production project_v1 serialization and scans its JSON plus every included UI/evidence publication. Manifest completeness is derived from all tracked #512 evidence and the SRP/SOR declarations; non-publication exclusions require explicit reviewed reasons. Every included JSON receives structural duplicate-key and sensitive-field validation, non-JSON publications receive token/path and sensitive-assignment checks, and eleven negative classes include manifest-path leakage and omitted declared evidence. Hot-reload cancellation remains causally synchronized.

## PVF Lane Truth
- Initial PVF lane: `runtime_tests`
- Planned PVF lane: `runtime_tests`
- Final PVF lane: `runtime_tests`
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
- Completion state: `implementation_validated_pending_review`
- Issue goal ref: `Issue #816 session goal pending bind`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/816/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_collected`
- Variance note: `No execution metrics yet.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/816/cards/sor.md`
- Tracked implementation artifacts: `.csdlc/prepared/issues/512/validate-obs-b-redaction.sh; .csdlc/prepared/issues/816; adl-runtime/tests/distributed_projection.rs; adl-runtime-kernel/src/config_reload.rs; adl-runtime/tests/config_reload.rs`
- Additional proof artifacts: `Terminal proof: production project_v1 JSON, 17 included UI/evidence publications, 12 explicit non-publication classifications, 1 clean fixture, 11 negative fixtures including manifest-path leakage and omitted-declared-artifact rejection; 8 config-reload integration tests; 2 kernel unit tests; repeated redaction and cancellation runs.`

## Actions taken
- `Execute the production project_v1 serialization path and structurally scan its emitted Runtime JSON instead of counting Rust source as publication output.`
- `Parse publication JSON with duplicate-key rejection and require each provider_payload, prompt, output, and tool_arguments value to be exactly [REDACTED].`
- `Derived the publication denominator from tracked #512 evidence and declared SRP/SOR scope, required reviewed exclusion reasons, redacted the exposed machine-local path, and preserved causal hot-reload watcher synchronization.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `Issue branch changes await independent review and PR integration.`
- Integration state: `worktree_only`
- Verification scope: `Bound issue #816 worktree.`
- Integration method used: `PR publication pending`
- Verification performed:
  - `git diff --check`
    `No whitespace errors; no main integration claimed.`
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
  - `bash .csdlc/prepared/issues/512/validate-obs-b-redaction.sh; python3 .csdlc/prepared/issues/816/validate-publication-manifest.py . .csdlc/prepared/issues/816/obs-b-publication-paths.txt; cargo test --locked --manifest-path adl-runtime/Cargo.toml --test config_reload; cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml config_reload --lib; repeated focused cancellation and redaction runs; cargo clippy --locked --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check; cargo fmt --manifest-path adl-runtime/Cargo.toml -- --check; python3 -m py_compile .csdlc/prepared/issues/816/validate-publication-json.py .csdlc/prepared/issues/816/validate-publication-manifest.py; git diff --check`
    `Proves the manifest exactly classifies every tracked or declared #512 artifact, omitted declared artifacts fail, production-derived Runtime JSON and every included publication are scanned, JSON structure is unambiguous, sensitive text and manifest-path leakage are rejected, cancellation assertions follow observed watcher transitions, and both touched Rust manifests are formatting-clean.`
- Results:
  - `Passed: manifest completeness for 29 declared paths, production project_v1 JSON, all 17 included UI/evidence publications, 1 clean and 11 negative redaction fixtures, all 8 config-reload integration tests, 2 kernel unit tests, all-target kernel clippy, both manifest-qualified cargo fmt checks, and repeated causal proofs.`

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
      - "production Runtime projection, exact 29-path declared manifest denominator with 17 included publications, 11 negative fixture groups, and repeated causal cancellation runs"
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
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `25 repetitions each for revert and unreadable-file cancellation after explicit pending-state synchronization.`
- Fixtures or scripts used: `.csdlc/prepared/issues/816/obs-b-publication-paths.txt, validate-publication-manifest.py, validate-publication-json.py, fixtures/obs-b-publication-clean.json, production distributed projection test fixture, generated manifest-path leak fixture, and omitted-declared-artifact fixture`
- Replay verification (same inputs -> same artifacts/order): `The validator regenerates Runtime JSON through project_v1 on every run and rejects duplicate-key decoy-plus-leak input.`
- Ordering guarantees (sorting / tie-break rules used): `Runtime bytes exist before structural scanning; cancellation tests observe pending state before the cancelling action and cancellation state before asserting generation zero.`
- Artifact stability notes: `The generated Runtime projection uses deterministic production JCS serialization; the manifest must exactly cover the declared #512 denominator, paths remain repository-relative and unique, and exclusions require reviewed non-publication reasons.`

## Security / Privacy Checks
- Secret leakage scan performed: `Yes; validator reports only path and category on failure, never matching content.`
- Prompt / tool argument redaction verified: `Every manifest JSON is parsed with duplicate-key rejection and exact sensitive-field values; non-JSON publications reject raw sensitive assignments; manifest-path leakage is a required negative.`
- Absolute path leakage check: `Manifest rejects absolute and parent-traversal paths; included publication bytes are scanned for machine-local paths; every tracked #512 evidence path and SRP/SOR declaration must be included or explicitly classified as a reviewed non-publication exclusion.`
- Sandbox / policy invariants preserved: `All tracked work is in the bound FastWork worktree.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable`
- Run artifact root: `.csdlc/evidence/816`
- Replay command used for verification: `bash .csdlc/prepared/issues/512/validate-obs-b-redaction.sh && cargo test --locked --manifest-path adl-runtime/Cargo.toml --test config_reload`
- Replay result: `pass`

## Artifact Verification
- Primary proof surface: `.csdlc/prepared/issues/512/validate-obs-b-redaction.sh, adl-runtime/tests/distributed_projection.rs, and adl-runtime/tests/config_reload.rs`
- Required artifacts present: `true`
- Artifact schema/version checks: `Validator emits one machine-readable JSON summary on success; manifest-integrity validation reports its exact declared denominator.`
- Hash/byte-stability checks: `not_applicable: no immutable publication hash contract added`
- Missing/optional artifacts and rationale: `No live cloud or browser execution is required for these validation-integrity defects.`

## Decisions / Deviations
- `Used a read-only watcher status channel as the smallest deterministic testability seam.`
- `Kept the OBS-B validator in Bash and made its denominator explicit instead of adding another language runtime.`

## Follow-ups / Deferred work
- `Obtain independent exact-head review and fix every finding.`
- `Publish with Closes #816 and shepherd required CI.`
