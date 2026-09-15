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
Branch: not bound yet; proposed codex/887-v0922-codefriend-local-fitness
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:05:16.416878+00:00

Execution:
- Actor: `Worker #10`
- Model: `none; no model invocation`
- Provider: `none; deterministic local execution`
- Start Time: `2026-09-15`
- End Time: `not_complete; publication and integration pending`

## Summary

Local proof passed: 8 fitness tests plus 11 shared evidence tests under llvm-cov; 3 installed pass/fail/error scenarios with repeat, readback, tamper, deletion and source immutability checks. Strict Clippy passed. Fitness library line coverage 216/222 (97.30%); CLI 84/92 (91.30%). Exact-head review, publication and hosted CI pending.

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
- Completion state: `local_proof_complete_review_pending`
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
- Additional proof artifacts: `docs/codefriend/LOCAL_FITNESS_INSTALLED_PROOF.json; docs/codefriend/LOCAL_FITNESS_PROOF_INVENTORY.json; .csdlc/evidence/887/`

## Actions taken
- `Implemented policy validation, bounded literal use evaluation and shared ReviewRecord findings.`
- `Implemented fitness run/read with exit codes 0/1/2, JSON stdout, redacted events, artifact guards and live readback.`
- `Executed installed pass/fail/error scenarios and focused tests; independent interim review findings resolved.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; root main remains inspection-only`
- Worktree-only paths remaining: `All #887 implementation, cards and proof remain in its bound worktree; no PR yet.`
- Integration state: `local_execution_not_published`
- Verification scope: `Bounded local fitness predicates and installed macOS consumer; hosted CI and Linux proof pending.`
- Integration method used: `none; not published or merged`
- Verification performed:
  - `pending publication and hosted CI`
    `Local installed consumer proof only; no merge evidence.`
- Result: `not_integrated`

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
  - `passed_local; hosted_ci_pending`

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
    status: present_local_proof_pending_final_review
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
- Required artifacts present: `Source, fixtures, tests, docs, installed proof and coverage summary present; exact-head review and remote receipts pending.`
- Artifact schema/version checks: `Policy/report deny unknown fields; live readback recomputes report and rejects tampering.`
- Hash/byte-stability checks: `Installed proof repeats compare complete report objects; policy digest and evidence/run identity retained.`
- Missing/optional artifacts and rationale: `No provider, external qualification or CI integration artifacts claimed. CI integration belongs to #888.`

## Decisions / Deviations
- `Use admitted CF-EVIDENCE directly, without adding an undeclared architecture dependency.`
- `Raw identifier review finding fixed with IdentExt::unraw and three cases. Subprocess fixtures isolated to avoid inherited lock contention. Oversized acquisition rejects before admission, so partial proof uses a scoped missing file.`

## Follow-ups / Deferred work
- `Finish expanded local coverage and committed exact-head review, then native publication.`
- `Resolve required CI, obtain explicit merge authorization, then native finish and clean.`
