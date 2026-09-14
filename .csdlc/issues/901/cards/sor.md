# v0922-provider-recovery-qualification

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

Task ID: issue-0901
Run ID: issue-0901
Version: 0.92.2
Title: [v0.92.2][QUAL-PROVIDER] Execute real provider failure and recovery qualification
Branch: codex/901-v0922-provider-recovery-qualification
Card Status: draft
Status: EXECUTED
Generated: 2026-09-14T18:06:04.530196+00:00

Execution:
- Actor: `Worker #9 in active Sprint #931 / child #901 goal`
- Model: `gemma:2b artifact sha256:c1864a5eb19305c40519da12cc543519e48a0697ecd30e15d5ac228644957d12`
- Provider: `task-owned local llama-server through production OpenAI-compatible adl-provider-adapter route`
- Start Time: `2026-09-13T01:12:05.040590+00:00`
- End Time: `2026-09-13T01:33:04.698149+00:00`

## Summary

The standalone provider packet is preserved and the Runtime-lifecycle P2 is remediated with a registered #855 Runtime proof. A second review P2 is remediated by reopening and hash-binding six retained execution artifacts. Exact-head independent review passed; refreshed CI, merge and closeout remain pending.

## PVF Lane Truth
- Initial PVF lane: `provider`
- Planned PVF lane: `provider`
- Final PVF lane: `provider`
- Lane change reason: `planned provider lane executed without lane change`

## Issue Metrics Truth
- Expected runtime class: `task-owned local provider integration`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `34`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `No operator issue token budget assigned; VPP estimates are planning only`
- Goal metrics data source: `live-run-09 standalone production-adapter packet and runtime-live-12 registered Runtime packet`
- Goal metrics source ref: `unknown`
- Data-source confidence: `high for recorded local run`
- Estimate error percent: `unknown`
- Completion state: `review_passed_ci_pending`
- Issue goal ref: `thread-goal 01a0924d-fbc0-7d21-b1ec-965c8a9562a4; active Sprint #931 objective explicitly includes child #901 qualification`
- Sprint goal ref: `Sprint 5 umbrella #931; child #901 provider failure and recovery qualification`
- Goal metrics rollup ref: `.csdlc/evidence/901/goal-metrics.json not collected; active Sprint #931 goal service owns session accounting`
- Validation planning prompt: `.csdlc/issues/901/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/901/cards/sor.md`
- Tracked implementation artifacts: `adl/tools/issue855_provider_lifecycle.py; adl/tools/run_issue901_provider_recovery_qualification.py; adl/tools/run_issue901_runtime_recovery_qualification.py; adl/tools/test_run_issue901_provider_recovery_qualification.py; adl/tools/test_run_issue901_runtime_recovery_qualification.py; docs/milestones/v0.92.2/evidence/qual-provider-901/README.md; qualification-report.json; runtime-qualification-report.json`
- Additional proof artifacts: `.csdlc/evidence/901/live-run-09 and .csdlc/evidence/901/runtime-live-12 (private retained runs); runtime-live-12 portable validation binds the Runtime install receipt, CSM configuration status, raw Runtime observations, checkpoint, proxy records, and Guardian log; earlier failed attempts remain private and excluded from acceptance`

## Actions taken
- `Built and invoked the exact production adl-provider-adapter binary from source c6651bb59fc117f10abc9e613d0042f567759ee8`
- `Executed distinct task-owned provider loss, 150 ms timeout, adapter interruption, and fresh-PID recovery scenarios`
- `Validated both raw artifact packets and portable receipts, including six registered Runtime artifact bindings; passed 25 focused deterministic Python tests, strict X.509 verification, py_compile, and diff hygiene; independent exact-head review passed at 2930f13da6a48331438bf81f046a9e37a925e973.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; issue branch only`
- Worktree-only paths remaining: `.csdlc/evidence/901 private raw and failed diagnostic runs`
- Integration state: `draft_pr_present_remediation_unpublished`
- Verification scope: `local source, live task-owned provider execution, registered Runtime lifecycle, raw-artifact-bound portable receipts and deterministic validators passed; exact-head review passed and CI pending`
- Integration method used: `existing draft PR #974; remediation commits not yet pushed`
- Verification performed:
  - `Native publish with authenticated PR readback and refreshed hosted CI remain pending for the remediation head`
    `Draft PR #974 exists at the earlier reviewed head; remediation commits and refreshed hosted CI are not yet published`
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
  - `25 focused Python tests; standalone portable validate-report; registered Runtime portable validate-report with six retained artifact bindings; strict X.509 chain verification; py_compile; git diff --check`
    `Proves actual provider loss and successful fresh-provider recovery through registered Runtime, plus distinct client interruption; preserves separate adapter timeout proof without overstating Runtime timeout configuration`
- Results:
  - `Standalone four-scenario provider-adapter receipt remains valid. Registered Runtime runtime-live-12 passed loss, recovery and client interruption in one unchanged Runtime and validates against six retained raw artifacts. 25 deterministic validator tests passed. Independent review at 2930f13da6a48331438bf81f046a9e37a925e973 found no actionable findings; CI remains pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: local_complete_review_passed_ci_pending
    checks_run:
      - "passed: raw and portable validator reports contain no errors"
  determinism:
    status: 21/21 negative evidence-integrity cases and 4/4 positive validation cases passed
    replay_verified: passed
    ordering_guarantees_verified: passed
  security_privacy:
    status: passed_local
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed_local
    required_artifacts_present: yes for local remediation and independent review; refreshed CI pending
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `25/25 focused Python tests passed: 21 negative evidence-integrity cases and four positive validation cases`
- Fixtures or scripts used: `run_issue901_provider_recovery_qualification.py; run_issue901_runtime_recovery_qualification.py; both focused unittest modules; issue855_provider_lifecycle.py`
- Replay verification (same inputs -> same artifacts/order): `portable validate-report passed`
- Ordering guarantees (sorting / tie-break rules used): `loss, timeout, interruption, and recovery serialized; recovery used a new provider PID; four request body digests are unique`
- Artifact stability notes: `The standalone and Runtime portable reports bind their private execution artifacts by byte size and SHA-256. The Runtime validator reopens six retained files and rejects a coherent report when those artifacts are absent. Runtime executable source is cb292d7fbfb55b987a05ce1c0c3bd88954c23ebb; independent review passed at 2930f13da6a48331438bf81f046a9e37a925e973.`

## Security / Privacy Checks
- Secret leakage scan performed: `performed; no credential values or prompt/output text in portable receipt`
- Prompt / tool argument redaction verified: `portable receipt omits prompt and generated output; production adapter logs contain redacted event fields`
- Absolute path leakage check: `passed for portable receipt; no /Users, /Volumes, or /private paths`
- Sandbox / policy invariants preserved: `only task-owned process groups and dynamic loopback ports used; #851 and shared Ollama service untouched`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/901/live-run-09 and .csdlc/evidence/901/runtime-live-12 (private); docs/milestones/v0.92.2/evidence/qual-provider-901/qualification-report.json and runtime-qualification-report.json (portable)`
- Run artifact root: `.csdlc/evidence/901/live-run-09 and .csdlc/evidence/901/runtime-live-12 (private)`
- Replay command used for verification: `documented runner invocation with exact adapter/provider/model inputs; output directory must be new`
- Replay result: `live-run-09 passed 4/4 standalone scenarios; runtime-live-12 passed 3/3 registered Runtime scenarios`

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.92.2/evidence/qual-provider-901/qualification-report.json; runtime-qualification-report.json`
- Required artifacts present: `local execution and review artifacts present; refreshed CI, merge and terminal receipts pending`
- Artifact schema/version checks: `portable report validate-report passed with four scenarios`
- Hash/byte-stability checks: `adapter, provider binary, model file, public report, and four request-body digests recorded`
- Missing/optional artifacts and rationale: `Draft PR #974 exists. Refreshed-head hosted CI, merge, and terminal receipts are absent because the remediation commits have not yet been published; independent review evidence is retained privately.`

## Decisions / Deviations
- `#855 is satisfied: PR #964 merged and issue #855 closed, delivering the registered-provider lifecycle. #852 is also accepted through merged PR #963 for any WSS failure-event evidence consumed. #851 remains dirty and unmerged in its separate registered worktree; its bytes are preserved and its paths are excluded.`
- `Use only task-owned local subprocesses and new #901 paths; preserve #851 and do not mutate shared provider services`

## Follow-ups / Deferred work
- `Publish the native draft PR with Closes #901 and verify exact base, head and linkage.`
- `Observe required CI and resolve any current-head findings before requesting merge.`
