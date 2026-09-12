---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "908-aws-inventory-validation-plan"
issue: 908
task_id: "issue-0908"
run_id: "issue-0908"
version: "v0.92.2"
title: "[v0.92.2][OPS-AWS] Produce one current AWS inventory packet from the #484 baseline"
branch: "codex/908-aws-inventory"
generated_at: "2026-09-12T00:09:29.974020+00:00"
card_status: "ready"
status: "prepared"
initial_pvf_lane: "cloud-operations"
planned_pvf_lane: "cloud-operations"
lane_registry_path: "docs/templates/prompts/1.0.5"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "bounded"
validation_resource_profile: "Read-only AWS inventory plus deterministic completeness/redaction negatives"
validation_family: "Read-only AWS inventory plus deterministic completeness/redaction negatives"
validation_size_split: "Small local Python checks plus bounded external AWS API reads; no Rust changes."
expected_proof_cost: "Up to 900 seconds of read-only cloud and local CPU; no cloud resource creation."
planned_validation_seconds: "1800"
planned_validation_tokens: "12000"
issue_goal_ref: "Goal #908 created for current inventory, reviewed PR and green CI under umbrella #934"
sprint_goal_ref: "Sprint 8 umbrella #934; AWS inventory child #908"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/908"
  - kind: "stp"
    ref: ".csdlc/issues/908/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/908/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/908/cards/spp.md"
selected_lanes:
  - "External read-only AWS observation; deterministic local packet completeness, redaction and negative tests; independent evidence review."
parallel_groups:
  - "Independent read-only regional calls after successful business identity guard."
validation_commands:
  - "python3 .csdlc/evidence/908/inventory.py capture; python3 .csdlc/evidence/908/inventory.py validate; python3 .csdlc/evidence/908/test_inventory.py; independent review of sanitized actual readbacks and delta"
failure_policy: "Missing or failed proof blocks acceptance; read failure is not resource absence; local checks do not prove live deployment."
notes: "Read failures are not absence. Unknown ownership remains frozen. Raw identities, credentials and object contents never enter new evidence."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Current sanitized read-only business AWS inventory delta against immutable #484, including SCR, S3, model artifacts and maintenance record.

## Lane Registry Inputs

- Registry path: `docs/templates/prompts/1.0.5`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `cloud-operations`
- Planned PVF lane for execution: `cloud-operations`

## Selected Validation Lanes

- External read-only AWS observation; deterministic local packet completeness, redaction and negative tests; independent evidence review.

## Parallelization Plan

- Parallel groups: Independent read-only regional calls after successful business identity guard.
- Validation runtime class: `bounded`
- Validation resource profile: `Read-only AWS inventory plus deterministic completeness/redaction negatives`
- Validation family: `Read-only AWS inventory plus deterministic completeness/redaction negatives`
- Validation size split: `Small local Python checks plus bounded external AWS API reads; no Rust changes.`

## Goal Accounting Hooks

- Issue goal ref: `Goal #908 created for current inventory, reviewed PR and green CI under umbrella #934`
- Sprint goal ref: `Sprint 8 umbrella #934; AWS inventory child #908`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Up to 900 seconds of read-only cloud and local CPU; no cloud resource creation.`
- Planned validation seconds: `1800`
- Planned validation token budget: `12000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- python3 .csdlc/evidence/908/inventory.py capture; python3 .csdlc/evidence/908/inventory.py validate; python3 .csdlc/evidence/908/test_inventory.py; independent review of sanitized actual readbacks and delta

## Failure Semantics

- Missing or failed proof blocks acceptance; read failure is not resource absence; local checks do not prove live deployment.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

Read failures are not absence. Unknown ownership remains frozen. Raw identities, credentials and object contents never enter new evidence.
