---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "codefriend-result-integrity-validation-plan"
issue: 1162
task_id: "issue-1162"
run_id: "issue-1162"
version: "0.92.2"
title: "[v0.92.2][TAIL-06][P1] Repair CodeFriend result integrity and website interoperability"
branch: "codex/1162-codefriend-result-integrity"
generated_at: "2026-09-23T17:29:32.826544+00:00"
card_status: "ready"
status: "planned"
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "local_focused"
validation_resource_profile: "local_cpu_disk_isolated_git_processes_no_live_credentials"
validation_family: "csdlc_recovery_transport_operator_contract"
validation_size_split: "focused_targets"
expected_proof_cost: "local CPU and disk; no model or cloud spend"
planned_validation_seconds: "1800"
planned_validation_tokens: "unknown"
issue_goal_ref: "issue-1162"
sprint_goal_ref: "issue-921"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1162"
  - kind: "stp"
    ref: ".csdlc/issues/1162/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1162/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/1162/cards/spp.md"
selected_lanes:
  - "tooling; owner_binary"
parallel_groups:
  - "Only disjoint isolated fixtures may run in parallel; serialize shared owner installation and lifecycle mutation."
validation_commands:
  - "cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_html; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_review; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_agent_publication. Website component runs focused Node regressions and full npm test outside native Cargo proof. New tests declare release/contract/tooling lane as applicable, regression proof role, deterministic local fixtures, local CPU/disk resources, and required gate status. No paid provider, deployment, or live user action."
failure_policy: "Any failed, zero-test, skipped mandatory, stale-candidate or missing regression blocks acceptance; retain evidence and repair without hiding failure."
notes: "Cross-repository compatibility can falsely pass with hand-authored fixtures; fixtures must be emitted by the actual native v4 producer. PDF validation must inspect independently reconstructed semantics and reject active/external content. Async reauthorization must retain controlled identity and fail closed after revocation."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Repair the seven Group B CodeFriend result-integrity and website-interoperability findings from #919 as one aggregate issue under #921, while preserving #918 and #919.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`

## Selected Validation Lanes

- tooling; owner_binary

## Parallelization Plan

- Parallel groups: Only disjoint isolated fixtures may run in parallel; serialize shared owner installation and lifecycle mutation.
- Validation runtime class: `local_focused`
- Validation resource profile: `local_cpu_disk_isolated_git_processes_no_live_credentials`
- Validation family: `csdlc_recovery_transport_operator_contract`
- Validation size split: `focused_targets`

## Goal Accounting Hooks

- Issue goal ref: `issue-1162`
- Sprint goal ref: `issue-921`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `local CPU and disk; no model or cloud spend`
- Planned validation seconds: `1800`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_html; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_review; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_agent_publication. Website component runs focused Node regressions and full npm test outside native Cargo proof. New tests declare release/contract/tooling lane as applicable, regression proof role, deterministic local fixtures, local CPU/disk resources, and required gate status. No paid provider, deployment, or live user action.

## Failure Semantics

- Any failed, zero-test, skipped mandatory, stale-candidate or missing regression blocks acceptance; retain evidence and repair without hiding failure.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Cross-repository compatibility can falsely pass with hand-authored fixtures; fixtures must be emitted by the actual native v4 producer. PDF validation must inspect independently reconstructed semantics and reject active/external content. Async reauthorization must retain controlled identity and fail closed after revocation.
