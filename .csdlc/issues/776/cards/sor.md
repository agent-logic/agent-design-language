# versioned-template-structure-schemas

Canonical Template Source: `docs/templates/prompts/1.0.4/sor.md`

Authority notice: V3-F/#505 is the pending tooling changeover decision; until
that operator-reviewed cutover is approved, merged, and terminally reconciled,
C-SDLC v2 remains live authority.
Legacy `pr` editor routes are historical/retired compatibility orientation,
not current lifecycle authority.

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-0776
Run ID: issue-0776
Version: 1.0.4
Title: [v0.92.1][TAIL-06.17][csdlc] Resolve structure schemas beside versioned templates
Branch: codex/776-versioned-template-structure-schemas
Card Status: ready
Status: IMPLEMENTED
Generated: <timestamp>

Execution:
- Actor: `codex:worker-8`
- Model: `gpt-5-codex`
- Provider: `OpenAI`
- Start Time: `2026-09-09`
- End Time: `in_progress`

## Summary

Implemented native versioned structure-schema resolution and real-current-registry validation across SIP, STP, SPP, VPP, SRP, and SOR. Exact-head independent review and publication remain pending.

## PVF Lane Truth
- Initial PVF lane: `csdlc-v3-local-commands`
- Planned PVF lane: `csdlc-v3-local-commands`
- Final PVF lane: `csdlc-v3-local-commands`
- Lane change reason: `No lane change; the regression exposed additional schema semantics within the same native local validation surface.`

## Issue Metrics Truth
- Expected runtime class: `focused`
- Estimated elapsed seconds: `900`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `12000`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `300`
- Actual validation seconds: `under_10`
- Actual PR wait seconds: `not_applicable`
- Actual CI wait seconds: `not_applicable`
- Budget source: `issue-776 bounded plan`
- Goal metrics data source: `command results and issue goal`
- Goal metrics source ref: `Issue #776 session goal`
- Data-source confidence: `medium`
- Estimate error percent: `unknown`
- Completion state: `implemented_review_pending`
- Issue goal ref: `Issue #776 session goal`
- Sprint goal ref: `v0.92.1 TAIL-06.17`
- Goal metrics rollup ref: `not_applicable`
- Validation planning prompt: `.csdlc/issues/776/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `false_for_known_validation_pair`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `Unknown session metrics are not treated as zero variance.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/776/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3/src/commands/local/mod.rs; csdlc-v3/tests/local_commands.rs; .csdlc/issues/776`
- Additional proof artifacts: `Native command outputs retained in session; no separate tracked log required.`

## Actions taken
- `Initialized and bound issue #776 through authenticated native C-SDLC v3.`
- `Corrected sibling schema resolution and preserved schema identity, heading, locked-line, and applicable scaffold validation.`
- `Added and ran a real-current-registry regression covering all six card kinds plus the complete local_commands target.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; PR publication is pending`
- Worktree-only paths remaining: `all issue #776 implementation and lifecycle paths`
- Integration state: `worktree_only`
- Verification scope: `bound issue #776 worktree`
- Integration method used: `pending typed publication`
- Verification performed:
  - `git diff --check`
    `Verified no whitespace errors before review preparation.`
- Result: `Implementation is locally proven but not yet published or merged.`

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
  - `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`
    `Executed the complete focused native local-command integration target, including the real six-card current-registry regression.`
- Results:
  - `PASS: 34 passed, 0 failed; exact regression separately passed 1/1; git diff --check passed; rustfmt was applied after its check reported only mechanical wrapping.`

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
      - "34/34 local_commands tests passed, including all-six current registry structure validation"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: not_applicable
  security_privacy:
    status: passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: present_in_bound_worktree
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `real-current-registry regression and complete local_commands target`
- Fixtures or scripts used: `csdlc-v3/tests/local_commands.rs operational authority fixture plus docs/templates/prompts/current.json`
- Replay verification (same inputs -> same artifacts/order): `Repeated exact regression reached the same all-six pass after bounded corrections.`
- Ordering guarantees (sorting / tie-break rules used): `Not applicable to path resolution.`
- Artifact stability notes: `No template or schema bytes changed.`

## Security / Privacy Checks
- Secret leakage scan performed: `No credential surfaces touched; diff remains code, tests, and typed issue records only.`
- Prompt / tool argument redaction verified: `No prompts, credentials, or raw provider arguments recorded.`
- Absolute path leakage check: `Tracked source/test changes use repository-relative paths; lifecycle worktree identity is typed state.`
- Sandbox / policy invariants preserved: `All implementation occurred in the bound FastWork worktree; no raw GitHub lifecycle write was used.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable`
- Run artifact root: `csdlc-v3/target local test fixtures are disposable build output`
- Replay command used for verification: `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands current_registry_versioned_templates_resolve_all_structure_schemas -- --exact`
- Replay result: `PASS: 1 passed, 0 failed`

## Artifact Verification
- Primary proof surface: `csdlc-v3/tests/local_commands.rs real-current-registry regression`
- Required artifacts present: `Production fix, regression, and typed lifecycle bundle are present in the bound worktree.`
- Artifact schema/version checks: `All six current 1.0.4 structure schemas were loaded and identity-bound by native validation.`
- Hash/byte-stability checks: `No template or schema content changes.`
- Missing/optional artifacts and rationale: `Hosted CI and PR evidence are pending publication.`

## Decisions / Deviations
- `The path-only issue diagnosis was refined after the real registry test showed scaffold_lines is a cross-template vocabulary.`
- `Structure validation now checks applicable scaffold lines plus declared headings and locked lines; no schema content was altered.`

## Follow-ups / Deferred work
- `Obtain bounded exact-head independent review and resolve all findings.`
- `Publish through native v3, shepherd hosted checks, and coordinate the tested commit to #758; do not merge without operator authorization.`
