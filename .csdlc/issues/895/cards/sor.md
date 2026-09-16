# v0922-publication-approval

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

Task ID: issue-0895
Run ID: issue-0895
Version: 0.92.2
Title: [v0.92.2][CF-UX] Enforce exact-artifact publication approval
Branch: codex/895-v0922-publication-approval
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:10:15.986006+00:00

Execution:
- Actor: `codex/root`
- Model: `GPT-5`
- Provider: `OpenAI`
- Start Time: `2026-09-16; exact start time not collected`
- End Time: `in_progress`

## Summary

Implemented #895 exact-artifact publication approval with destination/target binding, explicit decisions, rollback anchoring, stable verified snapshots, serialized atomic visibility, and collision-safe staging. Reserved staging targets are denied and success leaves exact approved bytes visible. Local proof passes; independent exact-head review and integration remain pending.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime; deterministic installed CLI and local filesystem admission`
- Lane change reason: `No lane change; proof uses isolated local CPU/filesystem fixtures with no provider, network, cloud, or remote publication effect.`

## Issue Metrics Truth
- Expected runtime class: `bounded_local`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `No operator issue token budget assigned; VPP estimates are planning only`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implementation_and_local_proof_complete_review_pending`
- Issue goal ref: `Active Sprint 4 #930 execution goal; #895 is the current bounded child objective`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/895/goal-metrics.json (planned, absent until execution)`
- Validation planning prompt: `.csdlc/issues/895/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No measured execution/estimate pair exists`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/895/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/publication/{mod.rs,manifest.rs,approval.rs}; adl/src/codefriend/evidence/contracts.rs; adl/src/codefriend/mod.rs; adl/src/cli/codefriend_publication_cmd.rs; adl/src/cli/codefriend_cmd.rs; adl/src/cli/mod.rs; adl/tests/codefriend_ux.rs; adl/tests/codefriend_evidence.rs; adl/tests/fixtures/codefriend/evidence/{publication-v1.json,publication-schema-v1.json}; adl/tests/fixtures/codefriend/publication/PVF.json; native issue cards and transaction receipt`
- Additional proof artifacts: `Staging-collision visibility unit 1/1; concurrent-growth unit 1/1; lock-held serialization unit 1/1; verified-snapshot unit 1/1; installed publication-control scenarios 6/6; evidence regressions 11/11; review regressions 14/14; rustfmt, strict all-target/all-feature Clippy, and diff hygiene passed.`

## Actions taken
- `Bound approval to the canonical destination and a validated relative target. Persisted publication validation and final admission both reject absolute/traversing targets, and admission enforces target containment.`
- `The locked canonical publication-control store uses a durable local head and a deterministic external head anchor whose canonical topology must not be within or contain the decision store.`
- `Admission stages exact verified bytes in a reserved internal namespace, skips any stage equal to the final target, and cleans staging only on failure. Proof covers staging-name collision visibility, reserved-target denial, growth, rollback, revocation, destination identity, and snapshot integrity.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; issue branch implementation is not merged`
- Worktree-only paths remaining: `all #895 implementation and lifecycle paths pending exact review and PR merge`
- Integration state: `worktree_only`
- Verification scope: `bound #895 worktree; candidate includes exact approval, validated relative non-reserved target, canonical destination binding, collision-safe atomic staging, canonical store and separate rollback anchor, stable bounded snapshots, revocation serialization, and exact lifecycle truth`
- Integration method used: `bound issue worktree; commit and PR pending`
- Verification performed:
  - `git status --short --branch; git diff --check`
    `Verified issue-only worktree ownership and patch hygiene; no main write.`
- Result: `Local implementation and proof complete; main integration not yet attempted.`

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
  - `cargo test --manifest-path adl/Cargo.toml admission_holds_the_store_lock_until_publication_is_visible --lib; cargo test --manifest-path adl/Cargo.toml atomic_publication_uses_the_verified_snapshot_not_a_second_source_read --lib; cargo test --manifest-path adl/Cargo.toml --test codefriend_ux; cargo test --manifest-path adl/Cargo.toml --test codefriend_evidence; cargo test --manifest-path adl/Cargo.toml --test codefriend_review; cargo fmt --manifest-path adl/Cargo.toml --check; cargo clippy --manifest-path adl/Cargo.toml --all-targets --all-features -- -D warnings; git diff --check`
    `Proves reserved staging targets are rejected and a same-name staging collision cannot delete successful publication; absolute targets, wrong destinations, nested anchors, concurrent growth, rollback, deleted tails, and revocation interleavings fail closed; exact verified bytes remain visible; evidence/review compatibility, formatting, warnings, and patch hygiene pass.`
- Results:
  - `passed: staging-collision visibility unit 1/1; concurrent-growth unit 1/1; lock-held serialization unit 1/1; verified-snapshot unit 1/1; codefriend_ux 6/6; codefriend_evidence 11/11; codefriend_review 14/14; rustfmt passed; strict Clippy passed; diff check passed`

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
      - "Independent exact-head review is pending and must review final committed product, test, PVF, and lifecycle bytes."
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
      present: true
      approved: pending independent review
```

## Determinism Evidence
- Determinism tests executed: `yes; isolated canonical approval stores, controlled timestamps, lock-held invalidate and withhold attempts before atomic visibility, alternate-store replay attempts, deleted revocation tails, verified byte snapshots, create-only decision records, and local atomic admission`
- Fixtures or scripts used: `adl/tests/codefriend_ux.rs; adl/tests/fixtures/codefriend/evidence/review-v1.json; adl/tests/fixtures/codefriend/publication/PVF.json`
- Replay verification (same inputs -> same artifacts/order): `The suite rejects absolute target approval/admission, wrong-root admission, reserved anchor-store topology, concurrent file growth, coherent decision-store rollback, alternate-store replay, deleted revocation tails, and revocation interleaving; exact verified bytes are published only to the bound contained destination.`
- Ordering guarantees (sorting / tie-break rules used): `one DecisionStore lock remains owned from authoritative head resolution through byte snapshot, staging, and atomic destination rename; revocation can linearize only before admission reads the head or after publication is visible`
- Artifact stability notes: `Decision records are digest-named and create-only. Publication identity binds a validated relative non-reserved target and canonical destination root. Successful atomic rename is not followed by staging cleanup; collision proof requires the target and exact bytes to remain visible. External-anchor, stable snapshot, path-based symlink, full-machine rollback, and fail-closed crash boundaries remain recorded without overclaim.`

## Security / Privacy Checks
- Secret leakage scan performed: `credential/path scanner exercised with negative fixtures before any publication write`
- Prompt / tool argument redaction verified: `machine JSON stays on stdout; provider/tool execution is outside #895 and not invoked`
- Absolute path leakage check: `absolute host paths are rejected from publishable textual artifacts; local execution paths are not serialized into product records`
- Sandbox / policy invariants preserved: `yes; local isolated filesystem only, no external publication or credentials`

## Replay Artifacts
- Trace bundle path(s): `adl/tests/fixtures/codefriend/publication/PVF.json and installed CLI test output`
- Run artifact root: `.csdlc/evidence/895 (native review and publication evidence pending)`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml atomic_publication_uses_the_verified_snapshot_not_a_second_source_read --lib; cargo test --manifest-path adl/Cargo.toml --test codefriend_ux`
- Replay result: `passed staging-collision visibility 1/1, concurrent-growth 1/1, serialization 1/1, snapshot-integrity 1/1, and installed integration scenarios 6/6`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_ux.rs and codefriend::publication::approval::tests::atomic_publication_uses_the_verified_snapshot_not_a_second_source_read`
- Required artifacts present: `yes for implementation and local prepublication proof; independent review, PR, CI, merge, and terminal receipts remain pending`
- Artifact schema/version checks: `passed through strict serde schemas and installed CLI parsing`
- Hash/byte-stability checks: `passed for review, finding set, manifest, store identity, decision records, authoritative head commitment, verified snapshot bytes, and admission receipts`
- Missing/optional artifacts and rationale: `Independent review, PR, CI, merge, and terminal evidence cannot exist before publication.`

## Decisions / Deviations
- `#891 and #881 are closed/accepted; #895 is bound and implementation is complete within its publication-control scope without #894 renderer ownership.`
- `Six exact-head reviews failed. The sixth found a deterministic staging-name alias that could delete a successfully published target. All actionable findings from every round are repaired locally without provider, network, cloud, renderer, or remote-publication scope.`

## Follow-ups / Deferred work
- `Obtain a distinct fresh exact-head review of the immutable collision-safe staging remediation candidate.`
- `After PASS and current-base reconciliation, publish with Closes #895, shepherd CI, merge, finish, and clean through native authority.`
