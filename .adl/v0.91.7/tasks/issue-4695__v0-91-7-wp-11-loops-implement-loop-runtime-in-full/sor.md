# v0-91-7-wp-11-loops-implement-loop-runtime-in-full

Canonical Template Source: `docs/templates/prompts/1.0.3/sor.md`

Execution Record Requirements:
- The output card is a machine-auditable execution record.
- All sections must be fully populated. Empty sections, placeholders, or implicit claims are not allowed.
- Every command listed must include both what was run and what it verified.
- If something is not applicable, include a one-line justification.

Task ID: issue-4695
Run ID: issue-4695
Version: v0.91.7
Title: [v0.91.7][WP-11][loops] Implement loop runtime in full
Branch: codex/4695-v0-91-7-wp-11-loops-implement-loop-runtime-in-full
Card Status: ready
Status: IN_PROGRESS
Generated: 2026-07-10T02:57:02Z

Execution:
- Actor: `codex`
- Model: `gpt-5/codex`
- Provider: `openai`
- Start Time: 2026-07-10T06:36:46Z
- End Time: 2026-07-10T06:53:58Z

## Summary

Implemented the bounded Runtime v2 loop runtime for issue #4695 in the bound worktree. The implementation consumes the WP-11 #4694 reasoning graph contract, validates graph/state binding, validates loop definitions against graph nodes and edges, enforces termination limits, emits deterministic replay events, exposes `adl runtime-v2 loop-runtime`, and adds focused negative tests for missing graph/state, missing graph nodes, invalid loop definitions, termination limits, forged replay order/final state, invalid resumed state, and deterministic canonical ordering. Local focused validation passes, but PR publication is blocked because no proving pre-PR review result is available.

## PVF Lane Truth
- Initial PVF lane: `runtime`
- Planned PVF lane: `runtime`
- Final PVF lane: `runtime`
- Lane change reason: `not_applicable; execution stayed in the runtime lane`

## Issue Metrics Truth
- Expected runtime class: `small`
- Estimated elapsed seconds: `14400`
- Actual elapsed seconds: `not_collected`
- Actual active work seconds: `not_collected`
- Estimated total tokens: `300000`
- Actual total tokens: `not_collected`
- Estimated validation seconds: `1800`
- Actual validation seconds: `not_collected`
- Actual PR wait seconds: `not_applicable_pre_pr`
- Actual CI wait seconds: `not_applicable_pre_pr`
- Budget source: `SPP estimate fields`
- Goal metrics data source: `unknown`
- Goal metrics source ref: `unknown`
- Data-source confidence: `unknown`
- Estimate error percent: `unknown`
- Completion state: `implementation_validated_review_blocked`
- Issue goal ref: `issue-4695`
- Sprint goal ref: `unknown`
- Goal metrics rollup ref: `unknown`
- Validation planning prompt: `.adl/v0.91.7/tasks/issue-4695__v0-91-7-wp-11-loops-implement-loop-runtime-in-full/vpp.md`
- Missing-telemetry rule: elapsed, active-work, validation, token, PR-wait, and CI-wait actuals are recorded as `not_collected` or `not_applicable_pre_pr`, not inferred from chat timestamps.
- Goal-metrics substrate note: terminal goal accounting remains pending until PR publication and required follow-through complete.

## Variance Analysis
- Threshold policy: require variance analysis when any known estimated/actual pair for elapsed seconds, total tokens, or validation seconds differs by more than 10 percent.
- Variance analysis required: `not_applicable`
- Variance analysis completed: `not_applicable`
- Variance category: `not_applicable`
- Variance note: Actual elapsed, active-work, validation, and token metrics were not collected in a machine-authoritative form during this pre-PR implementation slice.
- Sprint rollup guidance: keep `not_applicable` out of category totals and never treat unknown or not-collected metrics as zero variance.

## Artifacts produced
- Tracked implementation artifacts:
  - `adl/src/runtime_v2/loop_runtime.rs`
  - `adl/src/runtime_v2/tests/loop_runtime.rs`
  - `adl/src/runtime_v2/mod.rs`
  - `adl/src/runtime_v2/tests.rs`
  - `adl/src/cli/runtime_v2_cmd/commands.rs`
  - `adl/src/cli/runtime_v2_cmd/helpers.rs`
  - `adl/src/cli/runtime_v2_cmd/tests.rs`
  - `adl/src/cli/usage.rs`
- Local ignored proof artifact:
  - `artifacts/v0917/issue-4695-loop-runtime/loop-runtime.json`
- Additional proof artifacts: validation command output retained in terminal/session transcript; no non-ignored generated proof packet was created.
- Review artifacts:
  - `.adl/reviews/issue-4695-pre-pr-review/`
  - `.adl/reviews/issue-4695-pre-pr-review-live/`
  - `.adl/reviews/issue-4695-pre-pr-review-live-gemma12b/`
  - `.adl/reviews/issue-4695-pre-pr-review-live-gemma12b-committed/`
  - `.adl/reviews/issue-4695-pre-pr-review-live-phi4mini-committed/`
  - `.adl/reviews/issue-4695-pre-pr-review-live-phi4mini-focused/`

## Actions taken
- Added the `RuntimeV2LoopRuntimePacket`, loop definition, loop state, replay event, and replay validation contract.
- Integrated the loop runtime with `runtime_v2_reasoning_graph_contract()` and required matching graph ids, existing graph nodes, and graph edge endpoints.
- Implemented termination-limit enforcement with a bounded `MAX_LOOP_ITERATIONS` and per-definition `max_iterations`.
- Added deterministic replay validation, canonical sorting, and fail-closed resumed-state checks for unknown or non-prefix completed steps.
- Added `adl runtime-v2 loop-runtime --out OUT_PATH` command handling, stdout path reporting, help text, output-path validation, and CLI regression coverage.
- Generated the ignored local JSON proof artifact through the new command.

## Main Repo Integration (REQUIRED)
- Main-repo paths updated: not yet; this record is pre-PR worktree truth for branch `codex/4695-v0-91-7-wp-11-loops-implement-loop-runtime-in-full`.
- Worktree-only paths remaining: yes, until repo-native PR publication integrates the branch.
- Integration state: worktree_only
- Verification scope: worktree
- Integration method used: implementation on the bound issue branch/worktree, followed by focused local validation and pending repo-native PR publication.
- Verification performed:
  - `cargo fmt --manifest-path adl/Cargo.toml`
    `Verified Rust formatting for the touched crate after implementation edits.`
  - `cargo test --manifest-path adl/Cargo.toml runtime_v2_loop_runtime -- --nocapture`
    `Verified the Runtime v2 loop-runtime unit tests and matching CLI loop-runtime tests selected by the focused marker.`
  - `adl/target/debug/adl runtime-v2 loop-runtime --out artifacts/v0917/issue-4695-loop-runtime/loop-runtime.json`
    `Verified the new CLI path can emit the canonical loop-runtime packet to a repository-relative output path.`
  - `git diff --check`
    `Verified whitespace/diff hygiene for the worktree changes.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review --backend fixture --visibility read-only-repo --issue 4695 --include-working-tree --fixture-case clean`
    `Generated a fixture review packet with no findings, but the gate blocked PR opening because fixture review is non-proving.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live --backend ollama --visibility read-only-repo --issue 4695 --include-working-tree --allow-live-ollama --timeout-secs 90`
    `Attempted live repo-native review; the reviewer returned HTTP 404 Not Found and the gate blocked PR opening.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-gemma12b-committed --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model gemma4:12b-mlx --timeout-secs 180`
    `Attempted live repo-native review against the committed branch diff; the reviewer result was skipped because the local Ollama generate request failed, and the gate blocked PR opening.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-phi4mini-committed --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model phi4-mini:latest --timeout-secs 180`
    `Retried live repo-native review with a smaller installed local model; the reviewer result was skipped for the same Ollama generate transport failure, and the gate blocked PR opening.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-phi4mini-focused --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model phi4-mini:latest --timeout-secs 180 --file adl/src/runtime_v2/loop_runtime.rs --file adl/src/runtime_v2/tests/loop_runtime.rs --file adl/src/cli/runtime_v2_cmd/commands.rs --file adl/src/cli/runtime_v2_cmd/tests.rs --file adl/src/cli/runtime_v2_cmd/helpers.rs --file adl/src/runtime_v2/mod.rs --file adl/src/runtime_v2/tests.rs --file adl/src/cli/usage.rs`
    `Retried live repo-native review with a narrower focused packet over the changed implementation files; the reviewer result was skipped for the same Ollama generate transport failure, and the gate blocked PR opening.`
- Result: FAIL

Rules:
- Final artifacts must exist in the main repository, not only in a worktree.
- Do not leave docs, code, or generated artifacts only under a `adl-wp-*` worktree.
- Prefer git-aware transfer into the main repo through repo-native PR publication.
- If artifacts exist only in the worktree, the task is not closeout-complete.
- Integration state describes lifecycle state of the integrated artifact set, not where verification happened.
- Verification scope describes where the verification commands were run.
- `worktree_only` means implementation and local validation are done in the bound issue worktree, but PR and main integration truth are not terminal.
- `review_blocked` means repo-native review was attempted but did not produce a proving approval required for PR publication.
- Completed output records must not leave `Status` as `NOT_STARTED`.
- By `pr finish`, `Status` should normally be `DONE` or `FAILED` according to terminal issue truth.

## Validation
- Validation commands and their purpose:
  - `cargo fmt --manifest-path adl/Cargo.toml`
    `Formatted the Rust crate after implementation edits.`
  - `cargo test --manifest-path adl/Cargo.toml runtime_v2_loop_runtime -- --nocapture`
    `Ran 8 loop-runtime unit tests covering reasoning-graph integration, missing graph/state binding, invalid loop definitions, missing graph nodes, termination limits, deterministic replay, forged replay rejection, and invalid resumed state; the same marker also exercised the loop-runtime CLI write/help tests.`
  - `adl/target/debug/adl runtime-v2 loop-runtime --out artifacts/v0917/issue-4695-loop-runtime/loop-runtime.json`
    `Generated the bounded loop-runtime proof packet through the implemented CLI command.`
  - `git diff --check`
    `Checked diff whitespace hygiene.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review --backend fixture --visibility read-only-repo --issue 4695 --include-working-tree --fixture-case clean`
    `Ran repo-native fixture review; no findings were produced, but gate disposition was block_pr_open because fixture review is non-proving.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live --backend ollama --visibility read-only-repo --issue 4695 --include-working-tree --allow-live-ollama --timeout-secs 90`
    `Attempted live repo-native review; gate disposition was block_pr_open because Ollama returned HTTP 404 Not Found.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-gemma12b-committed --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model gemma4:12b-mlx --timeout-secs 180`
    `Attempted live review against the committed diff; gate disposition was block_pr_open because the reviewer was skipped after an Ollama generate transport failure.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-phi4mini-committed --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model phi4-mini:latest --timeout-secs 180`
    `Retried live review with a smaller installed model; gate disposition remained block_pr_open because the reviewer was skipped after an Ollama generate transport failure.`
  - `adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-phi4mini-focused --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model phi4-mini:latest --timeout-secs 180 --file adl/src/runtime_v2/loop_runtime.rs --file adl/src/runtime_v2/tests/loop_runtime.rs --file adl/src/cli/runtime_v2_cmd/commands.rs --file adl/src/cli/runtime_v2_cmd/tests.rs --file adl/src/cli/runtime_v2_cmd/helpers.rs --file adl/src/runtime_v2/mod.rs --file adl/src/runtime_v2/tests.rs --file adl/src/cli/usage.rs`
    `Retried live review with a narrower focused packet; gate disposition remained block_pr_open because the reviewer was skipped after an Ollama generate transport failure.`
- Results:
  - `PASS` for formatting, focused tests, CLI artifact generation, and diff hygiene.
  - `BLOCKED` for PR publication review gate.

Validation command/path rules:
- Recorded paths are repository-relative.
- No absolute host paths are required for this record.
- `absolute_path_leakage_detected: false` means the final recorded artifact does not contain unjustified absolute host paths.
- Commands are listed with their observed validation purpose.

## Verification Summary

```yaml
verification_summary:
  validation:
    status: PASS
    checks_run:
      - "cargo fmt --manifest-path adl/Cargo.toml"
      - "cargo test --manifest-path adl/Cargo.toml runtime_v2_loop_runtime -- --nocapture"
      - "adl/target/debug/adl runtime-v2 loop-runtime --out artifacts/v0917/issue-4695-loop-runtime/loop-runtime.json"
      - "git diff --check"
      - "adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review --backend fixture --visibility read-only-repo --issue 4695 --include-working-tree --fixture-case clean"
      - "adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live --backend ollama --visibility read-only-repo --issue 4695 --include-working-tree --allow-live-ollama --timeout-secs 90"
      - "adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-gemma12b-committed --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model gemma4:12b-mlx --timeout-secs 180"
      - "adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-phi4mini-committed --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model phi4-mini:latest --timeout-secs 180"
      - "adl/target/debug/adl tooling code-review --out .adl/reviews/issue-4695-pre-pr-review-live-phi4mini-focused --backend ollama --visibility read-only-repo --issue 4695 --allow-live-ollama --model phi4-mini:latest --timeout-secs 180 --file adl/src/runtime_v2/loop_runtime.rs --file adl/src/runtime_v2/tests/loop_runtime.rs --file adl/src/cli/runtime_v2_cmd/commands.rs --file adl/src/cli/runtime_v2_cmd/tests.rs --file adl/src/cli/runtime_v2_cmd/helpers.rs --file adl/src/runtime_v2/mod.rs --file adl/src/runtime_v2/tests.rs --file adl/src/cli/usage.rs"
  determinism:
    status: PASS
    replay_verified: true
    ordering_guarantees_verified: true
  security_privacy:
    status: PASS
    secrets_leakage_detected: false
    prompt_or_tool_arg_leakage_detected: false
    absolute_path_leakage_detected: false
  artifacts:
    status: PASS
    required_artifacts_present: true
    schema_changes:
      present: false
      approved: not_applicable
```

## Determinism Evidence
- Determinism tests executed: `runtime_v2_loop_runtime_replay_order_is_deterministic` and `runtime_v2_loop_runtime_rejects_forged_replay_order_and_final_state`.
- Fixtures or scripts used: built-in Runtime v2 reasoning graph prototype and the new loop-runtime contract.
- Replay verification (same inputs -> same artifacts/order): replay events must match loop-definition order, contiguous `event_sequence`, expected iterations, and deterministic final state.
- Ordering guarantees (sorting / tie-break rules used): canonical JSON sorts terminal nodes, loop steps, completed step ids, validation commands, non-claims, and replay guarantees.
- Artifact stability notes: the generated JSON packet is deterministic for the bundled WP-11 reasoning graph prototype.

## Security / Privacy Checks
- Secret leakage scan performed: content/path review of changed implementation and generated packet; no secrets were introduced.
- Prompt / tool argument redaction verified: no provider credentials, tokens, or private prompt payloads are recorded in the loop-runtime artifact.
- Absolute path leakage check: `git diff --check` passed, CLI stdout preserves requested repository-relative output paths, and the generated packet records only repository-relative refs.
- Sandbox / policy invariants preserved: work was performed only in `.worktrees/adl-wp-4695`; sibling WP issue worktrees were not edited.

## Replay Artifacts
- Trace bundle path(s): `not_applicable; this issue emits a loop-runtime packet rather than a trace bundle.`
- Run artifact root: `artifacts/v0917/issue-4695-loop-runtime/`
- Replay command used for verification: `cargo test --manifest-path adl/Cargo.toml runtime_v2_loop_runtime -- --nocapture`
- Replay result: PASS

## Artifact Verification
- Primary proof surface: `adl/src/runtime_v2/loop_runtime.rs` plus focused tests in `adl/src/runtime_v2/tests/loop_runtime.rs`.
- Required artifacts present: implementation, tests, CLI integration, usage text, and generated ignored packet are present in the issue worktree.
- Artifact schema/version checks: `schema_version` is `runtime_v2.loop_runtime.v1`; validation rejects mismatched schema, graph ids, missing graph refs, invalid loop definitions, and forged replay state.
- Hash/byte-stability checks: not_run; deterministic serialization tests cover canonical ordering but no hash manifest was required by this issue.
- Missing/optional artifacts and rationale: broad workspace test, coverage, remote/AWS proof, and release-gate CI are deferred to PR/CI policy because the issue touched a bounded Runtime v2 contract and CLI subcommand.

## Decisions / Deviations
- `VPP was updated from the generated docs-diff profile to a runtime-focused validation profile because the actual touched surface is Rust runtime and CLI code.`
- `The generated loop-runtime JSON proof is under ignored artifacts/ and is recorded as local proof evidence, not as a tracked release artifact.`
- `No v0.92 readiness, unbounded autonomy, adl.skill.v1 ratification, or full v0.94 reasoning/provenance graph engine claim is made.`

## Follow-ups / Deferred work
- `Obtain a proving pre-PR review result or explicit operator waiver; current repo-native review gate blocks PR publication.`
- `Publish through repo-native PR flow only after the review gate is satisfied.`
- `Record PR URL, CI state, and closeout truth after publication/merge through normal lifecycle.`
