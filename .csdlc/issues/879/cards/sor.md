# 879-github-ingestion

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

Task ID: issue-0879
Run ID: issue-0879
Version: v0.92.2
Title: [v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet
Branch: codex/879-github-ingestion
Card Status: ready
Status: MERGE_RESOLUTION_APPROVED
Generated: 2026-09-12T05:03:33.504300+00:00

Execution:
- Actor: `Codex subagent execute_877`
- Model: `not_collected`
- Provider: `not_collected`
- Start Time: `not_collected`
- End Time: `not_collected; publication and CI remain pending`

## Summary

Delivered production GET-only GitHub Git-data acquisition for pinned commits and PR heads, including fork repository provenance, exact local packet parity, inherited path/redaction/object guards, aggregate transport bounds and sanitized failures. Eight GitHub tests, ten inherited local tests and eight installed-candidate tests passed; focused clippy and formatting passed. Independent source review approved edc4d0a7df7b3c55c20f65d6a08c9a53affaa502. Record-only publication-card correction awaits acknowledgement; native publication and required CI remain pending.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `No lane change; runtime acquisition behavior with installed-command HTTP proof.`

## Issue Metrics Truth
- Expected runtime class: `bounded local CPU/disk/loopback integration; full CI pending`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `No explicit token or elapsed budget supplied for this issue goal.`
- Goal metrics data source: `not_collected; no issue-scoped metrics summary artifact available`
- Goal metrics source ref: `not_collected`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `reviewed_resolution_push_ci_pending`
- Issue goal ref: `Active #879 execution goal: reviewed passing PR; no merge or cleanup.`
- Sprint goal ref: `Sprint 2 umbrella #928`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/879/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: `No comparable measured estimate/actual pairs were collected; unknown metrics are not zero variance.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/879/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/ingestion/github.rs; adl/src/codefriend/ingestion/mod.rs; adl/src/cli/codefriend_github_cmd.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_github_ingestion.rs; docs/codefriend/GITHUB_INGESTION.md`
- Additional proof artifacts: `.csdlc/evidence/879/PROOF.json; .csdlc/evidence/879/REVIEW.md`

## Actions taken
- `Verified merged #878 predecessor at the bound baseline, created the issue goal, and implemented in codex/879-github-ingestion; primary main remained clean.`
- `Executed actual controlled Git-data HTTP acquisition and packet/receipt readers, commit/PR/fork local parity, seventeen transport failures, secret/path/symlink/partial/aggregate-budget cases, and installed CLI/logging proof.`
- `Recorded early metadata-budget finding and its fix; independent source review approved edc4d0a7df. Corrected publication-card P2 through native semantic edits; record-only acknowledgement pending.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; issue implementation is committed only on its bound branch`
- Worktree-only paths remaining: `All issue implementation and proof paths listed above await PR integration.`
- Integration state: `worktree_only`
- Verification scope: `Bound issue branch; local candidate and stable installed candidate on macOS.`
- Integration method used: `Native PR publication pending; no merge or manual transfer performed.`
- Verification performed:
  - `git status --short --branch; git rev-parse HEAD; git merge-base --is-ancestor 3fa601a888a8ba90541fd2d092802a9291b78ae9 HEAD`
    `Confirmed clean bound branch and accepted prerequisite ancestry; no claim of merge or primary-tree integration.`
- Result: `PR955 merge-resolution source5d15ea31f2 independently approved. GitHub and evidence production route tests pass; fmt/scoped diff checks pass. Final metadata acknowledgement, push/native reconciliation and new CI pending. Prior CI34674795360 belongs only to old1cdfhead.`

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
  - `cargo test --manifest-path adl/Cargo.toml --test codefriend_github_ingestion --test codefriend_ingestion; final GitHub target rerun; installed .adl/bin/adl repeat via ADL_CODEFRIEND_TEST_BINARY; cargo clippy --manifest-path adl/Cargo.toml --lib --bin adl --test codefriend_github_ingestion -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml --check; native csdlc validate.`
    `Proved actual HTTP and installed command/reader execution, packet and evidence identity parity, bounded and sanitized failures, compatibility logging, existing local contract preservation and native six-card structural validity.`
- Results:
  - `Conflict-focused GitHub installed-command transport/readback test passed (1); evidence local-admit/restart/read/delete CLI test passed (1); fmt and scoped diff check passed. Original broad proofs retained; no broad tests rerun for this additive dispatch resolution.`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: passed_local_ci_pending
    checks_run:
      - "8 GitHub +10 inherited tests;8 installed repeat; focused clippy/fmt/native cards"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: passed_bounded_fixture_checks
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: present_locally
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: true
```

## Determinism Evidence
- Determinism tests executed: `Exact commit/PR/fork packet equality with local ingestion, omitted-object parity, repeated candidate and installed fixture runs.`
- Fixtures or scripts used: `adl/tests/codefriend_github_ingestion.rs creates actual isolated Git objects and controlled loopback HTTP; inherited adl/tests/codefriend_ingestion.rs.`
- Replay verification (same inputs -> same artifacts/order): `Same revision/scope/content yields identical full Packet including object and packet identities across local, GitHub commit and PR paths.`
- Ordering guarantees (sorting / tie-break rules used): `Shared sorted unique Scope paths and deterministic packet/object serialization; no timestamps or host paths in packet identity.`
- Artifact stability notes: `Transport/PR provenance remains separate from unchanged repository_packet.v1 identity. Source proof is pinned in PROOF.json; record-only card corrections do not change source bytes.`

## Security / Privacy Checks
- Secret leakage scan performed: `Fixture source redaction, URL/userinfo rejection, token sentinel assertions on request headers/stdout/stderr/compatibility logs and omitted content passed.`
- Prompt / tool argument redaction verified: `Hostile source instructions remain inert; no provider/tool calls; transport errors use static sanitized codes.`
- Absolute path leakage check: `Shared portable packet and separate receipt omit checkout/output/cache paths; tests check error/log privacy and full packet parity.`
- Sandbox / policy invariants preserved: `Read-only GET transport; no repository code execution, GitHub mutation, paid provider use, main edits, merge or cleanup.`

## Replay Artifacts
- Trace bundle path(s): `No retained raw traces; deterministic tests recreate their isolated Git/HTTP fixtures. Safe summary retained in .csdlc/evidence/879/PROOF.json.`
- Run artifact root: `.csdlc/evidence/879`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --test codefriend_github_ingestion; set ADL_CODEFRIEND_TEST_BINARY to the stable installed candidate for installed CLI repeat.`
- Replay result: `8 tests passed in both candidate and installed-candidate runs.`

## Artifact Verification
- Primary proof surface: `adl/tests/codefriend_github_ingestion.rs and .csdlc/evidence/879/PROOF.json`
- Required artifacts present: `true; source, tests, docs, native cards and safe proof/review records are committed on the issue branch.`
- Artifact schema/version checks: `Unchanged codefriend.repository_packet.v1 consumed by AdmissionInput; new codefriend.github_acquisition.v1 receipt association validated by Acquisition::read. Schema-bearing source included in approved source review.`
- Hash/byte-stability checks: `Full local/GitHub Packet equality; actual Git blob SHA-1 verification; inherited SHA-1/SHA-256 reader tests; installed binary hash recorded in PROOF.json.`
- Missing/optional artifacts and rationale: `Live GitHub corroboration is optional and not run; no Vector download or paid call. Linux/full integration belongs to pending required CI.`

## Decisions / Deviations
- `Use existing shared GitHub credential resolver only for fixed production API host; controlled literal-loopback fixture mode never resolves or forwards credentials.`
- `Preserved R1 cumulative metadata finding and R2 rendered publication-card P2 with dispositions. Corrected native semantic fields rather than hand-editing Markdown or weakening source proof.`

## Follow-ups / Deferred work
- `Final metadata-only acknowledgement, then push reviewed merge and native publication reconciliation.`
- `Verify new-head checks start; user owns PR merge.`
