# issue-855-provider-neutral-dynamic-agent-lifecycle

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

Task ID: issue-0855
Run ID: issue-0855
Version: v0.92.2
Title: [v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle
Branch: codex/855-provider-neutral-lifecycle
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-12

Execution:
- Actor: `Codex issue855 team`
- Model: `not_collected`
- Provider: `OpenAI`
- Start Time: `not_collected`
- End Time: `not_collected`

## Summary

Implemented provider-neutral dynamic agent lifecycle and published draft PR #964 against main. Local installed five-provider and hosted-topology fixtures passed. Actual hosted live02 dispatched three OpenAI requests: operator reply and signed A2A/peer reply succeeded, continuation failed; Anthropic and Vertex were not called. Original continuation category was discarded, so underlying live failure remains unknown. Bounded category propagation and prompt phase clarification pass deterministic regression. Required CI runtime coverage failed and is under bounded repair. Full hosted acceptance and current CI remain incomplete; no merge readiness claim.

## PVF Lane Truth
- Initial PVF lane: `provider`
- Planned PVF lane: `provider`
- Final PVF lane: `provider`
- Lane change reason: `No lane change; installed fixture and hosted proof are distinct required integration surfaces.`

## Issue Metrics Truth
- Expected runtime class: `not_collected`
- Estimated elapsed seconds: `not_collected`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `not_collected`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `not_collected`
- Actual CI wait seconds: `not_collected`
- Budget source: `operator-approved bounded hosted envelope; no implementation token budget`
- Goal metrics data source: `goal tool; final snapshot pending`
- Goal metrics source ref: `not_collected`
- Data-source confidence: `in_progress`
- Estimate error percent: `not_collected`
- Completion state: `in_progress`
- Issue goal ref: `01a0941a-fa4c-7bc1-8585-7b7e48e2fe00`
- Sprint goal ref: `issue-928`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/855/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `false`
- Variance category: `not_collected`
- Variance note: `Final goal metrics not yet collected; do not infer elapsed or token variance.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/855/cards/sor.md`
- Tracked implementation artifacts: `adl-provider-core; ADL compatibility provider/model/CLI facades; runtime kernel registry/control/assembly/telemetry/roster; Cargo owner locks; OpenAPI; CI provider lane; lifecycle harness and docs`
- Additional proof artifacts: `adl-provider-core/PROOF_INVENTORY.md; docs/runtime-v3/fixtures/issue855/INSTALLED_PROOF_RUN09.json; ignored .adl/issue855 records preserve all attempts and source/binary hashes.`

## Actions taken
- `Composed actual kernel with canonical provider registry and validated sidecar snapshots before admission restoration.`
- `Proved provider transports, explicit failure recovery, signed A2A, cancellation accounting, config compatibility and bounded local subprocess supervision.`
- `Corrected independent review findings and prepared exact-binary final installed and authorized hosted demonstration.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; primary main remains inspection-only`
- Worktree-only paths remaining: `All issue changes remain on the bound branch before PR merge.`
- Integration state: `worktree_only`
- Verification scope: `bound_worktree`
- Integration method used: `not_collected`
- Verification performed:
  - `Native review, authenticated github-pr draft creation and publish reconciliation passed for PR964 at189aff00; base main and draft status verified.`
    `not_collected`
- Result: `not_collected`

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
  - `See adl-provider-core/PROOF_INVENTORY.md for precise cargo commands and PVF classification; .adl/issue855 contains append-only per-attempt logs.`
    `Full leaf94/94 plus one Runtime mock constructor regression; selected kernel control85/85, ADL compatibility18/18, CLI 7/7, API11/11; affectedClippy and CIpath/runtimecontracts pass; installedfixture5/5 and hostedtopologyfixture3/3 at 61095.`
- Results:
  - `Continuation remediation: actual production dispatch regression 1/1, prompt contracts 2/2 and kernel Clippy pass. Prior fixture and source proofs retained. PR964 CI runtime coverage failed; other observed completed checks passed, remaining jobs pending. Actual hosted live02 failed after 3 OpenAI requests; no paid retry performed.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: in_progress
    checks_run:
      - "Local selected proof passes; required installed/hosted/CI completion pending"
  determinism:
    status: partial
    replay_verified: false
    ordering_guarantees_verified: true
  security_privacy:
    status: in_progress
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: in_progress
    required_artifacts_present: false
    schema_changes:
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed: `Local deterministic provider transport and lifecycle tests; full matrix draft fixture 5/5, final binary proof pending.`
- Fixtures or scripts used: `adl/tools/issue855_provider_lifecycle.py; provider-core and kernel fixtures`
- Replay verification (same inputs -> same artifacts/order): `Not claimed: randomized ephemeral TLS and runtime identities produce different artifact bytes.`
- Ordering guarantees (sorting / tie-break rules used): `Canonical signed sender/recipient names and actual A2A work identity; generation-guarded definition/health publication.`
- Artifact stability notes: `Proof manifests retain binary hashes and source identity; prior failed attempts remain unchanged.`

## Security / Privacy Checks
- Secret leakage scan performed: `Value-free changed-tracked-text pattern scan checked67paths with no credential-shaped values or unjustified host paths (.adl/issue855/tracked-redaction-scan-1.json). Behavioral redaction tests separately pass.`
- Prompt / tool argument redaction verified: `Provider accounting records counts/estimates, not prompts or credentials; fixtures retain bounded synthetic data only.`
- Absolute path leakage check: `Final tracked scan pending; execution checkout location is required in native binding metadata.`
- Sandbox / policy invariants preserved: `Only bound issue checkout edited; process-only credential references; hosted HTTPS and private-network checks; no billing/resource changes.`

## Replay Artifacts
- Trace bundle path(s): `.adl/issue855`
- Run artifact root: `.adl/issue855`
- Replay command used for verification: `See named fixture driver commands in docs/runtime-v3/PROVIDER_NEUTRAL_LIFECYCLE.md`
- Replay result: `Final proof pending; draft fixture matrix passed.`

## Artifact Verification
- Primary proof surface: `adl-provider-core/PROOF_INVENTORY.md and docs/runtime-v3/fixtures/issue855`
- Required artifacts present: `false: final hosted/CI proof pending`
- Artifact schema/version checks: `Native card structure validation plus OpenAPI contracts; final proof packet validation pending.`
- Hash/byte-stability checks: `Installed fixture manifests record binary SHA256 and invariant init SHA256/runtime incarnation; final binary run pending.`
- Missing/optional artifacts and rationale: `Required full hosted and current successful CI evidence remain absent and are not waived. Live02 failure report and separate diagnosis are preserved.`

## Decisions / Deviations
- `Lower-level provider core avoids ADL/kernel dependency cycle while preserving original adapter clients and ADL facades.`
- `Provider capabilities are conservative; explicit operator retries recover dynamic failed inference; healthy metadata checks never issue completion calls.`

## Follow-ups / Deferred work
- `Complete reviewed CI repair and continuation remediation; reconcile explicit retry authorization before any further hosted calls.`
- `Finalize independent review, native review/publication and required CI; root coordinates any merge.`
