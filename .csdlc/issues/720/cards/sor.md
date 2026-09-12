# 720-observatory-live

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

Task ID: issue-0720
Run ID: issue-0720
Version: v0.92.2
Title: [v0.92.2][Observatory] Remove retained-mode demo hazards
Branch: codex/720-observatory-live
Card Status: ready
Status: IMPLEMENTED
Generated: 2026-09-12T00:17:18.048072+00:00

Execution:
- Actor: `Planning #7 / sprint8_720`
- Model: `inherited Codex model`
- Provider: `OpenAI`
- Start Time: `2026-09-12`
- End Time: `in_progress`

## Summary

Removed retained telemetry startup/fallback/routes and static polling; locally validated and independently reviewed. Published as PR #939. Current integration and CI status must be read from that PR.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `No lane change; UI contract only.`

## Issue Metrics Truth
- Expected runtime class: `unknown`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `unknown`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `Local implementation complete; publication and CI truth in PR #939`
- Issue goal ref: `#720 execution goal created before implementation`
- Sprint goal ref: `#934 Sprint 8`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/720/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `unknown`
- Variance category: `unknown`
- Variance note: `No collected comparable totals yet; not claiming zero variance.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/720/cards/sor.md`
- Tracked implementation artifacts: `demos/html-observatory/app.js, index.html, README.md, tests/; adl/tools/test_html_observatory.sh`
- Additional proof artifacts: `.csdlc/evidence/720/VALIDATION.md`

## Actions taken
- `Native edit normalized preparation placeholders, umbrella reference and issue-local plan; native validate/doctor passed.`
- `Removed retained-mode telemetry routes and startup loading while preserving immutable historical files.`
- `Executed 24 node tests, Runtime contract shell, integrated HTTPS/WSS proof and local Chrome transition regression.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none before merge`
- Worktree-only paths remaining: `all issue changes on codex/720-observatory-live`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree`
- Integration method used: `planned reviewed PR; no merge authorization`
- Verification performed:
  - `git status --short; git diff --check`
    `Checked only issue-owned source/cards staged and no whitespace errors.`
- Result: `branch work only; not merged`

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
  - `node --test demos/html-observatory/tests/*.test.mjs; bash adl/tools/test_html_observatory.sh; bash adl/tools/test_v0917_html_observatory_integrated_proof.sh; node demos/html-observatory/tests/live_only.browser.mjs`
    `Proves empty live shell, absent retained routes, Runtime protocol contracts, HTTPS/WSS behavior, actual Chrome startup and disconnect/navigation behavior with intercepted deterministic requests.`
- Results:
  - `Local pass. Hosted CI tracked on exact PR #939 head, not asserted by this local record.`

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
      - "24 node tests; contract shell; integrated HTTPS/WSS shell; Chrome live-only transitions"
  determinism:
    status: passed
    replay_verified: false
    ordering_guarantees_verified: false
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: present
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: false
```

## Determinism Evidence
- Determinism tests executed: `Chrome regression uses fully intercepted deterministic API responses; node regression uses static inputs.`
- Fixtures or scripts used: `demos/html-observatory/tests/live_only.browser.mjs; live_only.test.mjs`
- Replay verification (same inputs -> same artifacts/order): `Not a runtime replay change; no replay claim.`
- Ordering guarantees (sorting / tie-break rules used): `Existing request-generation guard preserved across asynchronous failures.`
- Artifact stability notes: `Historical artifact bytes restored after integrated validator regenerated its local TLS proof.`

## Security / Privacy Checks
- Secret leakage scan performed: `Reviewed staged diff; fixtures contain only synthetic identities and no credentials.`
- Prompt / tool argument redaction verified: `No provider or cloud execution; no sensitive payloads introduced.`
- Absolute path leakage check: `Product source and evidence use relative paths; native binding records necessarily retain host path.`
- Sandbox / policy invariants preserved: `Only bound worktree edits; Chrome proof permission granted; no cloud access.`

## Replay Artifacts
- Trace bundle path(s): `unknown`
- Run artifact root: `.adl/runs/720`
- Replay command used for verification: `unknown`
- Replay result: `unknown`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/720/VALIDATION.md`
- Required artifacts present: `yes`
- Artifact schema/version checks: `Native six-card validation passes; no product schema changes.`
- Hash/byte-stability checks: `git diff shows no historical artifact changes.`
- Missing/optional artifacts and rationale: `No cloud deployment proof because #910 owns deployment after accepted #720 output.`

## Decisions / Deviations
- `Removed boot packet seeding as well as mode controls: otherwise startup still presents historical telemetry.`
- `Current baseline already lacked setRuntimeTestStatus and first orphan; remaining two orphan status constants removed.`

## Follow-ups / Deferred work
- `PR #939 required checks and merge remain governed by live GitHub state; no merge performed by this task.`
- `#910 deployment is separate and requires precise approval.`
