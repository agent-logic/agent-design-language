# sor-authority-notice

Canonical Template Source: `docs/templates/prompts/1.0.4/sor.md`

Authority notice: C-SDLC v3 is operational after #505 / PR #591; authenticated canonical selector and reconciliation receipt validation are required. Missing or stale proof suspends authority.

Legacy `pr` editor routes are historical/retired compatibility orientation,
not current lifecycle authority.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-0804
Run ID: issue-0804
Version: v0.92.1
Title: [v0.92.1][defect] Align active SOR template authority notice with C-SDLC v3
Branch: codex/804-sor-authority-notice
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-09T18:25:45.944998+00:00

Execution:
- Actor: `planning-5`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `not_collected`
- End Time: `not_finished`

## Summary

Active SOR notice and schema now match operational v3 authority. Native-rendered local SOR validates; historical versions and unrelated issue cards unchanged.

## PVF Lane Truth
- Initial PVF lane: `prompt_template`
- Planned PVF lane: `prompt_template`
- Final PVF lane: `prompt_template`
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
- Goal metrics source ref: `Current issue goal; no per-phase metrics snapshot`
- Data-source confidence: `unknown`
- Estimate error percent: `not_collected`
- Completion state: `implementation_validated_pending_publication`
- Issue goal ref: `Issue #804 session goal in current Codex task`
- Sprint goal ref: `not_applicable: standalone defect`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/804/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_collected`
- Variance note: `No estimated/actual pair collected; no precision inferred.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/804/cards/sor.md`
- Tracked implementation artifacts: `docs/templates/prompts/1.0.4/sor.md; docs/templates/prompts/1.0.4/schemas/sor.structure.json`
- Additional proof artifacts: `.csdlc/evidence/804/authority-proof.json`

## Actions taken
- `Matched active SOR template and schema authority notice to current registry.`
- `Rendered all local issue cards via native edit and validated all six.`
- `Ran focused schema and notice checks and independent review.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none before merge`
- Worktree-only paths remaining: `Issue branch changes await PR integration.`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree`
- Integration method used: `PR publication pending`
- Verification performed:
  - `git diff --check`
    `Checked changed whitespace; no main integration claim.`
- Result: `Not merged`

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo (`git checkout BRANCH -- PATH` or commit + cherry-pick).
- If artifacts exist only in the worktree, the task is NOT complete.
- Integration state describes lifecycle state of the integrated artifact set, not where verification happened.
- Verification scope describes where the verification commands were run.
- worktree_only means at least one required path still exists only outside the main repository path.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By typed `csdlc-finish`, `Status` should normally be `DONE` (or `FAILED` if the run failed and the record is documenting that failure).

## Validation
- Validation commands and their purpose:
  - `Native csdlc edit and validate using issue-local requests and active current.json; python3 adl/tools/test_prompt_template_structure_schemas.py; git diff --check; authority parity assertions retained in .csdlc/evidence/804/authority-proof.json`
    `Checks active schema readability, rendering, authority parity and bounded negative stale-notice fixture.`
- Results:
  - `PASS: native six-card validation, Python six-schema smoke, authority consistency, schema field parity and whitespace checks.`

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
      - "Native six-card validation and focused prompt-template proof"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: false
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed: `Fixed-input notice/hash/schema checks`
- Fixtures or scripts used: `.csdlc/evidence/804/authority-proof.json`
- Replay verification (same inputs -> same artifacts/order): `Native renderer equality validated against current template and values.`
- Ordering guarantees (sorting / tie-break rules used): `No ordering behavior changed.`
- Artifact stability notes: `Only active notice metadata and template prose changed.`

## Security / Privacy Checks
- Secret leakage scan performed: `Diff inspected; public template notice only.`
- Prompt / tool argument redaction verified: `No credentials in artifacts; approved token path only used operationally.`
- Absolute path leakage check: `Implementation/proof use repo-relative paths; native binding retains required worktree identity.`
- Sandbox / policy invariants preserved: `Bound FastWork worktree; no primary issue artifacts.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable: no runtime trace`
- Run artifact root: `.csdlc/evidence/804`
- Replay command used for verification: `Native csdlc edit and validate using issue-local requests and active current.json; python3 adl/tools/test_prompt_template_structure_schemas.py; git diff --check; authority parity assertions retained in .csdlc/evidence/804/authority-proof.json`
- Replay result: `Focused checks passed`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/804/authority-proof.json`
- Required artifacts present: `true`
- Artifact schema/version checks: `Native structure validation and active Python schema smoke passed.`
- Hash/byte-stability checks: `Template and schema SHA256 hashes retained in proof.`
- Missing/optional artifacts and rationale: `No runtime/cloud proof needed for notice-only correction.`

## Decisions / Deviations
- `Edit active 1.0.4 only, as explicitly scoped by #804.`
- `Use focused template checks instead of broad runtime tests.`

## Follow-ups / Deferred work
- `Publish independently reviewed commit and verify required CI.`
- `Merge and postmerge closeout remain asynchronous.`
