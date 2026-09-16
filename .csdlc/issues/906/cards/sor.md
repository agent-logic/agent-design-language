# v0922-process-parser-simplification

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

Task ID: issue-0906
Run ID: issue-0906
Version: 0.92.2
Title: [v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor
Branch: codex/906-v0922-process-parser-simplification
Card Status: draft
Status: IN_PROGRESS
Generated: 2026-09-12T00:22:05.416044+00:00

Execution:
- Actor: `/root`
- Model: `unknown`
- Provider: `unknown`
- Start Time: `2026-09-16; exact session start timestamp not recorded`
- End Time: `in_progress pending publication and hosted CI`

## Summary

Implemented the bounded process-status parser extraction and measurable target-selection simplification with focused local proof; review, PR publication, and hosted CI remain pending.

## PVF Lane Truth
- Initial PVF lane: `tooling`
- Planned PVF lane: `tooling`
- Final PVF lane: `tooling`
- Lane change reason: `not_applicable; tooling lane remained unchanged`

## Issue Metrics Truth
- Expected runtime class: `deterministic local tooling with controlled loopback and owned-process fixtures`
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
- Completion state: `review_complete_publication_pending`
- Issue goal ref: `Active issue #906 implementation, review, publication, and green CI goal`
- Sprint goal ref: `v0.92.2 execution Sprint 7; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/906/goal-metrics.json (planned; absent until execution)`
- Validation planning prompt: `.csdlc/issues/906/cards/vpp.md`
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
- Local ignored output-card scaffold at `.csdlc/issues/906/cards/sor.md`
- Tracked implementation artifacts: `adl/src/cli/process_cmd.rs; adl/src/cli/process_cmd/args.rs; adl/tests/cli_smoke/process_status.rs; .csdlc/evidence/906/process-parser-inventory.md`
- Additional proof artifacts: `.csdlc/evidence/906/process-parser-inventory.md`

## Actions taken
- `Preserved and isolated the historical overlapping worktree with exact patch identity`
- `Extracted the complete process-status parser into process_cmd/args.rs and replaced four target options plus count/selection chain with one typed target slot and explicit conflict tracking`
- `Added focused parser and installed CLI regressions and completed local validation`

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: `none; changes remain worktree-only before review/publication`
- Worktree-only paths remaining: `adl/src/cli/process_cmd.rs; adl/src/cli/process_cmd/args.rs; adl/tests/cli_smoke/process_status.rs; .csdlc/evidence/906/process-parser-inventory.md; issue-local lifecycle records`
- Integration state: `worktree_only`
- Verification scope: `Independent implementation review at 9a1578aca9bf52d269369b3745ea2cac4b64b8a8; native review-record generation and hosted CI pending`
- Integration method used: `worktree-only implementation pending PR`
- Verification performed:
  - `cargo test --manifest-path adl/Cargo.toml --bin adl-process; cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status; cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml -- --check; git diff --check`
    `Focused local parser and installed CLI proof passed; hosted integration remains pending`
- Result: `not_integrated`

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
  - `cargo test --manifest-path adl/Cargo.toml --bins process_cmd::args::tests; cargo test --manifest-path adl/Cargo.toml --bin adl-process; cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status; cargo clippy --manifest-path adl/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path adl/Cargo.toml -- --check; git diff --check`
    `Proves pure parser behavior and error precedence through 12 filtered binary-path executions, the complete 11-test standalone adl-process suite, 16 installed CLI process-status tests, strict all-target lint, formatting, and patch hygiene`
- Results:
  - `Local required checks and independent exact-head review passed; publication and hosted CI remain pending`

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
      - "12 parser binary-path executions, 11 standalone adl-process tests, and 16 installed CLI tests passed"
  determinism:
    status: passed
    replay_verified: true
    ordering_guarantees_verified: not_run
  security_privacy:
    status: passed_local
    secrets_leakage_detected: not_run
    prompt_or_tool_arg_leakage_detected: not_run
    absolute_path_leakage_detected: not_run
  artifacts:
    status: passed_local
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `true`
- Fixtures or scripts used: `Existing process_cmd unit surface, standalone adl-process binary unit surface, and cli_smoke process_status integration surface; no new fixture family`
- Replay verification (same inputs -> same artifacts/order): `passed; focused deterministic proof was repeated with locked dependencies`
- Ordering guarantees (sorting / tie-break rules used): `Repeated same-target last-value behavior and delayed conflict/error precedence covered in unit and installed CLI tests`
- Artifact stability notes: `Historical dirty process-status-fanout worktree preserved byte-for-byte; implementation isolated in the bound #906 FastWork worktree`

## Security / Privacy Checks
- Secret leakage scan performed: `not_applicable; no secret-bearing input or credential-handling surface changed`
- Prompt / tool argument redaction verified: `not_applicable; no provider credentials or sensitive tool arguments are used by this local parser proof`
- Absolute path leakage check: `reviewed; the evidence intentionally records the operator-local historical worktree path required for ownership preservation`
- Sandbox / policy invariants preserved: `true`

## Replay Artifacts
- Trace bundle path(s): `not_applicable; this deterministic local parser refactor produces no runtime trace bundle`
- Run artifact root: `.csdlc/evidence/906`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml --bins process_cmd::args::tests; cargo test --manifest-path adl/Cargo.toml --bin adl-process; cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status`
- Replay result: `passed: 12 parser binary-path executions, 11 standalone adl-process tests, and 16 installed CLI tests`

## Artifact Verification
- Primary proof surface: `.csdlc/evidence/906/process-parser-inventory.md and focused Rust test output`
- Required artifacts present: `true`
- Artifact schema/version checks: `not_applicable; no schema-bearing artifact changed`
- Hash/byte-stability checks: `Baseline and current source SHA-256 plus recursive line inventory recorded in .csdlc/evidence/906/process-parser-inventory.md`
- Missing/optional artifacts and rationale: `Independent exact-head review completed with no open findings; PR publication and hosted CI remain pending`

## Decisions / Deviations
- `#864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added.`
- `Ownership resolution 2026-09-16: historical worktree /Users/daniel/git/agent-design-language/.worktrees/adl-process-status-fanout on branch codex/reduce-process-status-fanout is operator-owned June 19 WIP at 1ea914010e6b96482a95bd6f64c8318f1a19b937. Its four-file dirty patch has SHA-256 aef64c67a22d74ff5dc4962d5c6f25ab09a06ca64bb1763c48984d73558fbf84; no origin or legacy-origin branch or PR exists. Preserve every byte there. Issue #906 will not copy, reset, cherry-pick, or modify that worktree and will execute only in its separate bound FastWork worktree from current origin/main. The source and CLI-test paths are clear through isolation; the unrelated finish files remain untouched.`

## Follow-ups / Deferred work
- `Publish the independently reviewed head through native C-SDLC v3`
- `Publish PR with Closes #906, settle required CI, then defer terminal finish and cleanup as instructed`
