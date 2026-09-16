# v0922-test-planner

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

Task ID: issue-0894
Run ID: issue-0894
Version: 0.92.2
Title: [v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings
Branch: codex/894-v0922-test-planner
Card Status: ready
Status: implemented_review_passed_pr_open_ci_refresh_pending
Generated: 2026-09-12T00:10:09.925246+00:00

Execution:
- Actor: `codex:/root/execute_894_to_pr under Sprint 4 #930 delegated execution`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `2026-09-16T00:00:00-07:00`
- End Time: `2026-09-16T00:00:00-07:00`

## Summary

Implemented the bounded test-plan generator, resolved all product findings, reconciled current origin/main, preserved both #894 test-plan and #895 publication CLI routing, and obtained a distinct fresh exact-head PASS. The open PR requires this truthful metadata commit, final exact-head metadata review, push, and renewed standard CI before merge.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `not_run; implementation has not started`

## Issue Metrics Truth
- Expected runtime class: `bounded_local`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `No operator issue token budget assigned; VPP estimates are planning only`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implemented_review_passed_pr_open_ci_refresh_pending`
- Issue goal ref: `Sprint 4 #930 active goal covers #894 execution; this delegated lane advances #894 without replacing the root sprint goal.`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/894/goal-metrics.json (planned, absent until execution)`
- Validation planning prompt: `.csdlc/issues/894/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No measured execution/estimate pair exists`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/894/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/actions/test_plan.rs; adl/src/codefriend/actions/mod.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_testplan.rs`
- Additional proof artifacts: `cargo output from focused codefriend_testplan and codefriend_remediate commands in this session`

## Actions taken
- `Added CodeFriend test-plan action module with typed schema, digest binding, create-only output, traceability, and validation.`
- `Registered `adl codefriend plan tests` and read subcommand.`
- `Replaced manual punctuation char-comparison closure in test-plan path token normalization with an array pattern accepted by clippy.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; implementation is in bound FastWork worktree only`
- Worktree-only paths remaining: `#894 lifecycle records plus adl/src/codefriend/actions/test_plan.rs, action/CLI registration changes, and adl/tests/codefriend_testplan.rs remain on the bound branch pending review/publication.`
- Integration state: `pr_open_post_main_merge_review_passed_ci_refresh_pending`
- Verification scope: `bound issue worktree`
- Integration method used: `Current origin/main e24e0438e40d1f716bd0653ea369d06450827230 is ancestral. After #893 merged, four unmerged #893 corrective commits and lifecycle residue formerly present through the development stack were removed by forward scope-cleanup commit 4e3ff76d5086c7133f442f060f30253712134c63.`
- Verification performed:
  - `Review exact revision 45f25ee536a227550ecff9c4909d739123888879 against origin/main after focused codefriend_testplan and codefriend_ux proof.`
    `Fresh reviewer verified the test-plan generator, path preservation, complete bundle admission, create-only and tamper defenses, and simultaneous #895 publication CLI routing with no actionable findings.`
- Result: `Exact diff against current origin/main contains only #894 lifecycle, test-plan generator, CLI registration, and focused test paths; local proof passed and review/publication remain pending.`

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
  - `CARGO_TARGET_DIR=/Users/daniel/git/agent-design-language/.git/csdlc-v3/local/build-cache/issue-894-postmerge cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan; CARGO_TARGET_DIR=/Users/daniel/git/agent-design-language/.git/csdlc-v3/local/build-cache/issue-894-postmerge cargo test --manifest-path adl/Cargo.toml --test codefriend_ux; cargo fmt --manifest-path adl/Cargo.toml -- --check; git diff --check origin/main...HEAD`
    `Eight nonzero #894 test-plan tests and seven merged #895 publication integration tests passed after resolving the current-main CLI conflict. Formatting and diff hygiene passed. No optional, paid, provider, cloud, or broad coverage lane was run locally.`
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
      - "Post-current-main focused product and adjacent CLI integration proof: 15 tests passed"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: true_bounded_validation_cache_path_only
  artifacts:
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: issue #894 owns codefriend.test_plan.v1 product artifact schema
```

## Determinism Evidence
- Determinism tests executed: `test_plan_maps_every_traceable_finding_to_executable_case; test_plan_omits_findings_without_repository_path; test_plan_preserves_dot_directories_and_root_files; test_plan_consumes_tracked_predecessor_synthesis_with_concrete_mapping; test_plan_reader_rejects_placeholder_untraceable_or_non_test_cases; installed_cli_generates_and_reads_test_plan_without_source_mutation; generator_rejects_standalone_or_tampered_synthesis_bundle; reader_rejects_plan_manifest_synthesis_and_partition_tampering`
- Fixtures or scripts used: `adl/tests/codefriend_testplan.rs synthetic ReviewSynthesis fixtures plus tracked `.csdlc/evidence/892/predecessor-openai-r5-synthesis/synthesis.json`; installed `adl` test binary; no provider calls`
- Replay verification (same inputs -> same artifacts/order): `passed_for_retained_predecessor_and_negative_fixtures`
- Ordering guarantees (sorting / tie-break rules used): `deterministic sorted test case ids; omitted findings tracked explicitly`
- Artifact stability notes: `not_run; implementation has not started`

## Security / Privacy Checks
- Secret leakage scan performed: `not_applicable; no credentials or provider calls used`
- Prompt / tool argument redaction verified: `not_applicable; no provider prompt or credential args used`
- Absolute path leakage check: `SOR command fields intentionally record the host-local Git-common CARGO_TARGET_DIR used to avoid FastWork disk exhaustion during validation. Generated test-plan artifacts remain repository-relative and no provider/secret path is emitted.`
- Sandbox / policy invariants preserved: `true`

## Replay Artifacts
- Trace bundle path(s): `No separate trace bundle is required for this deterministic local consumer; retained predecessor input is .csdlc/evidence/892/predecessor-openai-r5-synthesis/synthesis.json and focused generated artifacts remain test-local.`
- Run artifact root: `.csdlc/evidence/894 (planned; focused proof output retained in session transcript)`
- Replay command used for verification: `CARGO_TARGET_DIR=/Users/daniel/git/agent-design-language/.git/csdlc-v3/local/build-cache/issue-894-target cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan`
- Replay result: `codefriend_testplan 8 passed; codefriend_ux 7 passed; fresh exact-head review PASS`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_testplan.rs plus installed CLI invocation under Cargo test and tracked #892 predecessor synthesis fixture`
- Required artifacts present: `true`
- Artifact schema/version checks: `Eight focused tests validate the complete source synthesis bundle, generated plan bundle, exact digests/counts/references, canonical plan recomputation, one-to-one selected-finding partition, installed CLI generation/readback, and tamper rejection.`
- Hash/byte-stability checks: `synthesis_digest and test_plan_digest are produced through existing CodeFriend hash helper`
- Missing/optional artifacts and rationale: `Execution artifacts are absent because preparation is not delivery`

## Decisions / Deviations
- `The former local #893 development stack is no longer publication authority. Current origin/main is ancestral and unmerged #893 corrective scope was removed forward-only before final #894 proof.`
- `The proof uses the retained accepted #892 synthesis plus deterministic local fixtures. It does not claim a new external-provider run, autonomous test implementation, source mutation, issue creation, publication, or merge.`

## Follow-ups / Deferred work
- `Commit the truthful SRP/SOR metadata, obtain a final fresh exact-head metadata review, and push the current reviewed product lineage.`
- `Update PR #1026 through native C-SDLC with Closes #894, observe renewed required standard CI, and merge only when exact green and clean.`
