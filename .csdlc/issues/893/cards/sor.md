# v0922-remediation-planner

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

Task ID: issue-0893
Run ID: issue-0893
Version: 0.92.2
Title: [v0.92.2][CF-REMEDIATE] Generate a bounded remediation plan from review findings
Branch: codex/893-v0922-remediation-planner
Card Status: ready
Status: implemented_review_passed_pr_open_ci_refresh_pending
Generated: 2026-09-12T00:10:02.687479+00:00

Execution:
- Actor: `unassigned implementation owner`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_collected; work resumed under active Sprint 4 goal`
- End Time: `not_collected; implementation remediation still in progress until fresh review and publication`

## Summary

Implemented and remediated the CodeFriend remediation planner, merged terminal #894, forward-reapplied the three reviewed #893 corrective commits, proved the combined remediation/test-plan/publication lineage, and obtained a fresh exact-head PASS. Corrective PR #1024 remains open pending metadata review, push, renewed CI, and merge.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `No lane change; runtime lane executed for deterministic local planner/reader proof.`

## Issue Metrics Truth
- Expected runtime class: `bounded_local`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `1200`
- Actual validation seconds: `approximately 147 seconds for cold-cache focused remediation test after local target cleanup, plus fast warm-cache fmt/synthesis/regression reruns; exact wall time is not recorded as authoritative metrics`
- Actual PR wait seconds: `not_started`
- Actual CI wait seconds: `not_started`
- Budget source: `No operator issue token budget assigned; VPP estimates are planning only`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implemented_review_passed_pr_open_ci_refresh_pending`
- Issue goal ref: `Sprint 4 #930 active goal covers #893 execution in this session; single goal slot prevented replacing it with a separate child goal.`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/893/goal-metrics.json (planned, absent until execution)`
- Validation planning prompt: `.csdlc/issues/893/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `Per-issue elapsed/token metrics are unavailable from the active sprint goal; validation timing is approximate and not used as precise variance data.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/893/cards/sor.md`
- Tracked implementation artifacts: `adl/src/cli/codefriend_cmd.rs; adl/src/codefriend/mod.rs; adl/src/codefriend/actions/mod.rs; adl/src/codefriend/actions/remediation.rs; adl/tests/codefriend_remediate.rs; .csdlc/issues/893; .csdlc/transactions/completed/893`
- Additional proof artifacts: `Focused local proof output retained in terminal history; no external provider or GitHub mutation proof claimed.`

## Actions taken
- `Made admitted evidence records the sole repository-path authority for remediation actions and removed semantic-anchor path inference.`
- `Reused the ingestion path validator and made missing or unrepresentable evidence-ID path bindings fail closed instead of silently omitting or truncating them.`
- `Added a completed canonical review fixture proving exact preservation of docs/My File.md, alongside public API, plan alias, substitution, tamper, and source-immutability negatives.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `adl/src/cli/codefriend_cmd.rs; adl/src/codefriend/mod.rs; adl/src/codefriend/actions/mod.rs; adl/src/codefriend/actions/remediation.rs; adl/tests/codefriend_remediate.rs`
- Worktree-only paths remaining: `.csdlc/issues/893/ and .csdlc/transactions/completed/893/ are generated lifecycle material in the bound worktree until publication/finish; implementation source changes are tracked candidate paths.`
- Integration state: `current_main_through_894_reconciled_review_passed_ci_refresh_pending`
- Verification scope: `bound_issue_worktree`
- Integration method used: `bounded implementation in registered FastWork issue worktree; not published or merged`
- Verification performed:
  - `Review exact revision a7cddf07dea4f557ed089e0be02adffbe3985717 against origin/main 75f6d600f3c474d2135f7928951a7ff79220187e after focused 24-test proof.`
    `Fresh reviewer verified completed-synthesis admission, exact evidence paths, canonical replanning, create-only output, intact #894/#895 routes, and no #896 absorption with no actionable findings.`
- Result: `Merged current origin/main at 75f6d600f3c474d2135f7928951a7ff79220187e (#894) into the issue branch, detected and repaired the resulting stale #893 source/test regression by forward-reapplying the three reviewed corrective commits, and validated exact combined behavior.`

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
  - `CARGO_TARGET_DIR=/Volumes/FastWork/adl-targets/issue-893 cargo test --manifest-path adl/Cargo.toml --test codefriend_remediate; CARGO_TARGET_DIR=/Volumes/FastWork/adl-targets/issue-893 cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan; CARGO_TARGET_DIR=/Volumes/FastWork/adl-targets/issue-893 cargo test --manifest-path adl/Cargo.toml --test codefriend_ux; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check origin/main...HEAD`
    `Nine remediation tests, eight test-plan tests, and seven publication UX tests passed after final current-main reconciliation and forward repair. Formatting and exact-range diff hygiene passed. No optional, provider, paid, cloud, or broad workspace lane was run locally.`
- Results:
  - `passed`

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
      - "Final current-main focused product and adjacent integration proof: 24 tests passed"
  determinism:
    status: passed_for_completed_synthesis_bundle_and_negative_fixtures
    replay_verified: The planner preserves exact paths from completed admitted evidence, including docs/My File.md, and the installed CLI generates and rereads the retained Vector action only from the manifest-declared artifact while preserving source bundle bytes.
    ordering_guarantees_verified: passed; validator recomputes topological order and rejects tampered non-topological action_order.
  security_privacy:
    status: passed_for_completed_synthesis_bundle_and_local_negative_fixture_scope
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: true_bounded_validation_cache_path_only
  artifacts:
    status: passed_for_completed_synthesis_bundle_scope
    required_artifacts_present: passed_for_completed_synthesis_bundle_scope
    schema_changes:
      present: true; new remediation plan and manifest schema tags are introduced inside CodeFriend action planning output.
      approved: issue-local implementation pending fresh review; not merged or published yet
```

## Determinism Evidence
- Determinism tests executed: `Nine focused tests cover completed-review-derived ordering, exact evidence-ID path resolution, ingestion-valid space paths, dot-directory/root-file paths, structural tamper rejection, real Vector generation/readback, bare synthesis rejection, aliased plan rejection, unrelated action rejection, missing evidence, bundle tamper rejection, source immutability, and fresh-output enforcement.`
- Fixtures or scripts used: `Retained completed Vector CF-SYNTHESIS bundle at .csdlc/evidence/892/predecessor-openai-r5-synthesis plus focused local negative fixtures generated under the Cargo target directory; no network or provider credentials used.`
- Replay verification (same inputs -> same artifacts/order): `passed_for_real_completed_synthesis_bundle_and_negative_fixtures`
- Ordering guarantees (sorting / tie-break rules used): `Generated remediation actions are dependency-ordered with deterministic severity/id tie-breaks; reader validation rejects action_order values that differ from recomputed topological order.`
- Artifact stability notes: `Planner refuses an existing output directory and writes output artifacts with create-new semantics before returning success.`

## Security / Privacy Checks
- Secret leakage scan performed: `not_applicable; no provider credentials or secret-bearing inputs used`
- Prompt / tool argument redaction verified: `not_applicable; no provider prompts or secret tool arguments used`
- Absolute path leakage check: `SOR command fields intentionally record the host-local Git-common `CARGO_TARGET_DIR` used to avoid FastWork disk exhaustion during validation. Generated product artifacts and CodeFriend planner output remain repository-relative and no provider/secret path is emitted.`
- Sandbox / policy invariants preserved: `passed; implementation reads local synthesis JSON and writes only caller-selected fresh output directory artifacts.`

## Replay Artifacts
- Trace bundle path(s): `Generated local test artifacts under adl/target/codefriend-remediate-tests; no durable release bundle created before review/publication.`
- Run artifact root: `test-generated per-run artifacts under adl/target/codefriend-remediate-tests during focused tests; durable SOR proof is this card plus command output retained in terminal history until formal evidence capture.`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_remediate; cargo fmt --manifest-path adl/Cargo.toml --check; cargo clippy --manifest-path adl/Cargo.toml --all-targets --all-features -- -D warnings; git diff --check`
- Replay result: `codefriend_remediate 9 passed; codefriend_testplan 8 passed; codefriend_ux 7 passed; fresh exact-head review PASS`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_remediate.rs and generated per-test artifacts under adl/target/codefriend-remediate-tests`
- Required artifacts present: `passed_for_completed_synthesis_bundle_scope`
- Artifact schema/version checks: `Nine focused tests parse and assert remediation and source manifests, completed review references and counts, exact evidence-bound paths including docs/My File.md, and installed readback; they reject bare synthesis files, aliased plan filenames, missing evidence, unrelated substitution, tampered bundles, invalid shapes, and non-topological order.`
- Hash/byte-stability checks: ``cargo fmt --manifest-path adl/Cargo.toml --check` passed; `git diff --check` passed.`
- Missing/optional artifacts and rationale: `CI, independent exact-head review and publication proof are pending. Provider-generated reviews and GitHub issue creation are sibling/follow-on concerns and not required for this local remediation-plan generator.`

## Decisions / Deviations
- `The CLI surface is `adl codefriend plan remediation` with a `read` subcommand so planning and complete-plan inspection remain distinct and source-mutating repair execution is not implied.`
- `The production proof consumes the retained completed Vector CF-SYNTHESIS bundle and exercises local negative fixtures. It does not rerun external providers, create GitHub issues, or claim publication authority.`

## Follow-ups / Deferred work
- `Commit this truthful SRP/SOR metadata and obtain a final fresh exact-head metadata review.`
- `Push and update PR #1024 through native C-SDLC, observe renewed required standard CI, and merge only when exact green and clean.`
