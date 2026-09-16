# scope-rebind-validator-recovery

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

Task ID: issue-1003
Run ID: issue-1003
Version: v0.92.2
Title: [v0.92.2][C-SDLC] Restore rebind and validator replacement after scope amendments
Branch: codex/1003-scope-rebind-validator-recovery
Card Status: ready
Status: in_progress
Generated: 2026-09-16T02:43:15.759077+00:00

Execution:
- Actor: `Pending execution; no result claimed.`
- Model: `Pending execution; no result claimed.`
- Provider: `Pending execution; no result claimed.`
- Start Time: `Pending execution; no result claimed.`
- End Time: `Pending execution; no result claimed.`

## Summary

Implemented and independently reviewed guarded rebind/head refresh and typed validator recovery. Local proof passed; publication and hosted CI remain pending.

## PVF Lane Truth
- Initial PVF lane: `csdlc`
- Planned PVF lane: `csdlc`
- Final PVF lane: `Pending execution; no result claimed.`
- Lane change reason: `Pending execution; no result claimed.`

## Issue Metrics Truth
- Expected runtime class: `Pending execution; no result claimed.`
- Estimated elapsed seconds: `Pending execution; no result claimed.`
- Actual elapsed seconds: `Pending execution; no result claimed.`
- Actual active work seconds: `Pending execution; no result claimed.`
- Estimated total tokens: `Pending execution; no result claimed.`
- Actual total tokens: `Pending execution; no result claimed.`
- Estimated validation seconds: `Pending execution; no result claimed.`
- Actual validation seconds: `Pending execution; no result claimed.`
- Actual PR wait seconds: `Pending execution; no result claimed.`
- Actual CI wait seconds: `Pending execution; no result claimed.`
- Budget source: `Pending execution; no result claimed.`
- Goal metrics data source: `Pending execution; no result claimed.`
- Goal metrics source ref: `Pending execution; no result claimed.`
- Data-source confidence: `Pending execution; no result claimed.`
- Estimate error percent: `Pending execution; no result claimed.`
- Completion state: `implementation_complete_pending_review_ci`
- Issue goal ref: `Worker10 #1003 and #1006 implementation`
- Sprint goal ref: `Not a sprint child; follow-up tooling repair`
- Goal metrics rollup ref: `session goal`
- Validation planning prompt: `.csdlc/issues/1003/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `Pending execution; no result claimed.`
- Variance analysis completed: `Pending execution; no result claimed.`
- Variance category: `Pending execution; no result claimed.`
- Variance note: `Pending execution; no result claimed.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/1003/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3 native intent/context, semantic policy/protocol, semantic_local_proof tests, scope_rebind_pvf.json, INTENT_COMMANDS.md`
- Additional proof artifacts: `Pending execution; no result claimed.`

## Actions taken
- `Reproduced both original failures in installed candidate fixtures before fixing source.`
- `Added exact registration/authority/recovery checks and blocked-doctor refusal.`
- `Ran semantic owner, installed intent, formatting and strict Clippy proof.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `Pending execution; no result claimed.`
- Worktree-only paths remaining: `All implementation paths remain on the issue branch.`
- Integration state: `worktree_only`
- Verification scope: `Pending execution; no result claimed.`
- Integration method used: `Pending execution; no result claimed.`
- Verification performed:
  - `Pending execution; no result claimed.`
    `Pending execution; no result claimed.`
- Result: `Not published or merged yet`

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
  - `cargo test --manifest-path csdlc-v3/Cargo.toml --lib --test semantic_local_proof; cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_intent_commands; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; cargo fmt --check; git diff --check`
    `Actual isolated installed CLI transitions and negative guards; no live issue mutation in tests.`
- Results:
  - `143 library tests, 12 installed semantic-local tests and 43 installed intent regressions passed; strict Clippy and formatting passed. CI pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: Pending execution; no result claimed.
    checks_run:
      - "Pending execution; no result claimed."
  determinism:
    status: Pending execution; no result claimed.
    replay_verified: Pending execution; no result claimed.
    ordering_guarantees_verified: Pending execution; no result claimed.
  security_privacy:
    status: Pending execution; no result claimed.
    secrets_leakage_detected: Pending execution; no result claimed.
    prompt_or_tool_arg_leakage_detected: Pending execution; no result claimed.
    absolute_path_leakage_detected: Pending execution; no result claimed.
  artifacts:
    status: Pending execution; no result claimed.
    required_artifacts_present: Pending execution; no result claimed.
    schema_changes:
      present: Pending execution; no result claimed.
      approved: Pending execution; no result claimed.
```

## Determinism Evidence
- Determinism tests executed: `Deterministic installed rebind, validator replacement, stale-request, wrong-branch and blocked-doctor regressions.`
- Fixtures or scripts used: `Pending execution; no result claimed.`
- Replay verification (same inputs -> same artifacts/order): `Pending execution; no result claimed.`
- Ordering guarantees (sorting / tie-break rules used): `Pending execution; no result claimed.`
- Artifact stability notes: `Pending execution; no result claimed.`

## Security / Privacy Checks
- Secret leakage scan performed: `Pending execution; no result claimed.`
- Prompt / tool argument redaction verified: `Pending execution; no result claimed.`
- Absolute path leakage check: `Pending execution; no result claimed.`
- Sandbox / policy invariants preserved: `Pending execution; no result claimed.`

## Replay Artifacts
- Trace bundle path(s): `Pending execution; no result claimed.`
- Run artifact root: `Pending execution; no result claimed.`
- Replay command used for verification: `Pending execution; no result claimed.`
- Replay result: `Pending execution; no result claimed.`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/1003/LOCAL_PROOF.json`
- Required artifacts present: `Local proof retained; hosted proof pending.`
- Artifact schema/version checks: `Pending execution; no result claimed.`
- Hash/byte-stability checks: `Pending execution; no result claimed.`
- Missing/optional artifacts and rationale: `Pending execution; no result claimed.`

## Decisions / Deviations
- `Pending execution; no result claimed.`
- `Pending execution; no result claimed.`

## Follow-ups / Deferred work
- `Publish reviewed separate PR closing #1003, require full CI and explicit merge authorization.`
- `#1006 remains separately owned and reviewed.`
