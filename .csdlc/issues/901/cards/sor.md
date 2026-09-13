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
Status: implemented
Generated: 2026-09-12T00:15:14.473149+00:00

Execution:
- Actor: `Worker #9 in active Sprint #931 / child #901 goal`
- Model: `gemma:2b artifact sha256:c1864a5eb19305c40519da12cc543519e48a0697ecd30e15d5ac228644957d12`
- Provider: `task-owned local llama-server through production OpenAI-compatible adl-provider-adapter route`
- Start Time: `2026-09-13T01:12:05.040590+00:00`
- End Time: `2026-09-13T01:24:47.636249+00:00`

## Summary

Implemented and executed real task-owned provider loss, timeout, interruption, and healthy recovery through the production adapter. Live-run-08 passed 4/4 at source 159595ac62ec845953ada5704d0abc8c6dc1fd41; independent exact-head review, CI, PR, merge, and closeout remain pending.

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
- Goal metrics data source: `live-run-08 portable receipt and local command results`
- Goal metrics source ref: `unknown`
- Data-source confidence: `high for recorded local run`
- Estimate error percent: `unknown`
- Completion state: `validation_passed_review_pending`
- Issue goal ref: `thread-goal 01a0924d-fbc0-7d21-b1ec-965c8a9562a4; active Sprint #931 objective explicitly includes child #901 qualification`
- Sprint goal ref: `Sprint 5 umbrella #931; child #901 provider failure and recovery qualification`
- Goal metrics rollup ref: `.csdlc/evidence/901/goal-metrics.json (planned, absent until execution)`
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
- Tracked implementation artifacts: `adl/tools/run_issue901_provider_recovery_qualification.py; adl/tools/test_run_issue901_provider_recovery_qualification.py; docs/milestones/v0.92.2/evidence/qual-provider-901/README.md; qualification-report.json`
- Additional proof artifacts: `.csdlc/evidence/901/live-run-08 raw private packet; earlier failed and superseded packets retained privately`

## Actions taken
- `Built and invoked the exact production adl-provider-adapter binary from source 159595ac62ec845953ada5704d0abc8c6dc1fd41`
- `Executed distinct task-owned provider loss, 150 ms timeout, adapter interruption, and fresh-PID recovery scenarios`
- `Validated the raw artifact packet, portable receipt, 15 deterministic evidence-integrity cases, 51 focused Rust tests, formatting and diff hygiene; independent exact-head review remains pending`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; issue branch only`
- Worktree-only paths remaining: `.csdlc/evidence/901 private raw and failed diagnostic runs`
- Integration state: `worktree_only`
- Verification scope: `local source, live provider execution, raw artifact binding, and portable receipt; independent exact-head review and CI pending`
- Integration method used: `pending publication`
- Verification performed:
  - `deferred; PR not published`
    `deferred; PR not published`
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
  - `cargo build; 51-test focused Rust provider-adapter suite; 15-test Python validator suite; live-run-08 4/4; raw and portable validate-report; cargo fmt --check; py_compile; git diff --check; path/redaction scan`
    `Proves actual provider invocation reached the task-owned process before each fault and successful distinct work completed on a fresh provider PID`
- Results:
  - `passed locally; independent exact-head review and CI pending`

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
      - "passed: raw and portable validator reports contain no errors"
  determinism:
    status: 15/15 negative validator cases passed
    replay_verified: passed
    ordering_guarantees_verified: passed
  security_privacy:
    status: passed_local
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed_local
    required_artifacts_present: yes for local proof; review, CI, PR, merge and terminal artifacts pending
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `15/15 evidence-integrity tests passed`
- Fixtures or scripts used: `run_issue901_provider_recovery_qualification.py; test_run_issue901_provider_recovery_qualification.py`
- Replay verification (same inputs -> same artifacts/order): `portable validate-report passed`
- Ordering guarantees (sorting / tie-break rules used): `loss, timeout, interruption, and recovery serialized; recovery used a new provider PID; four request body digests are unique`
- Artifact stability notes: `Portable report binds artifact refs, byte sizes and SHA-256 digests; raw validator re-hashes each live-run-08 artifact.`

## Security / Privacy Checks
- Secret leakage scan performed: `performed; no credential values or prompt/output text in portable receipt`
- Prompt / tool argument redaction verified: `portable receipt omits prompt and generated output; production adapter logs contain redacted event fields`
- Absolute path leakage check: `passed for portable receipt; no /Users, /Volumes, or /private paths`
- Sandbox / policy invariants preserved: `only task-owned process groups and dynamic loopback ports used; #851 and shared Ollama service untouched`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/901/live-run-08 (private); docs/milestones/v0.92.2/evidence/qual-provider-901/qualification-report.json (portable)`
- Run artifact root: `.csdlc/evidence/901/live-run-08 (private)`
- Replay command used for verification: `documented runner invocation with exact adapter/provider/model inputs; output directory must be new`
- Replay result: `live-run-08 passed 4/4`

## Artifact Verification
- Primary proof surface: `docs/milestones/v0.92.2/evidence/qual-provider-901/qualification-report.json`
- Required artifacts present: `yes for local implementation and execution; review/CI/PR artifacts pending`
- Artifact schema/version checks: `portable report validate-report passed with four scenarios`
- Hash/byte-stability checks: `adapter, provider binary, model file, public report, and four request-body digests recorded`
- Missing/optional artifacts and rationale: `Independent review, PR, CI, merge, and terminal receipts are absent because publication has not occurred.`

## Decisions / Deviations
- `#855 is satisfied: PR #964 merged and issue #855 closed, delivering the registered-provider lifecycle. #852 is also accepted through merged PR #963 for any WSS failure-event evidence consumed. #851 remains dirty and unmerged in its separate registered worktree; its bytes are preserved and its paths are excluded.`
- `Use only task-owned local subprocesses and new #901 paths; preserve #851 and do not mutate shared provider services`

## Follow-ups / Deferred work
- `Obtain independent exact-head review and fix actionable findings`
- `Run native review/publish, CI watch, then wait for operator merge authorization`
