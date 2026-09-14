# merge-graphql-observation

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

Task ID: issue-0975
Run ID: issue-0975
Version: v0.92.2
Title: [v0.92.2][C-SDLC] Repair native merge GraphQL observation transport
Branch: codex/975-v0922-merge-graphql-observation
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-14T17:17:30.658534+00:00

Execution:
- Actor: `Codex delegated agent review_767_refresh`
- Model: `unknown`
- Provider: `OpenAI`
- Start Time: `unknown`
- End Time: `not_applicable: publication pending`

## Summary

Implemented bounded JSON POST for both merge GraphQL observations; read-only production probes now return PR971 without truncation.

## PVF Lane Truth
- Initial PVF lane: `deterministic_local`
- Planned PVF lane: `deterministic_local`
- Final PVF lane: `deterministic_local`
- Lane change reason: `No lane change; native suites replace legacy wrapper plan as documented.`

## Issue Metrics Truth
- Expected runtime class: `short`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `no explicit budget`
- Goal metrics data source: `goal tool snapshot at initial handoff`
- Goal metrics source ref: `codex-goal:01a0875a-799c-7422-a9ae-e1eefbd1c932:975`
- Data-source confidence: `partial: initial handoff only; current totals not collected`
- Estimate error percent: `unknown`
- Completion state: `implementation_complete_review_pending`
- Issue goal ref: `codex-goal:01a0875a-799c-7422-a9ae-e1eefbd1c932:975`
- Sprint goal ref: `not_applicable: delegated tooling repair`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/975/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `not_applicable: estimates unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `Estimates and final metrics unavailable; no zero variance implied.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/975/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3/src/adapters/mod.rs; csdlc-v3/tests/support/intent_fixture.rs; .csdlc/issues/975; .csdlc/evidence/975/VALIDATION.md`
- Additional proof artifacts: `.csdlc/evidence/975/VALIDATION.md`

## Actions taken
- `Reproduced original adapter truncation using read-only production observations.`
- `Replaced GET URL query with bounded JSON POST over private curl stdin.`
- `Added focused production adapter and size-bound regressions; recorded PVF and native validation.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none: PR #976 remains open`
- Worktree-only paths remaining: `PR #976 contains csdlc-v3/src/adapters/mod.rs, csdlc-v3/tests/support/intent_fixture.rs, .csdlc/issues/975 and .csdlc/evidence/975/VALIDATION.md until merge.`
- Integration state: `pr_open`
- Verification scope: `bound issue worktree; no primary edits`
- Integration method used: `Issue branch published as PR #976; merge pending.`
- Verification performed:
  - `git status --short --branch; git rev-parse HEAD`
    `Confirmed assigned branch and committed work; not integration into main.`
- Result: `pr_open`

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
  - `cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_adapter_tests`
    `Four final adapter tests passed; full suite results listed in proof packet.`
- Results:
  - `Native library, adapter, operational CLI, publication and six-card validation passed as recorded in .csdlc/evidence/975/VALIDATION.md. The formerly failing installed merge/finish/cleanup scenario passed after fixture correction. Hosted replacement CI remains pending.`

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
      - ".csdlc/evidence/975/VALIDATION.md"
  determinism:
    status: passed for deterministic local fixtures
    replay_verified: true
    ordering_guarantees_verified: not_applicable: no ordering change
  security_privacy:
    status: bounded adapter checks passed; no comprehensive security audit claimed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: not_applicable: local worktree binding is explicit lifecycle metadata
  artifacts:
    status: present
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `merge_adapter_tests and existing merge eligibility/linkage negative matrices`
- Fixtures or scripts used: `Synthetic curl subprocess fixture in adapters/mod.rs; ignored read-only probe source.`
- Replay verification (same inputs -> same artifacts/order): `Repeated adapter fixtures use fixed synthetic responses; live observation is not deterministic replay.`
- Ordering guarantees (sorting / tie-break rules used): `No new lifecycle ordering semantics; existing durable-intent and merge tests passed.`
- Artifact stability notes: `Native renderer and digest validation passed; generated cards remain reproducible from values.`

## Security / Privacy Checks
- Secret leakage scan performed: `Focused synthetic credential redaction assertions passed; no broad secret scan run.`
- Prompt / tool argument redaction verified: `Production adapter fixture verifies query/credential absent from argv and response secret redaction before truncation.`
- Absolute path leakage check: `Issue records retain required local binding; no raw response or provider output retained.`
- Sandbox / policy invariants preserved: `Only bound975 worktree edited; primary clean; no GitHub mutation.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable: no Runtime trace produced`
- Run artifact root: `.adl/issue975 (ignored local probes and private binary)`
- Replay command used for verification: `cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_adapter_tests`
- Replay result: `four passed`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/975/VALIDATION.md`
- Required artifacts present: `true`
- Artifact schema/version checks: `Native six-card values/render/structure/digest validation passed.`
- Hash/byte-stability checks: `Native lifecycle digest validated; implementation unchanged during card remediation.`
- Missing/optional artifacts and rationale: `No raw remote payload or secrets retained; no runtime/cloud workload artifacts required.`

## Decisions / Deviations
- `GET returned introspection; POST transports the generated query while keeping mutation guards unchanged.`
- `Legacy lane wrapper still starts v2Gate10A; focused native v3 suites used instead.`

## Follow-ups / Deferred work
- `Complete exact-head rereview and replacement hosted CI, then merge PR #976 through native C-SDLC.`
- `Install accepted native owner from merged main and replay the authorized PR #971 merge.`
