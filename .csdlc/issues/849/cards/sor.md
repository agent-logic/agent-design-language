# v0922-merge-linkage-admission

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

Task ID: issue-0849
Run ID: issue-0849
Version: 0.92.2
Title: [v0.92.2][C-SDLC v3][P2] Preserve publication linkage during native PR merge
Branch: codex/849-v0922-merge-linkage-admission
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:22:05.416044+00:00

Execution:
- Actor: `Planning #7 / delegated sprint8_720; native issue #849 bound worktree`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `2026-09-12T01:48:57+00:00`
- End Time: `in_progress`

## Summary

Repair native merge admission with reviewed qualified publication linkage, authenticated body/closing relation and issue-state checks, and durable linkage reconciliation. Independent implementation review passed; full native suite remains blocked by pre-existing release inventory omission.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `No lane change; full native owner gate remains failed by baseline inventory omission`

## Issue Metrics Truth
- Expected runtime class: `Deterministic local Git/filesystem and fake authenticated transport, small CPU; no paid runtime`
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
- Completion state: `implementation_reviewed_validation_blocked`
- Issue goal ref: `Issue #849 session active: reviewed implementation and truthful PR publication through green CI; no merge authorization`
- Sprint goal ref: `v0.92.2 Sprint 7 umbrella #933`
- Goal metrics rollup ref: `.csdlc/evidence/849/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/849/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/849/cards/sor.md`
- Tracked implementation artifacts: `csdlc-v3/src/commands/remote/{mod,merge,merge_linkage}.rs; csdlc-v3/src/adapters/mod.rs; merge tests and receipt fixtures; operator documentation/man pages and #849 criterion map`
- Additional proof artifacts: `.csdlc/evidence/849/release-preflight-baseline-defect.json; docs/milestones/v0.92.2/evidence/MERGE_LINKAGE_849.md`

## Actions taken
- `Bound existing #849 gen2 preparation natively, created issue goal, and normalized bound planning truth under Sprint7 #933.`
- `Implemented qualified linkage receipt digest, constrained read-only query, pre-dispatch and postmerge mode/state checks, preserving no-second-PUT replay.`
- `Added negative and positive fake-transport proof; updated operator manual and current criterion correction map; independent sprint8_909 accepted exact implementation commit.`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none`
- Worktree-only paths remaining: `Issue #849 code, tests, docs, six cards and proof packet`
- Integration state: `worktree_only`
- Verification scope: `Bounded merge-linkage implementation and focused owner proofs`
- Integration method used: `local commit; publication pending`
- Verification performed:
  - `git status --short --branch; git rev-parse HEAD; gh pr view 948 --repo agent-logic/agent-design-language --json state,headRefOid,mergeCommit (read-only)`
    `Bound branch verified; PR not yet created; #948 OPEN at03c5aead, integration pending`
- Result: `Implementation committed locally; PR publication pending; no merge or terminal closeout`

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
  - `See .csdlc/evidence/849/VALIDATION.md for exact commands and outcomes`
    `Proves bounded local merge behavior and unaffected tested owner surfaces; excludes full-suite/CI/live-merge success`
- Results:
  - `16 merge tests pass; clippy/fmt/manual parity pass; full suite FAILED known baseline release-preflight omission; diagnostic remaining suite253passed1filtered`

Validation command/path rules:
- Prefer repository-relative paths in recorded commands and artifact references.
- Do not record absolute host paths in output records unless they are explicitly required and justified.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Do not list commands without describing their effect.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: failed_full_suite_known_baseline
    checks_run:
      - "Native six-card values/render/structure/digest validation passed gen7; exact current validation repeated before publication"
  determinism:
    status: passed_focused
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: bounded_adapter_tests_and_packet_inspection_passed
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: six_native_cards_and_local_proof_present
    required_artifacts_present: true
    schema_changes:
      present: true
      approved: Issue #849 requires bounded review/intent/linkage type change; no authority-generation change
```

## Determinism Evidence
- Determinism tests executed: `16 merge cases passed; reviewer independently passed16merge plus1adapter; full diagnostic owner253passed1filtered`
- Fixtures or scripts used: `remote/tests/merge_cases.rs and adapters/mod.rs fake transport/local Git fixtures; full native suite with known baseline failure recorded`
- Replay verification (same inputs -> same artifacts/order): `Successful and uncertain merge replay paths exercised; no automatic second PUT; linkage/issue-state drift rejected`
- Ordering guarantees (sorting / tie-break rules used): `Durable target guard and intent precede dispatch; authenticated fresh linkage/policy checked before PUT; poststate and replay tests passed`
- Artifact stability notes: `Implementation commit0264f321ac; native card values/render/schema validation passed; final metadata review renewed before publication`

## Security / Privacy Checks
- Secret leakage scan performed: `Reviewed tracked proof packet for credential values and raw bodies; none retained; ignored local logs are not public proof`
- Prompt / tool argument redaction verified: `Native adapter credential-isolation and machine-channel tests passed within253-test diagnostic suite; no secret values stored in issue evidence`
- Absolute path leakage check: `Proof packet uses repo-relative paths; generated SIP binding intentionally records exact approved FastWork identity`
- Sandbox / policy invariants preserved: `All issue edits in exact native-bound FastWork worktree; primary clean main; no live merge/cloud changes or shared binary install`

## Replay Artifacts
- Trace bundle path(s): `.adl/runs/849/native-tests-release-preflight-failed.log; .adl/runs/849/native-tests-excluding-known-failure.log; .adl/runs/849/clippy.log`
- Run artifact root: `.adl/runs/849`
- Replay command used for verification: `cargo test --manifest-path csdlc-v3/Cargo.toml --lib merge_cases`
- Replay result: `Positive replay does not dispatch a second PUT; uncertain/mismatched issue-state replay rejects`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/849/VALIDATION.md`
- Required artifacts present: `six native-generated cards, local proof packet and operator documentation`
- Artifact schema/version checks: `Native validate gen5 passed all six values/renders/structures/digest; manual inventory and30 rendered pages verified`
- Hash/byte-stability checks: `Review digest and durable intent/reconciliation identity tamper and replay tests passed; baseline release input blob equality recorded`
- Missing/optional artifacts and rationale: `No live merge or deployment proof is required or claimed. Hosted CI has not run. Final session timing/token metrics remain unknown until collection. Local test logs and tracked proof packet exist.`

## Decisions / Deviations
- `No recursive size-reduction accounting: issue adds behavior and tests and makes no size-reduction claim.`
- `Read-only GitHub query smoke proved schema on already-merged939/720; all merge dispatch tests remain fake transport.`

## Follow-ups / Deferred work
- `Route separate UTS release inventory omission introduced by #877; do not waive full owner suite`
- `Renew exact final-card review; native review/publication with Closes #849; monitor CI; preserve #948 source when accepted; no merge authorization`
