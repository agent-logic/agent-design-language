# v0922-codefriend-local-fitness

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

Task ID: issue-0887
Run ID: issue-0887
Version: 0.92.2
Title: [v0.92.2][CF-GOV] Execute local architecture fitness functions
Branch: codex/887-v0922-codefriend-local-fitness
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:05:16.416878+00:00

Execution:
- Actor: `Worker #10`
- Model: `none; no model invocation`
- Provider: `none; deterministic local execution`
- Start Time: `2026-09-15`
- End Time: `in_progress; published draft awaiting CI and authorized integration`

## Summary

Integrated main 7e586a34d6a6ee7e2103ca8d004bc0848a839e31 after PR #987 merged; preserved memory, fitness and architecture commands. 44 focused tests, strict Clippy and 20 installed scenarios passed. Independent conflict resolution review passed; final exact-head review and hosted CI follow. PR #989 remains open and nondraft.

## PVF Lane Truth
- Initial PVF lane: `owner_binary`
- Planned PVF lane: `owner_binary`
- Final PVF lane: `owner_binary`
- Lane change reason: `No lane change; runtime-classified semantic and installed proof supports owner-binary qualification.`

## Issue Metrics Truth
- Expected runtime class: `bounded local CPU/filesystem/Rust`
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
- Completion state: `base_integrated_local_proof_passed_review_and_ci_pending`
- Issue goal ref: `Active whole Sprint3#929 goal; #887 owns local machine-checkable fitness execution and installed proof.`
- Sprint goal ref: `Sprint #929, all eight children #882 through #889; shared whole-sprint goal remains active.`
- Goal metrics rollup ref: `.csdlc/evidence/887/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/887/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/887/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/governance; adl/src/cli/codefriend_fitness_cmd.rs; shared CLI/module registration; Cargo manifests; tests, fixtures, installed runner and docs.`
- Additional proof artifacts: `.csdlc/evidence/887/MAIN_987_INTEGRATION_PROOF.json; main-987-* raw proof retained locally.`

## Actions taken
- `Implemented policy validation, bounded literal use evaluation and shared ReviewRecord findings.`
- `Implemented fitness run/read with exit codes 0/1/2, JSON stdout, redacted events, artifact guards and live readback.`
- `Executed installed pass/fail/error scenarios and focused tests; independent interim review findings resolved.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; root main remains inspection-only`
- Worktree-only paths remaining: `Integrated source and refreshed proof pending push; raw logs, isolated binaries, fixture repositories and receipts remain local evidence.`
- Integration state: `published_base_integration_pending_push`
- Verification scope: `Bounded local fitness predicates and installed macOS consumer; hosted CI and Linux proof pending.`
- Integration method used: `Local merge origin/main in bound issue worktree; no remote PR merge. Existing PR publication and ready state have authenticated native receipts.`
- Verification performed:
  - `Native creation receipt and native publish --observe-github; live PR readback confirmed main base, exact head and draft state.`
    `Publication verified; no merge evidence.`
- Result: `Local merge of origin/main 7e586a34d6a6ee7e2103ca8d004bc0848a839e31 into #887 branch; no remote PR merge.`

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
  - `cargo test --offline --locked --manifest-path adl/Cargo.toml --test codefriend_cf_gov --test codefriend_evidence; cargo llvm-cov with same targets; strict Clippy; installed proof runner`
    `Local accepted predicates and installed consumer proved; hosted integration pending.`
- Results:
  - `44 focused tests (fitness8,memory8,structure17,evidence11), strict Clippy and 20 installed scenarios (fitness3,memory8,structure9) passed. Current-head hosted CI pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed_local
    checks_run:
      - "Installed predicate execution and artifact readback passed."
  determinism:
    status: passed
    replay_verified: passed
    ordering_guarantees_verified: passed
  security_privacy:
    status: bounded_checks_passed
    secrets_leakage_detected: none observed in bounded fixtures
    prompt_or_tool_arg_leakage_detected: none observed in bounded fixtures
    absolute_path_leakage_detected: none observed in fitness stderr
  artifacts:
    status: local_and_publication_proof_retained_ci_pending
    required_artifacts_present: local acceptance artifacts present
    schema_changes:
      present: new explicit fitness policy and report schema v1
      approved: within #887 accepted versioned-policy scope
```

## Determinism Evidence
- Determinism tests executed: `Installed pass/fail/error repeats produced identical reports over each original admission.`
- Fixtures or scripts used: `adl/tests/codefriend_cf_gov.rs; adl/tests/fixtures/codefriend/fitness; adl/tools/codefriend_fitness_installed_proof.py`
- Replay verification (same inputs -> same artifacts/order): `3 installed scenarios repeated and read back successfully.`
- Ordering guarantees (sorting / tie-break rules used): `Rules retain explicit input order; violations and errors are sorted/deduplicated; findings sorted by stable ID.`
- Artifact stability notes: `Readback recomputes complete report from live admission and policy; tampering and deletion denied.`

## Security / Privacy Checks
- Secret leakage scan performed: `No blanket secret guarantee. Shared evidence regressions cover admitted redaction; fitness CLI checks content-free error events.`
- Prompt / tool argument redaction verified: `Fitness failure envelope and stderr omit raw error chains and host paths.`
- Absolute path leakage check: `Installed proof checks host fixture root is absent from stderr; declared policy paths are relative.`
- Sandbox / policy invariants preserved: `No repository code, macro, provider or script execution; only explicit result artifacts and existing store lock.`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/887/`
- Run artifact root: `.csdlc/evidence/887/`
- Replay command used for verification: `python3 adl/tools/codefriend_fitness_installed_proof.py --binary <installed-adl> --fixture-root <new-fixture-root>`
- Replay result: `passed 3 installed scenarios`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/887/LOCAL_PROOF.json; docs/codefriend/LOCAL_FITNESS_INSTALLED_PROOF.json`
- Required artifacts present: `Updated source, combined tests, refreshed installed proofs and interim integration review present; final exact-head review and hosted CI pending.`
- Artifact schema/version checks: `Policy/report deny unknown fields; live readback recomputes report and rejects tampering.`
- Hash/byte-stability checks: `Installed proof repeats compare complete report objects; policy digest and evidence/run identity retained.`
- Missing/optional artifacts and rationale: `Full hosted validation remains required for this implementation PR. Product CI fitness integration belongs to #888.`

## Decisions / Deviations
- `Use admitted CF-EVIDENCE directly, without adding an undeclared architecture dependency.`
- `Raw identifier review finding fixed with IdentExt::unraw and three cases. Subprocess fixtures isolated to avoid inherited lock contention. Oversized acquisition rejects before admission, so partial proof uses a scoped missing file.`

## Follow-ups / Deferred work
- `Obtain final exact-head integration review, native review, push and verify new hosted CI before merge approval.`
- `Resolve required CI, obtain explicit merge authorization, then native finish and clean.`
