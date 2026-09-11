---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "<slug>-validation-plan"
issue: 522
task_id: "issue-0522"
run_id: "issue-0522"
version: "v0.92.1"
title: "[v0.92.1][TAIL-06] Review findings remediation"
branch: "codex/522-third-party-review-remediation"
generated_at: "<timestamp>"
card_status: "ready"
status: "<status>"
initial_pvf_lane: "finding-census"
planned_pvf_lane: "release-evidence"
lane_registry_path: "<lane_registry_path>"
lane_registry_template_set: "<lane_registry_template_set>"
validation_runtime_class: "small"
validation_resource_profile: "Local deterministic release-evidence validation plus authenticated GitHub readback; no cloud or GPU resources."
validation_family: "<validation_family>"
validation_size_split: "<validation_size_split>"
expected_proof_cost: "<expected_proof_cost>"
planned_validation_seconds: "900"
planned_validation_tokens: "8000"
issue_goal_ref: "<issue_goal_ref>"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/522"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
  - kind: "spp"
    ref: "<spp_card>"
selected_lanes:
  - "finding-census; remediation-proof; deferral-truth; packet-integrity; diff-hygiene"
parallel_groups:
  - "<parallel_groups_inline>"
validation_commands:
  - "ruby .csdlc/prepared/issues/522/test-production-validator.rb; ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb census; ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb dispositions; ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb all; git diff --check"
failure_policy: "Fail closed on source mismatch, duplicate or missing finding, stale/non-ancestral remediation, missing exact-head review, weak validation, incomplete deferral, or unresolved release blocker."
notes: "<notes_risks_inline>"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Ingest terminal #520/#521 sources; preserve the 19-finding census; verify every merged remediation; consume #851; execute #833 at the immutable candidate; assemble and validate the terminal ledger.

## Lane Registry Inputs

- Registry path: `<lane_registry_path>`
- Registry template set: `<lane_registry_template_set>`
- Initial PVF lane from issue creation: `finding-census`
- Planned PVF lane for execution: `release-evidence`

## Selected Validation Lanes

- finding-census; remediation-proof; deferral-truth; packet-integrity; diff-hygiene

## Parallelization Plan

- Parallel groups: <parallel_groups_inline>
- Validation runtime class: `small`
- Validation resource profile: `Local deterministic release-evidence validation plus authenticated GitHub readback; no cloud or GPU resources.`
- Validation family: `<validation_family>`
- Validation size split: `<validation_size_split>`

## Goal Accounting Hooks

- Issue goal ref: `<issue_goal_ref>`
- Sprint goal ref: `<sprint_goal_ref>`
- Goal metrics rollup ref: `<goal_metrics_rollup_ref>`

## Proof Cost / Runtime Expectations

- Expected proof cost: `<expected_proof_cost>`
- Planned validation seconds: `900`
- Planned validation token budget: `8000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- ruby .csdlc/prepared/issues/522/test-production-validator.rb; ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb census; ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb dispositions; ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb all; git diff --check

## Failure Semantics

- Fail closed on source mismatch, duplicate or missing finding, stale/non-ancestral remediation, missing exact-head review, weak validation, incomplete deferral, or unresolved release blocker.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

<notes_risks_inline>
