# v0922-four-perspective-review

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

Task ID: issue-0890
Run ID: issue-0890
Version: 0.92.2
Title: [v0.92.2][CF-REVIEW] Execute isolated four-perspective repository review
Branch: codex/890-v0922-four-perspective-review
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:09:56.378553+00:00

Execution:
- Actor: `codex:/root`
- Model: `gpt-5`
- Provider: `OpenAI`
- Start Time: `2026-09-15T00:00:00-07:00`
- End Time: `in_progress`

## Summary

Implemented the bounded #890 CodeFriend four-perspective review runner and CLI dispatch in the bound FastWork worktree. After origin/main advanced, merged current main into the issue worktree and reran focused validation successfully. Fresh review found a prompt/parser confidence-contract mismatch, stale SOR fields, and invalid-confidence fail-closed persistence gap; the prompt now requires the typed confidence object form, invalid typed confidence is retained as a failed lane/run artifact, SOR truth was repaired, and strict focused proof passed. Fresh exact-head review, PR publication, CI, merge and terminal finish remain pending.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `no lane change; planned runtime PVF lane remained the final runtime lane`

## Issue Metrics Truth
- Expected runtime class: `bounded local Rust/CLI runtime validation plus separately authorized registered-provider proof`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `no operator token budget assigned`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `invalid_confidence_fail_closed_repaired_validation_passed_review_pending`
- Issue goal ref: `Codex goal created for #890 in thread 019ff3cb-e462-7343-be60-e4ab2b6080e3: Sprint 4 #890 CF-REVIEW execution—implement and prove the bounded four-perspective CodeFriend repository review runner in the bound FastWork worktree, preserve #891/#892+ sibling boundaries, run focused proof, obtain independent exact-head review, and reach publication-ready state without writing tracked issue work on main.`
- Sprint goal ref: `v0.92.2 execution Sprint 4; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/890/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/890/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No measured execution or estimate pair exists`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/890/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/review/mod.rs; adl/src/codefriend/review/lanes.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/src/cli/codefriend_cmd.rs; adl/src/cli/usage.rs; adl/tests/codefriend_review.rs`
- Additional proof artifacts: `Ignored proof artifacts under adl/target/codefriend-890-openai-proof: vector-scope.json, openai-provider-request.json, store, and run-openai. Admitted packet 6baa7ec10f500c88d0f6aa2cc2db38316a5dadb22e6abf6e5e983b76c7b5426f; admission digest e79b8853bd1d5896f9f93b070b46cd5855731449ab6fde72e3ac66a61c79de4c; run digest d8d007707a21cc84325dcb5c6390675a50bb612b24b3eda5b5a0fb0d33e53d0d.`

## Actions taken
- `Source issue reviewed for native preparation`
- `Six-card values prepared through current native template fields`
- `Implemented the four-perspective CodeFriend review runner, executed deterministic local validation, executed registered OpenAI proof, merged current origin/main, reran focused validation, repaired the confidence prompt contract, and added invalid typed-confidence fail-closed persistence proof`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; primary main remains tracked clean and issue work is only in /Volumes/FastWork/adl-worktrees/adl-issue-890-v0922-four-perspective-review`
- Worktree-only paths remaining: `all #890 implementation/card changes remain in the bound issue worktree pending commit, review and publication`
- Integration state: `worktree_only`
- Verification scope: `bound issue worktree plus ignored registered-provider proof artifacts under adl/target/codefriend-890-openai-proof`
- Integration method used: `bound issue worktree local implementation`
- Verification performed:
  - `cargo fmt --manifest-path adl/Cargo.toml --check; cargo test --manifest-path adl/Cargo.toml --test codefriend_review --test codefriend_evidence --test codefriend_ingestion; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_review -- -D warnings; git diff --check`
    `Confirmed the merged head still compiles, keeps four-lane review-runner tests green, preserves evidence/ingestion compatibility, passes clippy warnings-as-errors for the review test target, and has clean diff hygiene.`
- Result: `local deterministic proof, registered OpenAI proof, and post-merge focused validation passed; not published`

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
  - `cargo fmt --manifest-path adl/Cargo.toml --check; cargo test --manifest-path adl/Cargo.toml --test codefriend_review --test codefriend_evidence --test codefriend_ingestion; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_review -- -D warnings; git diff --check; OPENAI_API_KEY=<operator-approved env from /Users/daniel/keys/openai2.key> ADL_OBSERVABILITY_OTEL=0 ./target/debug/adl codefriend review run --store adl/target/codefriend-890-openai-proof/store --packet-id 6baa7ec10f500c88d0f6aa2cc2db38316a5dadb22e6abf6e5e983b76c7b5426f --provider-request adl/target/codefriend-890-openai-proof/openai-provider-request.json --out adl/target/codefriend-890-openai-proof/run-openai --run-id issue-890-openai-vector-410da89a`
    `Covers installed CLI review run, four isolated provider-adapter calls, committed lane input/result artifacts, peer-input rejection, missing-evidence fail-closed behavior, invalid typed-confidence fail-closed persistence, durable evidence-store compatibility, no source mutation in deterministic local fixtures, and actual registered OpenAI execution against the pinned Vector dnsmsg-parser scope.`
- Results:
  - `passed locally in the bound #890 worktree before merge: codefriend_review 3/3, codefriend_evidence 11/11, codefriend_ingestion 10/10; fmt --check passed; clippy -D warnings for codefriend_review passed; git diff --check passed; registered OpenAI proof completed four lanes with provider_status ok and finding_count 0. After merging current origin/main, reran and passed: cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check HEAD~1..HEAD; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_review -- -D warnings; cargo test --manifest-path adl/Cargo.toml --test codefriend_review --test codefriend_evidence --test codefriend_ingestion (codefriend_review 3/3, codefriend_evidence 11/11, codefriend_ingestion 10/10). After invalid-confidence fail-closed repair, reran and passed: cargo fmt --manifest-path adl/Cargo.toml --check; cargo test --manifest-path adl/Cargo.toml --test codefriend_review (4/4); cargo test --manifest-path adl/Cargo.toml --test codefriend_review --test codefriend_evidence --test codefriend_ingestion (25/25); cargo clippy --manifest-path adl/Cargo.toml --test codefriend_review -- -D warnings; git diff --check.`

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
      - "post-invalid-confidence-repair cargo test --manifest-path adl/Cargo.toml --test codefriend_review --test codefriend_evidence --test codefriend_ingestion passed 25/25"
  determinism:
    status: passed for deterministic local controlled-provider tests
    replay_verified: true_for_local_controlled_provider
    ordering_guarantees_verified: true
  security_privacy:
    status: passed for no raw credential retention in persisted proof artifacts inspected
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false_for_tracked_outputs; required worktree identity is intentionally absolute and ignored proof artifact paths identify local proof storage
  artifacts:
    status: present for local proof and registered OpenAI proof; PR/CI/merge/terminal artifacts pending
    required_artifacts_present: true_for_prepublication_local_and_provider_proof_review_pending
    schema_changes:
      present: not_run
      approved: not_run
```

## Determinism Evidence
- Determinism tests executed: `codefriend_review controlled-provider success and negative tests passed`
- Fixtures or scripts used: `adl/tests/codefriend_review.rs plus ignored OpenAI proof packet/run under adl/target/codefriend-890-openai-proof`
- Replay verification (same inputs -> same artifacts/order): `true_for_local_controlled_provider`
- Ordering guarantees (sorting / tie-break rules used): `runner persists each lane input/result before aggregate review record and rejects peer-output/preloaded provider input`
- Artifact stability notes: `Per-lane input/result/provider-result artifacts and aggregate run/review records are deterministic for the controlled provider tests; registered OpenAI output is retained as ignored proof evidence with exact run digest.`

## Security / Privacy Checks
- Secret leakage scan performed: `local proof asserts persisted run/log/stdout artifacts do not retain the fixture credential; actual OpenAI key was mapped from the operator-approved local key file into command environment only and was not printed, copied, committed or persisted.`
- Prompt / tool argument redaction verified: `true`
- Absolute path leakage check: `tracked SOR uses repository-relative proof paths where possible; absolute worktree path appears only in required issue worktree identity fields`
- Sandbox / policy invariants preserved: `source checkout remained read-only during evidence admission/review; no runtime source mutation or publication authority is granted to repository contents`

## Replay Artifacts
- Trace bundle path(s): `adl/target/codefriend-890-openai-proof/run-openai/review-record.json; adl/target/codefriend-890-openai-proof/run-openai/run.json`
- Run artifact root: `.csdlc/evidence/890 (planned)`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_review`
- Replay result: `passed 3/3 after current-main merge`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/890 (planned)`
- Required artifacts present: `true for deterministic local proof, registered OpenAI proof, and post-merge focused validation; independent exact-head review, PR, CI, merge and terminal receipts remain pending`
- Artifact schema/version checks: `passed through cargo test --manifest-path adl/Cargo.toml --test codefriend_review --test codefriend_evidence --test codefriend_ingestion, native csdlc validate for gen10 before final proof recording, and cargo clippy warnings-as-errors for codefriend_review`
- Hash/byte-stability checks: `registered OpenAI proof run digest d8d007707a21cc84325dcb5c6390675a50bb612b24b3eda5b5a0fb0d33e53d0d retained for proof identity; final tracked lifecycle digest will be validated after this proof recording`
- Missing/optional artifacts and rationale: `PR, CI, merge and terminal receipts do not exist yet because publication is pending fresh exact-head review.`

## Decisions / Deviations
- `Dependencies #881 and #855 were accepted/merged before #890 implementation. After origin/main advanced, current main was merged into the bound #890 worktree and focused validation was rerun before fresh review.`
- `Registered OpenAI proof artifacts remain ignored under adl/target/codefriend-890-openai-proof to avoid committing provider output or credential-adjacent runtime logs; SOR records their packet/admission/run digests and publication remains pending fresh exact-head review.`

## Follow-ups / Deferred work
- `Validate final lifecycle digest, commit immutable head, then obtain fresh independent exact-head review`
- `Publish through native typed route if available after PASS, otherwise use only authorized audited PR-create transport and leave merge/finish to typed authority.`
