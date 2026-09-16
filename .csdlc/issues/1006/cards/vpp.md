---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "v0922-native-coordination-completion-validation-plan"
issue: 1006
task_id: "issue-1006"
run_id: "issue-1006"
version: "0.92.2"
title: "[v0.92.2][C-SDLC] Support native completion closure for coordination issues"
branch: "codex/1006-v0922-native-coordination-completion"
generated_at: "2026-09-16T02:45:19.992791+00:00"
card_status: "ready"
status: "local_pass_review_pass"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "adl/config/validation_lane_selector.v0.91.6.json"
lane_registry_template_set: "0.91.6"
validation_runtime_class: "small deterministic CPU/filesystem and fixture HTTP adapter"
validation_resource_profile: "Local isolated Git/temp directories and fixture transport; no credentials, paid services, real GitHub writes or source repository execution."
validation_family: "csdlc_native_remote_terminal"
validation_size_split: "Small focused owner and installed fixture tests; broad suites only if touched shared contracts require them."
expected_proof_cost: "Small focused proof; no user time or token limit assigned."
planned_validation_seconds: "not_budgeted_by_operator"
planned_validation_tokens: "not_budgeted_by_operator"
issue_goal_ref: "Active parent combined #1003/#1006 goal; no budget assigned."
sprint_goal_ref: "No active sprint execution budget assigned to this repair."
goal_metrics_rollup_ref: "No metrics rollup measured during preparation."
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1006"
  - kind: "stp"
    ref: ".csdlc/issues/1006/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1006/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1006/cards/spp.md"
selected_lanes:
  - "Small local deterministic owner tests and installed CLI fixture lane; hosted required CI separately; no paid/provider/cloud qualification."
parallel_groups:
  - "Independent read-only review may run beside focused validation; serialize shared-source edits with #1003."
validation_commands:
  - "cargo test --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::; cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands --test terminal_cleanup_cutover_commands; cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_coordination_completion; focused installed_intent_commands admin/terminal regressions; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check"
failure_policy: "Any missing or failed required semantic proof, uncertain reconciliation or invalid identity/evidence blocks publication; never count skipped lanes as passed."
notes: "Coordination classification and evidence freshness must be verified, not self-certified. Prevent stale state/replay, identity substitution and implicit ordinary-issue completion. #929 workaround is historical evidence, not authority for new bypasses."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

All declared bounded proof executed:123 tests and strict checks PASS; independent source review PASS. Hosted required CI separately pending.

## Lane Registry Inputs

- Registry path: `adl/config/validation_lane_selector.v0.91.6.json`
- Registry template set: `0.91.6`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- Small local deterministic owner tests and installed CLI fixture lane; hosted required CI separately; no paid/provider/cloud qualification.

## Parallelization Plan

- Parallel groups: Independent read-only review may run beside focused validation; serialize shared-source edits with #1003.
- Validation runtime class: `small deterministic CPU/filesystem and fixture HTTP adapter`
- Validation resource profile: `Local isolated Git/temp directories and fixture transport; no credentials, paid services, real GitHub writes or source repository execution.`
- Validation family: `csdlc_native_remote_terminal`
- Validation size split: `Small focused owner and installed fixture tests; broad suites only if touched shared contracts require them.`

## Goal Accounting Hooks

- Issue goal ref: `Active parent combined #1003/#1006 goal; no budget assigned.`
- Sprint goal ref: `No active sprint execution budget assigned to this repair.`
- Goal metrics rollup ref: `No metrics rollup measured during preparation.`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small focused proof; no user time or token limit assigned.`
- Planned validation seconds: `not_budgeted_by_operator`
- Planned validation token budget: `not_budgeted_by_operator`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path csdlc-v3/Cargo.toml --lib commands::remote::; cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands --test terminal_cleanup_cutover_commands; cargo test --manifest-path csdlc-v3/Cargo.toml --test installed_coordination_completion; focused installed_intent_commands admin/terminal regressions; cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings; cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check

## Failure Semantics

- Any missing or failed required semantic proof, uncertain reconciliation or invalid identity/evidence blocks publication; never count skipped lanes as passed.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Coordination classification and evidence freshness must be verified, not self-certified. Prevent stale state/replay, identity substitution and implicit ordinary-issue completion. #929 workaround is historical evidence, not authority for new bypasses.
