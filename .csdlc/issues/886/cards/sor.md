# v0922-architecture-drift

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

Task ID: issue-0886
Run ID: issue-0886
Version: 0.92.2
Title: [v0.92.2][CF-COG-DRIFT] Report architecture drift between compatible revisions
Branch: codex/886-v0922-architecture-drift
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:04:57.571871+00:00

Execution:
- Actor: `Worker10 /root/review_987_conflicts`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `2026-09-15; exact start time not separately measured`
- End Time: `ongoing_review_publication_ci_pending`

## Summary

Implemented architecture drift from validated graph pairs, original persistent CF-MEMORY comparison gate and shared structural Finding identities. 45 focused tests and11 installed scenarios passed; strict Clippy, fmt and diff checks passed. Independent review and hosted CI remain pending.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `No lane change; runtime local deterministic proof`

## Issue Metrics Truth
- Expected runtime class: `bounded local CPU/filesystem; no provider/network/source execution`
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
- Completion state: `implemented_local_proof_complete_review_pending`
- Issue goal ref: `Active whole Sprint929 goal includes886; parent assigned bounded implementation`
- Sprint goal ref: `v0.92.2 execution Sprint 3; umbrella management owned by #926`
- Goal metrics rollup ref: `Whole Sprint929 goal accounting; no separate child token claim`
- Validation planning prompt: `.csdlc/issues/886/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/886/cards/sor.md`
- Tracked implementation artifacts: `drift.rs; codefriend_structure_cmd.rs; module/help registration; codefriend_cf_cog_drift.rs; codefriend_drift_installed_proof.py; ARCHITECTURE_DRIFT.md; fixtures/drift/PVF.json`
- Additional proof artifacts: `Local logs, installed binary/provenance and11-scenario fixture packet retained separately`

## Actions taken
- `Implemented original graph compatibility gate and stable structural shared Finding comparison`
- `Proved added/removed references, boundary crossings, relocation, partial/narrower/policy NotComparable, tampering, expiry and baseline deletion`
- `Prepared exact-head independent review; no findings waived or review pass claimed`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `None; root main inspection-only`
- Worktree-only paths remaining: `All issue886 source/tests/docs/proof/cards; no PR yet`
- Integration state: `worktree_only`
- Verification scope: `Structural drift with real shared memory storage and graph producer`
- Integration method used: `not_published_or_merged`
- Verification performed:
  - `Pending native publication and hosted CI`
    `No remote or terminal claim`
- Result: `not_published_or_merged`

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
  - `cargo test --offline --locked --manifest-path adl/Cargo.toml --test codefriend_cf_cog_drift; producer structure/memory/evidence regressions; installed codefriend_drift_installed_proof.py; strict CLI Clippy; fmt; diff check`
    `45 focused tests (drift9,structure17,memory8,evidence11) and installed11 drift scenarios passed.`
- Results:
  - `passed_local_review_ci_pending`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed_local_review_ci_pending
    checks_run:
      - "45 focused tests and11 installed scenarios passed"
  determinism:
    status: passed_scoped_fixture_repeat
    replay_verified: yes_same_graph_pair
    ordering_guarantees_verified: yes_deterministic_repeat
  security_privacy:
    status: bounded_admission_and_output_guards_passed
    secrets_leakage_detected: No general secret-scanner claim; only admitted content retained
    prompt_or_tool_arg_leakage_detected: No provider prompts; fixture path output guards passed
    absolute_path_leakage_detected: none_in_installed_fixture_output
  artifacts:
    status: local_proof_present
    required_artifacts_present: local_execution_yes_review_pending
    schema_changes:
      present: codefriend.drift.v1 artifact; unchanged shared Finding/BaselineRef/Comparison schemas
      approved: awaiting_independent_review
```

## Determinism Evidence
- Determinism tests executed: `Repeat/relocation/multiple-occurrence tests and six installed fixture variants`
- Fixtures or scripts used: `codefriend_cf_cog_drift; codefriend_drift_installed_proof.py; inert distinct-revision Git fixtures`
- Replay verification (same inputs -> same artifacts/order): `Installed driver compares complete repeated JSON and readback summary`
- Ordering guarantees (sorting / tie-break rules used): `BTreeMap/BTreeSet structural facts and evidence; shared comparison orders stable IDs`
- Artifact stability notes: `Same inputs exact JSON; line relocation preserves identities while evidence changes`

## Security / Privacy Checks
- Secret leakage scan performed: `No separate scanner; admitted evidence and bounded output review only`
- Prompt / tool argument redaction verified: `No provider prompts; installed output fixture-path guard passed`
- Absolute path leakage check: `Installed runner asserts fixture absolute root absent from stdout/stderr`
- Sandbox / policy invariants preserved: `Inert fixture repos unchanged; no source scripts/builds/provider execution`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/886/LOCAL_PROOF.json and local execution logs`
- Run artifact root: `.csdlc/evidence/886`
- Replay command used for verification: `python3 adl/tools/codefriend_drift_installed_proof.py --binary <installed-adl> --output-root <new-root>`
- Replay result: `passed_same_input_fixture_repeat`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/886/LOCAL_PROOF.json`
- Required artifacts present: `Source/tests/docs/PVF/proof present; independent review pending`
- Artifact schema/version checks: `Strict graph/shared record and artifact validation; entire drift recomputation; malformed/tampered graph/artifact negative tests`
- Hash/byte-stability checks: `Source and installed binary SHA256 retained; repeated complete drift artifacts equal`
- Missing/optional artifacts and rationale: `Local execution proof present. Independent review and CI/terminal receipts are pending later lifecycle stages. No provider trace applicable.`

## Decisions / Deviations
- `Merged882 and885 accepted before native886 binding; current base7e586a34 includes both producers.`
- `Preserve original graph comparison; derived shared structural facts add drift lane without changing inherited guards. Policy changes remain NotComparable; boundary-crossing reference changes under identical policy are explainable edge deltas.`

## Follow-ups / Deferred work
- `Independent full work-product review; repair findings before native publication; CI and merge authorization remain required`
- `Parent owns sprint integration, native terminal reconciliation and cleanup after authorized merge`
