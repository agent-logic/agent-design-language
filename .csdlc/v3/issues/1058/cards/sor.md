# v0922-installed-local-review-agent

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

Task ID: issue-1058
Run ID: issue-1058
Version: v0.92.2
Title: [v0.92.2][CF-AGENT] Run website-controlled reviews through an installed local agent
Branch: codex/1058-v0922-installed-local-review-agent
Card Status: draft
Status: in_progress
Generated: 2026-09-16T20:16:52.878587+00:00

Execution:
- Actor: `Planning #7.3`
- Model: `not_run`
- Provider: `not_run`
- Start Time: `not_run`
- End Time: `not_run`

## Summary

Agent recovery, model identity, aggregate budget and unpair findings repaired; draft PR1066 remains unaccepted until required native/CI and real product evidence settle.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `not_run`

## Issue Metrics Truth
- Expected runtime class: `not_run`
- Estimated elapsed seconds: `not_run`
- Actual elapsed seconds: `not_run`
- Actual active work seconds: `not_run`
- Estimated total tokens: `not_run`
- Actual total tokens: `not_run`
- Estimated validation seconds: `not_run`
- Actual validation seconds: `not_run`
- Actual PR wait seconds: `not_run`
- Actual CI wait seconds: `not_run`
- Budget source: `not_run`
- Goal metrics data source: `not_run`
- Goal metrics source ref: `not_run`
- Data-source confidence: `not_run`
- Estimate error percent: `not_run`
- Completion state: `in_progress`
- Issue goal ref: `Active Sprint 10 #936 goal explicitly requested by operator; no issue goal replaces it`
- Sprint goal ref: `https://github.com/agent-logic/agent-design-language/issues/936`
- Goal metrics rollup ref: `Future #1058 SOR`
- Validation planning prompt: `.csdlc/issues/1058/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `not_run`
- Variance analysis completed: `not_run`
- Variance category: `not_run`
- Variance note: `not_run`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/1058/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/agent.rs; adl/src/bin/codefriend_agent.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/Cargo.toml; adl/tests/codefriend_agent.rs; adl/tests/fixtures/codefriend/agent/PVF.json; docs/codefriend/LOCAL_AGENT.md`
- Additional proof artifacts: `not_run`

## Actions taken
- `Implemented outbound pairing, private credentials, locally pinned consent, durable reservations, gateway lanes through existing runner and result forwarding.`
- `Frozen candidate94180a9453 passed58 tests:32agent14review11server1CLI; focused clippy passed. Tests use controlled loopback providers, not real product acceptance.`
- `Resolved four user P2 findings and review followups: acknowledged-operation reconnect, actual candidate/model attribution, aggregate report budget, and immediate unpair payload cleanup. Independent exact-head source review passed94180a9453.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `adl/src/codefriend/agent.rs; adl/src/bin/codefriend_agent.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/Cargo.toml; adl/tests/codefriend_agent.rs; adl/tests/fixtures/codefriend/agent/PVF.json; docs/codefriend/LOCAL_AGENT.md`
- Integration state: `pr_open`
- Verification scope: `Local component and existing runner regression evidence only`
- Integration method used: `Draft PR1066 stacked on PR1063; reviewed server repair68750d357f merged. Website PR5 at34319df retains identities and approval binding.`
- Verification performed:
  - `not_run`
    `not_run`
- Result: `not_merged`

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
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_agent --test codefriend_review; cargo clippy --manifest-path adl/Cargo.toml --bin codefriend-agent --test codefriend_agent -- -D warnings`
    `58 controlled local tests and focused clippy passed at94180a9453. Native proof/publication refresh and CI remain pending; real OAuth/provider/platform acceptance is outstanding.`
- Results:
  - `component_tests_passed_product_acceptance_outstanding`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: component_passed_product_unproven
    checks_run:
      - "not_run"
  determinism:
    status: not_run
    replay_verified: not_run
    ordering_guarantees_verified: not_run
  security_privacy:
    status: not_run
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: not_run
    absolute_path_leakage_detected: not_run
  artifacts:
    status: Component artifacts present; live acceptance missing
    required_artifacts_present: not_run
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `not_run`
- Fixtures or scripts used: `not_run`
- Replay verification (same inputs -> same artifacts/order): `not_run`
- Ordering guarantees (sorting / tie-break rules used): `not_run`
- Artifact stability notes: `not_run`

## Security / Privacy Checks
- Secret leakage scan performed: `not_run`
- Prompt / tool argument redaction verified: `not_run`
- Absolute path leakage check: `not_run`
- Sandbox / policy invariants preserved: `not_run`

## Replay Artifacts
- Trace bundle path(s): `not_run`
- Run artifact root: `.csdlc/evidence/1058`
- Replay command used for verification: `not_run`
- Replay result: `not_run`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_agent.rs; adl/tests/codefriend_review.rs`
- Required artifacts present: `false`
- Artifact schema/version checks: `not_run`
- Hash/byte-stability checks: `not_run`
- Missing/optional artifacts and rationale: `not_run`

## Decisions / Deviations
- `Agent Logic model access used; source evidence is explicitly disclosed in local consent and website result upload.`
- `No new issue goal: operator requires active Sprint 10 goal. No paid/deployment effects inferred.`

## Follow-ups / Deferred work
- `Complete #1057 website-side contract and real installed/provider acceptance before component acceptance.`
- `Integrate accepted components in #914; independently qualify #915. No merge-ready claim.`
