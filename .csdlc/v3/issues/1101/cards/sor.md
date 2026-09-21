# codefriend-update-cycle-activity-executors

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

Task ID: issue-1101
Run ID: issue-1101
Version: 1.0.5
Title: [v0.92.2][CF-ACTIVITIES] Generate documentation, diagrams and tests for update cycles
Branch: codex/1101-codefriend-update-cycle-activity-executors
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-20

Execution:
- Actor: `Planning #5`
- Model: `Codex`
- Provider: `OpenAI`
- Start Time: `not_collected`
- End Time: `not_collected`

## Summary

Implemented versioned repository update-cycle plans and source-bound documentation, Mermaid diagram, and test proposal results for hosted and paired-local CodeFriend execution while preserving the legacy review-only protocol. Independent exact-head review, publication, CI, merge, and website consumption remain pending.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `not_applicable: the implementation remained in the planned deterministic runtime lane`

## Issue Metrics Truth
- Expected runtime class: `bounded component implementation and focused Rust validation`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `unknown`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `0`
- Actual CI wait seconds: `0`
- Budget source: `not_collected`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `not_collected`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implementation_complete_review_pending`
- Issue goal ref: `Active issue #1101 implementation and reviewed-green PR publication goal`
- Sprint goal ref: `CodeFriend Beta 1 update-cycle prerequisite for codefriend.ai issue #6`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/1101/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown: estimate and actual telemetry were not collected`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No numeric estimate/actual pair exists; no variance percentage is claimed.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/1101/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/activities.rs; adl/src/codefriend/agent.rs; adl/src/codefriend/server.rs; adl/src/codefriend/review/runner.rs; adl/tests/codefriend_update_cycle.rs; adl/tests/codefriend_agent.rs; adl/tests/codefriend_server.rs; adl/tests/fixtures/codefriend/update-cycle/PVF.json; docs/codefriend/SERVER.md; docs/codefriend/LOCAL_AGENT.md`
- Additional proof artifacts: `Focused deterministic component tests and the issue-local PVF manifest; no live provider or deployment artifact is claimed.`

## Actions taken
- `Added strict versioned plan, input-manifest, proposal-output, activity-result, and aggregate-result contracts for independently selected work.`
- `Integrated hosted execution and a single durable paired-local cycle operation with cancellation, retention, observed model identity, exact request binding, and no-replay behavior.`
- `Preserved the original review-only wire shapes and added deterministic compatibility, failure, admission, and result-validation tests.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none: the implementation is committed only on the bound issue branch`
- Worktree-only paths remaining: `All issue #1101 implementation, tests, docs, and lifecycle records remain on the bound branch pending PR review and merge.`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree`
- Integration method used: `Three commits on the bound issue branch; no main checkout edits and no merge.`
- Verification performed:
  - `git status --short --branch; git diff --check origin/main...HEAD`
    `Verified the bound branch identity, clean worktree, and patch whitespace hygiene without claiming main integration.`
- Result: `Committed through ddc209c40be8266debe24562c823b8d80c6a1f74; not independently reviewed, published, merged, or deployed.`

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
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_update_cycle --test codefriend_server --test codefriend_agent --test codefriend_agent_receipt --test codefriend_agent_publication --test codefriend_integration; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_update_cycle --test codefriend_server --test codefriend_agent -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --all -- --check; git diff --check origin/main...HEAD`
    `Exercises activity selection and binding, malformed and failed outputs, hosted admission, paired-local single-operation replay protection, legacy agent/report compatibility, publication receipts, hosted integration, formatting, and patch hygiene.`
- Results:
  - `75 focused and compatibility tests passed with zero failures; strict focused Clippy, formatting, and diff hygiene passed. Independent exact-head review remains pending.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed_with_remaining_review_gates
    checks_run:
      - "75 focused CodeFriend tests, strict focused Clippy, formatting, and diff hygiene passed"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: passed_for_component_scope
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: true: issue #1101 explicitly requires versioned update-cycle contracts
```

## Determinism Evidence
- Determinism tests executed: `codefriend_update_cycle, codefriend_server, codefriend_agent, codefriend_agent_receipt, codefriend_agent_publication, and codefriend_integration`
- Fixtures or scripts used: `Local Git repositories, controlled clocks, bounded filesystem fixtures, loopback HTTP, and injected providers; no external provider.`
- Replay verification (same inputs -> same artifacts/order): `Paired-local cycle redelivery retained one gateway dispatch and the exact report; operation IDs bind agent, run, cycle, and plan digest.`
- Ordering guarantees (sorting / tie-break rules used): `Plans require canonical review, documentation, diagrams, tests ordering and reject duplicates; results preserve the selected order.`
- Artifact stability notes: `Input, plan, output, review, report, and request digests bind retained typed bytes; observed timestamps may vary without changing the execution identity comparison.`

## Security / Privacy Checks
- Secret leakage scan performed: `Output validation rejects recognized credential content and unsafe paths; focused fixtures contain no real credentials.`
- Prompt / tool argument redaction verified: `Prompts contain bounded admitted source and fixed instructions only; provider failures become fixed redacted activity error codes.`
- Absolute path leakage check: `Generated outputs reject unsafe paths; tracked docs use repository-relative references. The lifecycle worktree field is template-required local coordination metadata.`
- Sandbox / policy invariants preserved: `Generated content remains proposal-only and grants no source mutation or publication authority.`

## Replay Artifacts
- Trace bundle path(s): `not_applicable: deterministic component tests did not retain a live provider trace bundle`
- Run artifact root: `adl/target test fixtures only; disposable and not tracked`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_agent local_update_cycle_uses_one_durable_gateway_operation_and_preserves_activity_results -- --exact`
- Replay result: `Passed with one model gateway dispatch across initial delivery and redelivery.`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_update_cycle.rs; adl/tests/codefriend_server.rs; adl/tests/codefriend_agent.rs`
- Required artifacts present: `Versioned contracts, hosted and paired-local integration, compatibility tests, protocol docs, and PVF manifest are present on the bound branch.`
- Artifact schema/version checks: `Strict serde unknown-field rejection and explicit schema constants validate plan, manifest, output, aggregate result, command, and report shapes.`
- Hash/byte-stability checks: `Plan, manifest, output, operation request, gateway result, and local report digests are recomputed and compared by focused tests.`
- Missing/optional artifacts and rationale: `No rendered Mermaid, executed generated tests, measured coverage, source mutation, publication, live provider, installed service, or deployment proof is claimed by this issue.`

## Decisions / Deviations
- `Use one durable paired-local gateway operation for the complete plan so normal reconnection never requires a second update-cycle dispatch.`
- `Keep legacy review-only request and report serialization unchanged when cycle is absent.`

## Follow-ups / Deferred work
- `Obtain mandatory independent exact-head review and fix every actionable finding before publication.`
- `After #1101 is reviewed and merged, update codefriend.ai PR #9 to enable the four activities and consume the new result contract.`
