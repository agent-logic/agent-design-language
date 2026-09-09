# release-truth-refresh

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

Task ID: issue-0817
Run ID: issue-0817
Version: v0.92.1
Title: [v0.92.1][TAIL-06.12][release] Refresh candidate proof and canonical release truth
Branch: codex/817-release-truth-refresh
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-09T22:00:00Z

Execution:
- Actor: `codex`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `2026-09-09T22:00:00Z`
- End Time: `not_finished`

## Summary

Repaired six release-truth defects: exact-candidate V3-F inputs, live canonical status, #519 terminal projection, fourteen corrupt ownership links, eighteen YAML-disguised GCP-E JSON readbacks, and four empty failure artifacts.

## PVF Lane Truth
- Initial PVF lane: `docs_only`
- Planned PVF lane: `docs_only`
- Final PVF lane: `docs_only`
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
- Estimate error percent: `unknown`
- Completion state: `implementation_validated_pending_publication`
- Issue goal ref: `Issue #817 session goal active`
- Sprint goal ref: `Parent #522`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/817/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_collected`
- Variance note: `No estimate/actual metric pair was collected; variance is not inferred.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/817/cards/sor.md`
- Tracked implementation artifacts: `Release current-status projections and refresher; V3-F current packet; feature coverage ownership links; GCP-E retained JSON; #519 terminal projection; issue #817 focused validator.`
- Additional proof artifacts: `V3-F detached locked suite log and receipt; release-truth positive and three negative cases; current-status positive and nine negative cases.`

## Actions taken
- `Regenerated canonical status from exact Git bytes and read-only live issue truth; added candidate-aware validation and repaired ownership links.`
- `Normalized eighteen retained GCP-E readbacks to JSON and replaced four zero-byte files with truthful structured failure envelopes.`
- `Recorded #519 terminal projection without rewriting historical cards; refreshed V3-F source review and detached locked suite; resolved all three independent README findings.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; work remains on issue branch pending review and publication`
- Worktree-only paths remaining: `all issue #817 changes`
- Integration state: `worktree_only`
- Verification scope: `All six assigned #520 findings with positive and negative deterministic proof; exact-source V3-F full locked suite; no paid cloud rerun.`
- Integration method used: `pending native publication`
- Verification performed:
  - `Focused #817 positive/negative validators, native six-card validation, JSON parse sweep, exact-source V3-F locked suite, and git diff --check.`
    `Verifies the complete issue-branch artifact set and its exact-candidate bindings before native publication; hosted integration remains pending.`
- Result: `PASS in the bound issue worktree; native publication and hosted CI pending`

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
  - `python3 .csdlc/prepared/issues/817/validate-release-truth.py; python3 .csdlc/prepared/issues/817/validate-release-truth.py --negative; python3 .csdlc/prepared/issues/817/validate-release-truth.py --observe-github; python3 docs/milestones/v0.92.1/evidence/release/current-status/validate.py; python3 docs/milestones/v0.92.1/evidence/release/current-status/validate.py --negative; python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/validate.py; python3 docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/v3f-current/validate.py --negative; native csdlc validate; JSON parse sweep; git diff --check`
    `Authenticates exact #519 terminal source bytes and BLAKE3 state binding, optionally re-observes GitHub, rejects ten release-truth and fifteen V3-F tamper classes, binds failure stderr bytes and signatures while preserving unknown historical exit status, checks current status, parses retained GCP-E JSON, and validates lifecycle truth.`
- Results:
  - `PASS: receipt and live terminal verification; release-truth positive and 10 negatives; current-status positive and 9 negatives; 34 GCP-E JSON files nonempty and parseable; V3-F mapping positive and 15 negatives; detached locked suite 211 passed and 0 failed; native six-card validation; diff hygiene.`

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
      - "Focused release-truth, terminal source/live, failure stderr binding, current-status, V3-F, JSON, lifecycle, and diff-hygiene checks passed."
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
- Determinism tests executed: `Release-truth 10-case negative matrix, current-status 9-case negative matrix, and V3-F 15-case negative matrix.`
- Fixtures or scripts used: `.csdlc/prepared/issues/817/validate-release-truth.py; current-status/validate.py; v3f-current/validate.py and suite.log.`
- Replay verification (same inputs -> same artifacts/order): `Repeated focused validators produce the same classifications and exact denominator counts.`
- Ordering guarantees (sorting / tie-break rules used): `Issue observations, status rows, work packages, source maps, and V3-F scope are sorted or exact-map compared.`
- Artifact stability notes: `Candidate Git blobs, explicit hashes, exact source SHA, and immutable historical-card hashes bind retained truth.`

## Security / Privacy Checks
- Secret leakage scan performed: `Yes; changed evidence and suite log were checked for credentials and host-local paths.`
- Prompt / tool argument redaction verified: `No prompt payloads are published; retained command evidence exposes only bounded command classes and redacted paths.`
- Absolute path leakage check: `Passed; the V3-F validator rejects host-local suite-log prefixes.`
- Sandbox / policy invariants preserved: `Yes; all cloud reads were read-only and no paid provider mutation was performed.`

## Replay Artifacts
- Trace bundle path(s): `none; not required for deterministic release-evidence repair`
- Run artifact root: `.csdlc/evidence/817 and docs/milestones/v0.92.1/evidence/release`
- Replay command used for verification: `Run the three focused validators in positive and negative modes; add --observe-github for live #519 terminal readback.`
- Replay result: `passed`

## Artifact Verification
- Primary proof surface: `.csdlc/prepared/issues/817/validate-release-truth.py and v3f-current/validate.py`
- Required artifacts present: `true`
- Artifact schema/version checks: `All 34 GCP-E JSON artifacts parse; four adl.retained_command_failure.v1 envelopes bind stderr SHA-256, command/error signatures, and captured or explicitly unavailable exit status; terminal sources use native v3 schemas.`
- Hash/byte-stability checks: `SHA-256 binds terminal sources, historical cards, four failure stderr artifacts, suite log, mapping inputs, and current-status sources; BLAKE3 binds terminal state to receipt.`
- Missing/optional artifacts and rationale: `No new cloud execution or trace bundle was required; issue repairs retained and classified existing proof.`

## Decisions / Deviations
- `Used a tracked terminal projection and exact source copies rather than rewriting #519 historical cards.`
- `Did not rerun paid cloud workloads; normalized retained evidence without upgrading its proof class.`

## Follow-ups / Deferred work
- `Publish only after fresh independent exact-head PASS.`
- `Keep release authorization false until the parent #520 denominator is terminally resolved.`
