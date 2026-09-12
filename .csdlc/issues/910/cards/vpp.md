---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "910-observatory-deploy-validation-plan"
issue: 910
task_id: "issue-0910"
run_id: "issue-0910"
version: "v0.92.2"
title: "[v0.92.2][OBS-S3] Deploy the existing Observatory S3 and CloudFront sidecar"
branch: "codex/910-observatory-deploy"
generated_at: "2026-09-12T01:34:22.125782+00:00"
card_status: "ready"
status: "prepared"
initial_pvf_lane: "cloud-operations"
planned_pvf_lane: "cloud-operations"
lane_registry_path: "docs/templates/prompts/1.0.5"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded"
validation_resource_profile: "Plan/readback/browser live lanes; actual apply/upload blocked until explicit approval; no Runtime compute"
validation_family: "Plan/readback/browser live lanes; actual apply/upload blocked until explicit approval; no Runtime compute"
validation_size_split: "Focused local scripts/Terraform and external read-only AWS; no Rust changes planned."
expected_proof_cost: "Read-only API calls and local CPU during preparation; deployment cost approval pending exact plan."
planned_validation_seconds: "1800"
planned_validation_tokens: "12000"
issue_goal_ref: "Active #910 preparation goal: exact reviewed deployment approval handoff; no cloud writes"
sprint_goal_ref: "Sprint 8 umbrella #934"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/910"
  - kind: "stp"
    ref: ".csdlc/issues/910/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/910/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/910/cards/spp.md"
selected_lanes:
  - "Local static/plan checks and external read-only preflight now; authenticated apply/upload/readback/browser lanes gated on precise approval."
parallel_groups:
  - "Independent local asset checks and read-only cloud metadata after identity guard."
validation_commands:
  - "Existing Observatory Terraform/static validator; exact static asset hash/secret checks; explicit read-only business AWS/DNS/Runtime preflight; isolated backend-disabled Terraform fmt/validate/plan; independent plan/evidence review. No apply/upload/invalidation."
failure_policy: "Missing or failed proof blocks acceptance; read failure is not resource absence; local checks do not prove live deployment."
notes: "Stop on unavailable live Runtime origins, wrong identity, unknown state/custody, destructive plan or secret exposure. Preparation is not deployed completion."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Prepare exact reviewed Observatory assets/infrastructure deployment and approval packet; after explicit precise approval deploy and prove live HTTPS/WSS. This preparation phase performs no cloud writes.

## Lane Registry Inputs

- Registry path: `docs/templates/prompts/1.0.5`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `cloud-operations`
- Planned PVF lane for execution: `cloud-operations`

## Selected Validation Lanes

- Local static/plan checks and external read-only preflight now; authenticated apply/upload/readback/browser lanes gated on precise approval.

## Parallelization Plan

- Parallel groups: Independent local asset checks and read-only cloud metadata after identity guard.
- Validation runtime class: `bounded`
- Validation resource profile: `Plan/readback/browser live lanes; actual apply/upload blocked until explicit approval; no Runtime compute`
- Validation family: `Plan/readback/browser live lanes; actual apply/upload blocked until explicit approval; no Runtime compute`
- Validation size split: `Focused local scripts/Terraform and external read-only AWS; no Rust changes planned.`

## Goal Accounting Hooks

- Issue goal ref: `Active #910 preparation goal: exact reviewed deployment approval handoff; no cloud writes`
- Sprint goal ref: `Sprint 8 umbrella #934`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Read-only API calls and local CPU during preparation; deployment cost approval pending exact plan.`
- Planned validation seconds: `1800`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- Existing Observatory Terraform/static validator; exact static asset hash/secret checks; explicit read-only business AWS/DNS/Runtime preflight; isolated backend-disabled Terraform fmt/validate/plan; independent plan/evidence review. No apply/upload/invalidation.

## Failure Semantics

- Missing or failed proof blocks acceptance; read failure is not resource absence; local checks do not prove live deployment.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Stop on unavailable live Runtime origins, wrong identity, unknown state/custody, destructive plan or secret exposure. Preparation is not deployed completion.
