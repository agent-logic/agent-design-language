# 880-ci-ingestion

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

Task ID: issue-0880
Run ID: issue-0880
Version: v0.92.2
Title: [v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet
Branch: codex/880-ci-ingestion
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-09-12T05:07:36.211905+00:00

Execution:
- Actor: `fix_941_ci`
- Model: `GPT-6`
- Provider: `OpenAI`
- Start Time: `not_collected`
- End Time: `not_finished`

## Summary

Merged origin/main 9b5aae0ca0f8d53d67850238b25b955af1b4ccd9 into existing issue branch at 2adcc3352d9caa29037a5762cb2172fbd426db08. Sole manual conflict retains both CI and GitHub ingestion modules. Root independent exact-source review passed; focused CI ingestion and CI/UTS/workflow contracts passed. Updated-head hosted CI and native publication reconciliation remain pending.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `No lane change; focused local acquisition plus required hosted installed smoke`

## Issue Metrics Truth
- Expected runtime class: `focused integration`
- Estimated elapsed seconds: `not_collected`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `not_collected`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `not_collected`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `not_collected`
- Actual CI wait seconds: `not_collected`
- Budget source: `No explicit budget`
- Goal metrics data source: `not_collected`
- Goal metrics source ref: `Issue goal service`
- Data-source confidence: `unknown`
- Estimate error percent: `not_collected`
- Completion state: `current_head_ci_pending`
- Issue goal ref: `Active issue880 passing reviewed PR goal`
- Sprint goal ref: `Sprint2 #928`
- Goal metrics rollup ref: `Parent sprint goal; not exported`
- Validation planning prompt: `.csdlc/issues/880/cards/vpp.md`
- Missing-telemetry rule: record `unknown` or `not_collected`; do not invent precision from chat memory or broad timestamp guesses.
- Goal-metrics substrate note: consume the `#4264` issue-goal metrics summary when available and record `unknown` instead of duplicating raw session logs here.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `unknown`
- Variance analysis completed: `false`
- Variance category: `not_applicable`
- Variance note: `No measured estimate pair`
- Sprint rollup guidance: count only completed variance analyses by `Variance category`; keep `not_applicable` out of category totals and never treat unknown metrics as zero variance.

## Artifacts produced
- Local ignored output-card scaffold at `.csdlc/issues/880/cards/sor.md`
- Tracked implementation artifacts: `adl/src/codefriend/ingestion/ci.rs; adl/src/cli/codefriend_ci_cmd.rs; narrow module registrations; adl/tests/codefriend_ci_ingestion.rs; adl/tools/codefriend; adl/tools/ci_path_policy.sh; .github/workflows/ci.yaml; docs/codefriend-ci-ingestion.md`
- Additional proof artifacts: `.csdlc/evidence/880/IMPLEMENTATION_PROOF.md`

## Actions taken
- `Reused local immutable acquisition and production reader with a separate validated CI receipt.`
- `Added explicit CI CLI inputs and bounded allowlisted run metadata; category-only failures.`
- `Added required installed CI smoke, upload proof and aggregate/path selection contracts.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `None; primary main remains inspection-only.`
- Worktree-only paths remaining: `All issue880 changes until reviewed publication.`
- Integration state: `pr_open`
- Verification scope: `Bound local Git fixtures and actual CLI; hosted execution pending`
- Integration method used: `Existing PR #957 branch refreshed by local merge; no PR merge`
- Verification performed:
  - `git status --short --branch; git rev-parse HEAD`
    `Confirmed bound issue880 on accepted merged predecessor baseline`
- Result: `Implementation independently approved; PR publication and hosted CI pending.`

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
  - `cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_ci_ingestion; python3 adl/tools/codefriend/ci_smoke.py --binary .adl/bin/adl --candidate-revision EXACT_SHA --output NEW_DIRECTORY; ruby adl/tools/codefriend/test_ci_contract.rb; ruby adl-uts/tools/test_ci_contract.rb; ruby adl/tools/validate_ci_workflow_policy.rb`
    `Ten local production-route cases, receipt tamper guards, actual aggregate outcomes and installed path proof`
- Results:
  - `Source merge: codefriend_ci_ingestion 1/1 conformance test passes (ten production CLI cases); CI contract 12 aggregate outcomes and five path selections pass; UTS six outcomes pass; workflow policy passes; resolution-only diff hygiene passes. Incoming main evidence blank EOF warnings were not rewritten. Run 34675098052 attempt 2 passed on old cf25e273a39499c3d1b699fa7d9e461266024e0c only; not current-head proof.`

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
      - ".csdlc/evidence/880/IMPLEMENTATION_PROOF.md"
  determinism:
    status: passed
    replay_verified: false
    ordering_guarantees_verified: true
  security_privacy:
    status: passed
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
- Determinism tests executed: `Common packet exact local parity and relocated checkout parity through production CLI`
- Fixtures or scripts used: `adl/tools/codefriend/ci_smoke.py; test_ci_contract.rb`
- Replay verification (same inputs -> same artifacts/order): `No replay algorithm claim; production packet reader validates acquisition`
- Ordering guarantees (sorting / tie-break rules used): `Metadata validates before capture/write; packet readback precedes receipt publication; upload success precedes delivery evidence`
- Artifact stability notes: `Run metadata excluded from common packet; receipt bound to packet and revision`

## Security / Privacy Checks
- Secret leakage scan performed: `Synthetic credential metadata rejected before output; common source safety reused`
- Prompt / tool argument redaction verified: `No provider calls; category-only acquisition errors tested`
- Absolute path leakage check: `Fixture checkout absolute path absent from packet and receipt; logs local only`
- Sandbox / policy invariants preserved: `Bound FastWork worktree; no paid calls, main writes or deployment`

## Replay Artifacts
- Trace bundle path(s): `not_applicable`
- Run artifact root: `.adl/880-proof`
- Replay command used for verification: `not_applicable`
- Replay result: `not_claimed`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/880/IMPLEMENTATION_PROOF.md`
- Required artifacts present: `Implementation/tests/docs/cards exist; hosted proof pending`
- Artifact schema/version checks: `Packet reused from878; separate CI receipt validates packet/revision and rejects delivery claims`
- Hash/byte-stability checks: `Exact local/CI packet equality and relocation equality pass`
- Missing/optional artifacts and rationale: `No model/review algorithm, broad connector or fitness gate in scope`

## Decisions / Deviations
- `Independent additive CLI dispatch avoids sibling879 registration dependency`
- `Upload is separate transport proof; acquisition receipts explicitly not_established`

## Follow-ups / Deferred work
- `Final metadata delta review, native current-head review/publication reconciliation and hosted CI; no automatic merge.`
- `No merge or closeout in this task`
