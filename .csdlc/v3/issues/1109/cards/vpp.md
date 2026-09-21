---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "codefriend-four-plus-one-architecture-validation-plan"
issue: 1109
task_id: "issue-1109"
run_id: "issue-1109"
version: "1.0.5"
title: "[v0.92.2][CF-ARCH] Generate the complete 4+1 architecture package in Beta 1"
branch: "codex/1109-codefriend-four-plus-one-architecture"
generated_at: "<timestamp>"
card_status: "ready"
status: "IN_PROGRESS"
initial_pvf_lane: "runtime_full_validation"
planned_pvf_lane: "runtime_full_validation"
lane_registry_path: "docs/milestones/v0.92.2/evidence/issue-1109/README.md"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "deterministic-local-and-installed"
validation_resource_profile: "local CPU and five explicit single-attempt Anthropic generation calls; no other paid/cloud experiments"
validation_family: "codefriend"
validation_size_split: "focused-then-installed"
expected_proof_cost: "unknown"
planned_validation_seconds: "unknown"
planned_validation_tokens: "unknown"
issue_goal_ref: "issue-1109-current-session"
sprint_goal_ref: "not-assigned"
goal_metrics_rollup_ref: "issue-1109-current-session"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1109"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "Focused deterministic 35-test contract/native-journey lane locally; isolated installed ADL/external generation; fixture PDF inspection and HTML structure checks; broader runtime/coverage integration deferred to GitHub CI because no changed runtime kernel/provider contract requires broad local rerun."
parallel_groups:
  - "sequential implementation and proof; bounded pre-PR independent review"
validation_commands:
  - "cargo test --manifest-path adl/Cargo.toml --test codefriend_four_plus_one --test codefriend_journey; cargo fmt --manifest-path adl/Cargo.toml --check; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_four_plus_one --test codefriend_journey -- -D warnings; installed adl codefriend journey local and resume; native csdlc proof 1109 before publication."
failure_policy: "Fail closed on missing view evidence, mismatched revision, invalid references, or unrun required installed proof."
notes: "35 focused tests and clippy passed at 69afb828f3; final exact-head native proof required after HTML table repair. Installed ADL and external generation/status retrieval passed with partial evidence; neither proves #915. Browser visual inspection unavailable due supported runtime bootstrap failure; PDF pages inspected and HTML asset/link structure checked."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Generate a source-bound 4+1 package in the existing CodeFriend journey and publish through existing report owners; qualify complete, incomplete and conflicting evidence.

## Lane Registry Inputs

- Registry path: `docs/milestones/v0.92.2/evidence/issue-1109/README.md`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `runtime_full_validation`
- Planned PVF lane for execution: `runtime_full_validation`

## Selected Validation Lanes

- Focused deterministic 35-test contract/native-journey lane locally; isolated installed ADL/external generation; fixture PDF inspection and HTML structure checks; broader runtime/coverage integration deferred to GitHub CI because no changed runtime kernel/provider contract requires broad local rerun.

## Parallelization Plan

- Parallel groups: sequential implementation and proof; bounded pre-PR independent review
- Validation runtime class: `deterministic-local-and-installed`
- Validation resource profile: `local CPU and five explicit single-attempt Anthropic generation calls; no other paid/cloud experiments`
- Validation family: `codefriend`
- Validation size split: `focused-then-installed`

## Goal Accounting Hooks

- Issue goal ref: `issue-1109-current-session`
- Sprint goal ref: `not-assigned`
- Goal metrics rollup ref: `issue-1109-current-session`

## Proof Cost / Runtime Expectations

- Expected proof cost: `unknown`
- Planned validation seconds: `unknown`
- Planned validation token budget: `unknown`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- cargo test --manifest-path adl/Cargo.toml --test codefriend_four_plus_one --test codefriend_journey; cargo fmt --manifest-path adl/Cargo.toml --check; cargo clippy --manifest-path adl/Cargo.toml --test codefriend_four_plus_one --test codefriend_journey -- -D warnings; installed adl codefriend journey local and resume; native csdlc proof 1109 before publication.

## Failure Semantics

- Fail closed on missing view evidence, mismatched revision, invalid references, or unrun required installed proof.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

35 focused tests and clippy passed at 69afb828f3; final exact-head native proof required after HTML table repair. Installed ADL and external generation/status retrieval passed with partial evidence; neither proves #915. Browser visual inspection unavailable due supported runtime bootstrap failure; PDF pages inspected and HTML asset/link structure checked.
