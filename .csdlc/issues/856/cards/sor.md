# issue-856-native-release-preflight

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

Task ID: issue-0856
Run ID: issue-0856
Version: v0.92.1
Title: [v0.92.1][release] Reconcile release versions and restore native v3 ceremony preflight
Branch: codex/856-native-release-preflight
Card Status: ready
Status: in_progress
Generated: 2026-09-11T18:46:11.480766+00:00

Execution:
- Actor: `Planning #7`
- Model: `GPT-6`
- Provider: `OpenAI`
- Start Time: `unknown`
- End Time: `unknown`

## Summary

Implementation complete and initial 241 native tests passed; 23 package metadata checks passed; review lock-owner fix added; final proof/review and CI pending.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `unchanged`

## Issue Metrics Truth
- Expected runtime class: `small`
- Estimated elapsed seconds: `unknown`
- Actual elapsed seconds: `unknown`
- Actual active work seconds: `unknown`
- Estimated total tokens: `unknown`
- Actual total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Actual validation seconds: `unknown`
- Actual PR wait seconds: `unknown`
- Actual CI wait seconds: `unknown`
- Budget source: `issue goal; no explicit token budget`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `in_progress`
- Issue goal ref: `Planning #7 issue856 goal`
- Sprint goal ref: `not_applicable`
- Goal metrics rollup ref: `not_collected`
- Validation planning prompt: `.csdlc/issues/856/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `false`
- Variance category: `not_collected`
- Variance note: `No fabricated token/time measurements.`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/856/cards/sor.md`
- Tracked implementation artifacts: `Release package manifests/active locks; csdlc-v3/src/commands/release.rs and CLI; release wrapper/owner installer; tests; release inventory and docs.`
- Additional proof artifacts: `.csdlc/evidence/856`

## Actions taken
- `Reconciled coordinated release family versions and active lock identities; retained historical helper locks.`
- `Implemented native read-only preflight, removed unsafe fallback/bypasses, and added stable native owner install support.`
- `Ran native, metadata and shell proofs; fixed independent review lock owner finding.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `Issue856 implementation and cards`
- Integration state: `worktree_only`
- Verification scope: `Issue856 bound worktree and isolated local candidate fixtures`
- Integration method used: `bound issue branch; PR publication pending`
- Verification performed:
  - `git status --short --branch; git diff --check`
    `Verified worktree isolation and whitespace`
- Result: `not merged`

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
  - `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets; cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; bash adl/tools/test_release_ceremony.sh; bash adl/tools/test_owner_binary_install.sh`
    `Exercise exact identities, fail-closed guards, version/lock coverage, no mutation and stdout/stderr separation`
- Results:
  - `241 initial native tests PASS; final expanded candidate matrix PASS;23 metadata checks and shell/installer suites PASS; CI pending`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: local_passed_ci_pending
    checks_run:
      - "Real CLI candidate matrix with positive and negative cases"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: reviewed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: present
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: false
```

## Determinism Evidence
- Determinism tests executed: `csdlc-v3/tests/release_preflight.rs`
- Fixtures or scripts used: `csdlc-v3/tests/release_preflight.rs; adl/tools/test_release_ceremony.sh; adl/tools/test_owner_binary_install.sh`
- Replay verification (same inputs -> same artifacts/order): `Each request is nonmutating; changed identities rejected`
- Ordering guarantees (sorting / tie-break rules used): `Identity and authority admission precede eligibility; final checkout recheck`
- Artifact stability notes: `Candidate Git blobs and exact BLAKE3 bytes`

## Security / Privacy Checks
- Secret leakage scan performed: `No credentials used by preflight; inspected error/report fields`
- Prompt / tool argument redaction verified: `No request or gate bodies logged`
- Absolute path leakage check: `Tracked proof uses relative paths; generated binding records retain required worktree identity`
- Sandbox / policy invariants preserved: `Bound worktree writes only; no live release mutation`

## Replay Artifacts
- Trace bundle path(s): `.csdlc/evidence/856`
- Run artifact root: `.csdlc/evidence/856`
- Replay command used for verification: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test release_preflight`
- Replay result: `PASS including empty, omitted and source-tagged lock owner negatives`

## Artifact Verification
- Primary proof surface: `csdlc-v3/tests/release_preflight.rs`
- Required artifacts present: `true`
- Artifact schema/version checks: `Typed requests/gates/inventory reject unknown fields; native card validation`
- Hash/byte-stability checks: `BLAKE3 candidate notes/gate/inventory/evidence comparisons`
- Missing/optional artifacts and rationale: `No actual release gate or authorization manufactured`

## Decisions / Deviations
- `Preflight consistency does not authenticate local gate prose or grant release approval`
- `Replace combined mutation wrapper with preflight-only request; publication remains separate`

## Follow-ups / Deferred work
- `Renew exact-head review, publish PR and resolve required CI`
- `#526 consumes repair revision and exact command; release gates remain separate`
