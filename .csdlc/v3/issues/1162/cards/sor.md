# codefriend-result-integrity

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

Task ID: issue-1162
Run ID: issue-1162
Version: 0.92.2
Title: [v0.92.2][TAIL-06][P1] Repair CodeFriend result integrity and website interoperability
Branch: codex/1162-codefriend-result-integrity
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-23T17:29:32.826544+00:00

Execution:
- Actor: `codex`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `not_started`
- End Time: `not_started`

## Summary

Implemented all seven #1162 Group B repairs across the ADL producer and CodeFriend website consumer. The website component passed independent exact-head review and is published as draft PR #20. The ADL component passed 65 focused tests after resolving PDF semantic-integrity and concurrent-retry review findings. Final ADL exact-head review, native proof, draft publication, and CI remain pending. No merge or deployment is claimed.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `No lane change; tests remain deterministic local tooling and contract proof.`

## Issue Metrics Truth
- Expected runtime class: `not_started`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `in_progress`
- Actual CI wait seconds: `not_started`
- Budget source: `No token budget requested; no paid operations authorized by this plan`
- Goal metrics data source: `exact test counts and Git revisions; timing and token metrics were not collected`
- Goal metrics source ref: `local command results and exact Git heads`
- Data-source confidence: `low`
- Estimate error percent: `unknown`
- Completion state: `implementation_and_local_validation_complete_final_review_pending`
- Issue goal ref: `issue-1162`
- Sprint goal ref: `issue-921`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/1162/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `Elapsed and token metrics were not collected; no precise variance claim is made.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/1162/cards/sor.md`
- Tracked implementation artifacts: `ADL commit d357c7fcbf5bf0da5cf1ae67587cc95fd038c237 and CodeFriend website commit 8f7d28e7f6254a95721bfa8b15cd95770382607b; exact finding map at .csdlc/evidence/1162/FINDING_DISPOSITIONS.md.`
- Additional proof artifacts: `none`

## Actions taken
- `Preserved multiline, CRLF and tab-bearing excerpts across Markdown, HTML and PDF, and strengthened PDF verification against semantic substitution and active or external content.`
- `Serialized cancel, retry and settlement state so incomplete attempts cannot be retried and concurrent retry requests cannot double-dispatch.`
- `Added native v4 website compatibility, provenance enforcement, post-await authorization, immutable workflow actions, actual native-emitted fixtures, and accurate source-built proof wording.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; all ADL changes remain on the bound issue branch and all website changes remain on its matching issue branch`
- Worktree-only paths remaining: `ADL issue branch and generated lifecycle records await draft publication; CodeFriend website component is published only as draft PR #20.`
- Integration state: `worktree_and_draft_component_pr`
- Verification scope: `none; preparation only`
- Integration method used: `bounded implementation in two issue worktrees; website draft PR #20; ADL draft PR pending`
- Verification performed:
  - `git status --short --branch; git rev-parse HEAD in both bound worktrees; focused Rust and Node test suites`
    `Confirmed clean exact component heads, retained native-v4 fixtures, and passing focused behavior across producer and consumer.`
- Result: `Local implementation and validation passed; website independent review passed; no merge or deployment performed.`

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
  - `cargo test --locked --manifest-path adl/Cargo.toml --lib publication::pdf::tests; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_agent_publication; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_html; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_review; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_review_assessments; npm test in the website worktree`
    `Exercises exact text semantics, hostile PDF resealing and external content rejection, attempt-state concurrency, native-v4 producer compatibility, provenance enforcement, asynchronous authorization, and workflow policy.`
- Results:
  - `passed: 65 ADL focused tests and 160 CodeFriend website tests; website independent focused rereview also passed 31/31 and isolated assessment tests passed 13/13`

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
      - "Final ADL exact-head independent review and CI remain required before merge readiness."
  determinism:
    status: passed_local
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: passed_local
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: passed_local
    required_artifacts_present: true_for_prepublication
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `yes; local deterministic fixtures, isolated repositories, controlled provider transport and native-emitted assessment fixtures`
- Fixtures or scripts used: `focused CodeFriend Rust fixtures plus website test/fixtures/native-v4 emitted from the actual ADL producer`
- Replay verification (same inputs -> same artifacts/order): `All named local suites completed with nonzero denominators.`
- Ordering guarantees (sorting / tie-break rules used): `operator state is locked across cancellation, retry reservation, dispatch and final settlement; the next attempt is durable before dispatch`
- Artifact stability notes: `website provenance binds source revisions to exact result digests; PDF semantic markers preserve logical lines while allowing visual wrapping`

## Security / Privacy Checks
- Secret leakage scan performed: `fixture-only validation; no live credentials or provider calls`
- Prompt / tool argument redaction verified: `no live prompts, credentials or provider tool arguments used`
- Absolute path leakage check: `passed for tracked artifacts and tests`
- Sandbox / policy invariants preserved: `yes; no merge, deployment, hosted-provider call or shared owner-binary replacement`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/1162/FINDING_DISPOSITIONS.md and native-emitted website fixtures`
- Run artifact root: `.csdlc/evidence/1162`
- Replay command used for verification: `Run the named focused Cargo targets from the ADL worktree and npm test from the website worktree.`
- Replay result: `passed`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/1162/FINDING_DISPOSITIONS.md`
- Required artifacts present: `yes for implementation and local proof; ADL exact-head review, native publication, and CI pending`
- Artifact schema/version checks: `native result schemas and website consumer validations passed through focused tests`
- Hash/byte-stability checks: `native fixture source revision and source-object digests are enforced; PDF report, manifest, export and stage digests are verified alongside reconstructed semantics`
- Missing/optional artifacts and rationale: `ADL PR, CI, merge, deployment, terminal receipts and cleanup do not exist at this prepublication stage.`

## Decisions / Deviations
- `Retain exactly seven assigned findings in one aggregate issue with two repository components.`
- `Preserve frozen review artifacts; reconcile current source before implementation.`

## Follow-ups / Deferred work
- `Obtain independent exact-head ADL review and admit the native review receipt.`
- `Publish the ADL draft PR, shepherd both PRs through CI, and stop before merge or deployment.`
