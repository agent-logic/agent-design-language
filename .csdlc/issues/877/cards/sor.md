# 877-uts-package

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

Task ID: issue-0877
Run ID: issue-0877
Version: v0.92.2
Title: [v0.92.2][PLAT-UTS] Install a versioned UTS package consumed by Runtime tool dispatch
Branch: codex/877-uts-package
Card Status: ready
Status: reviewed; publication pending
Generated: 2026-09-12T00:53:52.824952+00:00

Execution:
- Actor: `Codex execute_877 subagent`
- Model: `Inherited Codex session configuration; no external model invocation in validation`
- Provider: `Inherited Codex session configuration; no external model invocation in validation`
- Start Time: `2026-09-12T00:39:35Z; issue goal creation timestamp`
- End Time: `Not ended; active review/publication work`

## Summary

adl-uts 0.1.0 owns the existing UTS declaration contract. Runtime registry loads through its version-aware validator; ACC remains authority. Actual governed readonly observation and all rejection cases passed; isolated package artifact consumed successfully.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `No lane change; package plus production-consumer behavior`

## Issue Metrics Truth
- Expected runtime class: `focused local package and Runtime integration`
- Estimated elapsed seconds: `Not separately measured; no numerical estimate or total claimed`
- Actual elapsed seconds: `Not separately measured; no numerical estimate or total claimed`
- Actual active work seconds: `Not separately measured; no numerical estimate or total claimed`
- Estimated total tokens: `Not separately measured; no numerical estimate or total claimed`
- Actual total tokens: `Not separately measured; no numerical estimate or total claimed`
- Estimated validation seconds: `Not separately measured; no numerical estimate or total claimed`
- Actual validation seconds: `Not separately measured; no numerical estimate or total claimed`
- Actual PR wait seconds: `No PR or hosted CI yet`
- Actual CI wait seconds: `No PR or hosted CI yet`
- Budget source: `No token budget specified`
- Goal metrics data source: `.csdlc/evidence/877/consumer-parity.json; local .adl/877-proof command logs`
- Goal metrics source ref: `.csdlc/evidence/877/consumer-parity.json; local .adl/877-proof command logs`
- Data-source confidence: `partial: test counts and command outcomes observed; time/token totals not yet finalized`
- Estimate error percent: `Not separately measured; no numerical estimate or total claimed`
- Completion state: `implementation and independent review complete; native publication and hosted CI pending`
- Issue goal ref: `Active issue #877 implementation goal in execute_877 agent; parent Sprint 2/#928 goal remains active`
- Sprint goal ref: `Sprint 2 umbrella #928`
- Goal metrics rollup ref: `Active issue #877 implementation goal in execute_877 agent; parent Sprint 2/#928 goal remains active`
- Validation planning prompt: `.csdlc/issues/877/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `Not applicable: no numeric estimate was declared`
- Variance analysis completed: `Not applicable: no numeric estimate was declared`
- Variance category: `Unestimated; proof counts and actual results retained without fabricated cost numbers`
- Variance note: `Unestimated; proof counts and actual results retained without fabricated cost numbers`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/877/cards/sor.md`
- Tracked implementation artifacts: `adl-uts/; adl/src/uts.rs; adl/src/tool_registry.rs; adl/src/resident_tool_execution.rs; adl/examples/uts_package_runtime.rs; docs/specs/uts/PACKAGE.md`
- Additional proof artifacts: `.csdlc/evidence/877/consumer-parity.json, runtime-dispatch.json, install-proof.json, install-repeat-proof.json, REVIEW.md; docs/validation/issue877_uts_package.json`

## Actions taken
- `Extracted existing types/validators into adl-uts 0.1.0 with bundled assets and compatibility reexport; preserved original 18 unit tests`
- `Integrated package loader into Runtime registry; executed actual governed readonly adapter with supported schemas and nine denial cases; production resident tick passed`
- `Packaged clean Git snapshot, consumed extracted artifact offline, checked schema mirror parity; retained evidence and added required CI package lane`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `None; implementation remains on codex/877-uts-package`
- Worktree-only paths remaining: `All issue implementation paths pending reviewed PR integration`
- Integration state: `worktree_only`
- Verification scope: `Issue-owned package, named Runtime consumers, focused proof and workflow coverage`
- Integration method used: `Not integrated; native publication will follow independent review`
- Verification performed:
  - `git status --short --branch; git log --oneline HEAD..origin/main`
    `Bound issue branch confirmed; latest origin/main documentation integrated without source conflicts`
- Result: `worktree_only`

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
  - `cargo test --locked --manifest-path adl-uts/Cargo.toml; focused ADL unit filters uts/tool_registry/governed_executor/resident_tool_execution; production resident tick test; cargo run --locked --manifest-path adl/Cargo.toml --example uts_package_runtime; package and consumer clippy; verify_install.py; test_ci_runtime_contracts.sh`
    `20 package tests, 70 retained UTS-filtered tests, 10 registry, 25 governed, 8 resident; 1 Runtime tick; 11 dispatch scenarios; isolated artifact consumed; strict clippy and CI contracts pass`
- Results:
  - `passed local focused proof and independent review; hosted CI pending`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed for local proof; no hosted or external claims
    checks_run:
      - "All original 18 contract tests remain in package; original 88 UTS-filtered denominator becomes 70 retained +18 moved"
  determinism:
    status: passed for local proof; no hosted or external claims
    replay_verified: Replay denial exercised: previously handled proposal caused zero adapter calls
    ordering_guarantees_verified: Fixed scenario order; BTreeMap/BTreeSet contract behavior unchanged
  security_privacy:
    status: passed for local proof; no hosted or external claims
    secrets_leakage_detected: No credentials used or retained; proof contains only synthetic aggregate observation and synthetic proposal; retained evidence paths are repository-relative
    prompt_or_tool_arg_leakage_detected: No credentials used or retained; proof contains only synthetic aggregate observation and synthetic proposal; retained evidence paths are repository-relative
    absolute_path_leakage_detected: No credentials used or retained; proof contains only synthetic aggregate observation and synthetic proposal; retained evidence paths are repository-relative
  artifacts:
    status: passed for local proof; no hosted or external claims
    required_artifacts_present: yes: package, migration docs, parity, actual Runtime dispatch and installed artifact evidence
    schema_changes:
      present: Package ownership/loader added; existing wire types/schema bytes retained
      approved: Within #877 explicit package/compatibility scope; independent source review approved a5bd7feabfaf3aa5f1e1f51d68f60b443d5a3c67; final record reconciliation before publication.
```

## Determinism Evidence
- Determinism tests executed: `20 package tests and focused ADL parity tests; 11 deterministic production-dispatch scenarios`
- Fixtures or scripts used: `adl-uts/tools/verify_install.py; uts_package_runtime example; existing UTS conformance and named-consumer tests`
- Replay verification (same inputs -> same artifacts/order): `Replay denial exercised: previously handled proposal caused zero adapter calls`
- Ordering guarantees (sorting / tie-break rules used): `Fixed scenario order; BTreeMap/BTreeSet contract behavior unchanged`
- Artifact stability notes: `Repeated clean installation at a5bd7feabf produced byte-identical reports and identical package SHA256 86a93d153cd0d7d020a71f3ce2f1c334171139011bf5b00b6c748781338306d1. Original earlier artifact proof retained.`

## Security / Privacy Checks
- Secret leakage scan performed: `Reviewed retained proof: synthetic proposal and aggregate-only observation, no credentials/private payloads; portable evidence paths`
- Prompt / tool argument redaction verified: `Reviewed retained proof: synthetic proposal and aggregate-only observation, no credentials/private payloads; portable evidence paths`
- Absolute path leakage check: `Reviewed retained proof: synthetic proposal and aggregate-only observation, no credentials/private payloads; portable evidence paths`
- Sandbox / policy invariants preserved: `No provider/cloud calls or external package publication; real adapter read-only; denials cause zero effects`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/877/runtime-dispatch.json`
- Run artifact root: `.csdlc/evidence/877/`
- Replay command used for verification: `cargo run --locked --manifest-path adl/Cargo.toml --example uts_package_runtime`
- Replay result: `Replay denial exercised: previously handled proposal caused zero adapter calls`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/877/runtime-dispatch.json`
- Required artifacts present: `yes: package, migration docs, parity, actual Runtime dispatch and installed artifact evidence`
- Artifact schema/version checks: `Package version 0.1.0 independent of uts.v1/uts.v1.1; unsupported versions rejected; schema JSON parsed and mirrors compared`
- Hash/byte-stability checks: `Clean source package SHA-256 recorded in install-proof.json; compatibility schema mirrors byte-equal; proof source revision retained`
- Missing/optional artifacts and rationale: `No external registry, provider or cloud evidence: explicitly out of scope`

## Decisions / Deviations
- `Validate declaration before v1 normalization so unsupported schema cannot be silently upgraded`
- `Preserve adl::uts source compatibility and schema mirrors; canonical implementation resides in package`

## Follow-ups / Deferred work
- `Native exact-head review reconciliation, PR publication and required CI`
- `Merge and terminal closeout are not claimed by this active implementation record`
