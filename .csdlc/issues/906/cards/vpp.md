---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-process-parser-simplification-validation-plan"
issue: 906
task_id: "issue-0906"
run_id: "issue-0906"
version: "0.92.2"
title: "[v0.92.2][PLAT-RUST] Complete one selected production Rust responsibility refactor"
branch: "codex/906-v0922-process-parser-simplification"
generated_at: "2026-09-12T00:22:05.416044+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded_local"
validation_resource_profile: "local CPU/Rust/Python/Git/filesystem, isolated authenticated fixtures or immutable revision snapshots; no paid/live effects"
validation_family: "process_parser_behavior_preservation"
validation_size_split: "focused per source acceptance; no reflexive full workspace suite"
expected_proof_cost: "2400 seconds and 7000 tokens estimated deterministic proof; required CI queue time separate from local proof"
planned_validation_seconds: "2400"
planned_validation_tokens: "7000"
issue_goal_ref: "not_created; create issue-bound goal before implementation"
sprint_goal_ref: "v0.92.2 execution Sprint 7; umbrella management owned by #926"
goal_metrics_rollup_ref: ".csdlc/evidence/906/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/906"
  - kind: "stp"
    ref: ".csdlc/issues/906/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/906/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/906/cards/spp.md"
selected_lanes:
  - "tooling; Required tooling lane, deterministic pure-parser/installed CLI behavior preservation with small CPU and isolated owned process fixtures; no broad process scan or unsafe network targets. First enumerate cargo test --manifest-path adl/Cargo.toml --test cli_smoke -- --list; run cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status plus newly authored parser cases and require nonzero denominator. Record exact source baseline and recursive source/function/branch counts before and after, independently assess removal of duplicated target selection rather than facade shrink. Cover missing/malformed/repeated/conflicting flags, option order and error order, PID/port zero/bounds, loopback-only host, unchanged defaults and output schema. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check and required platform CI separately. New fixture inventory must declare role/determinism/resources/release gate. Compilation, moved lines or additional tests alone do not establish simplification."
parallel_groups:
  - "serial within this issue; independent fixtures may parallelize only with isolated state"
validation_commands:
  - "Required tooling lane, deterministic pure-parser/installed CLI behavior preservation with small CPU and isolated owned process fixtures; no broad process scan or unsafe network targets. First enumerate cargo test --manifest-path adl/Cargo.toml --test cli_smoke -- --list; run cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status plus newly authored parser cases and require nonzero denominator. Record exact source baseline and recursive source/function/branch counts before and after, independently assess removal of duplicated target selection rather than facade shrink. Cover missing/malformed/repeated/conflicting flags, option order and error order, PID/port zero/bounds, loopback-only host, unchanged defaults and output schema. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check and required platform CI separately. New fixture inventory must declare role/determinism/resources/release gate. Compilation, moved lines or additional tests alone do not establish simplification."
failure_policy: "Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof."
notes: "Ownership resolution 2026-09-16: historical worktree /Users/daniel/git/agent-design-language/.worktrees/adl-process-status-fanout on branch codex/reduce-process-status-fanout is operator-owned June 19 WIP at 1ea914010e6b96482a95bd6f64c8318f1a19b937. Its four-file dirty patch has SHA-256 aef64c67a22d74ff5dc4962d5c6f25ab09a06ca64bb1763c48984d73558fbf84; no origin or legacy-origin branch or PR exists. Preserve every byte there. Issue #906 will not copy, reset, cherry-pick, or modify that worktree and will execute only in its separate bound FastWork worktree from current origin/main. The source and CLI-test paths are clear through isolation; the unrelated finish files remain untouched."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Resolve dirty process-status-fanout source and CLI-test ownership without copying/resetting inherited work; freeze exact baseline plus recursive source/function/branch inventory and argv behavior corpus; extract ParsedStatus and pure parsing/validation helpers into process_cmd/args.rs and simplify duplicate target-selection representation while retaining production real_process_status caller; prove exact accepted/rejected flags, repeats/order/error order/defaults/zero bounds/loopback rules with parser and installed CLI regressions; compare recursive totals and control-flow simplification, preserve probe/output schema and supported-platform behavior, then independently review exact head.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- tooling; Required tooling lane, deterministic pure-parser/installed CLI behavior preservation with small CPU and isolated owned process fixtures; no broad process scan or unsafe network targets. First enumerate cargo test --manifest-path adl/Cargo.toml --test cli_smoke -- --list; run cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status plus newly authored parser cases and require nonzero denominator. Record exact source baseline and recursive source/function/branch counts before and after, independently assess removal of duplicated target selection rather than facade shrink. Cover missing/malformed/repeated/conflicting flags, option order and error order, PID/port zero/bounds, loopback-only host, unchanged defaults and output schema. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check and required platform CI separately. New fixture inventory must declare role/determinism/resources/release gate. Compilation, moved lines or additional tests alone do not establish simplification.

## Parallelization Plan

- Parallel groups: serial within this issue; independent fixtures may parallelize only with isolated state
- Validation runtime class: `bounded_local`
- Validation resource profile: `local CPU/Rust/Python/Git/filesystem, isolated authenticated fixtures or immutable revision snapshots; no paid/live effects`
- Validation family: `process_parser_behavior_preservation`
- Validation size split: `focused per source acceptance; no reflexive full workspace suite`

## Goal Accounting Hooks

- Issue goal ref: `not_created; create issue-bound goal before implementation`
- Sprint goal ref: `v0.92.2 execution Sprint 7; umbrella management owned by #926`
- Goal metrics rollup ref: `.csdlc/evidence/906/goal-metrics.json (planned; absent until execution)`

## Proof Cost / Runtime Expectations

- Expected proof cost: `2400 seconds and 7000 tokens estimated deterministic proof; required CI queue time separate from local proof`
- Planned validation seconds: `2400`
- Planned validation token budget: `7000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Required tooling lane, deterministic pure-parser/installed CLI behavior preservation with small CPU and isolated owned process fixtures; no broad process scan or unsafe network targets. First enumerate cargo test --manifest-path adl/Cargo.toml --test cli_smoke -- --list; run cargo test --manifest-path adl/Cargo.toml --test cli_smoke process_status plus newly authored parser cases and require nonzero denominator. Record exact source baseline and recursive source/function/branch counts before and after, independently assess removal of duplicated target selection rather than facade shrink. Cover missing/malformed/repeated/conflicting flags, option order and error order, PID/port zero/bounds, loopback-only host, unchanged defaults and output schema. Run cargo fmt --manifest-path adl/Cargo.toml --check and git diff --check and required platform CI separately. New fixture inventory must declare role/determinism/resources/release gate. Compilation, moved lines or additional tests alone do not establish simplification.

## Failure Semantics

- Required failures, skipped or zero-test proof block acceptance. Preserve guards; record durable anomalies; repair and rerun affected proof and independent exact-head review. CI evidence is separate from local proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Ownership resolution 2026-09-16: historical worktree /Users/daniel/git/agent-design-language/.worktrees/adl-process-status-fanout on branch codex/reduce-process-status-fanout is operator-owned June 19 WIP at 1ea914010e6b96482a95bd6f64c8318f1a19b937. Its four-file dirty patch has SHA-256 aef64c67a22d74ff5dc4962d5c6f25ab09a06ca64bb1763c48984d73558fbf84; no origin or legacy-origin branch or PR exists. Preserve every byte there. Issue #906 will not copy, reset, cherry-pick, or modify that worktree and will execute only in its separate bound FastWork worktree from current origin/main. The source and CLI-test paths are clear through isolation; the unrelated finish files remain untouched.
