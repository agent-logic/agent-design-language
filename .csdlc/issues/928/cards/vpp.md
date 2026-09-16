---
schema_version: "0.1"
artifact_type: "structured_validation_planning_prompt"
name: "combined-sprint-review-validation-plan"
issue: 928
task_id: "issue-0928"
run_id: "issue-0928"
version: "v0.92.2"
title: "[v0.92.2][Sprint 2] Runtime/provider foundations and ingestion"
branch: "codex/928-combined-sprint-review"
generated_at: "2026-09-12T06:54:28.990815+00:00"
card_status: "ready"
status: "IN_PROGRESS"
initial_pvf_lane: "docs_diff_check"
planned_pvf_lane: "docs_diff_check"
lane_registry_path: "docs/validation/pvf_lanes.json"
lane_registry_template_set: "1.0.5"
validation_runtime_class: "small"
validation_resource_profile: "Local filesystem, Git object database, and bounded read-only GitHub queries; no paid provider calls"
validation_family: "sprint_review"
validation_size_split: "One nine-child ledger, one separate corrective row, seven lane records"
expected_proof_cost: "Small local review and hosted CI; no new paid execution"
planned_validation_seconds: "900"
planned_validation_tokens: "6000"
issue_goal_ref: "Active Sprint 2 completion objective under #928."
sprint_goal_ref: "#928"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/928"
  - kind: "stp"
    ref: ".csdlc/issues/928/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/928/cards/sip.md"
  - kind: "spp"
    ref: ".csdlc/issues/928/cards/spp.md"
selected_lanes:
  - "Native six-card validation; packet JSON/Markdown hygiene; nine-child plus separate corrective ledger; live issue/PR/check observation; ten merge ancestry checks; seven sprint-review lanes; independent exact-head review; PR CI."
parallel_groups:
  - "Independent review lanes after final source freeze; no overlapping child edits"
validation_commands:
  - "csdlc validate; jq empty on packet JSON; git diff --check; deterministic packet-accounting assertion; git merge-base --is-ancestor for each recorded merge; read-only GitHub issue/PR/check views."
failure_policy: "Fail closed for a missing original child, duplicate accounting, stale accepted head, failed applicable check, missing merge ancestry, undispositioned P1/P2 finding, absent lane record, or overclaimed proof. Preserve non-blocking residuals explicitly."
notes: "No broad child suite rerun by umbrella. Accepted-head CI and issue-level proof are inputs. The combined acquisition-to-store path remains unexecuted; child terminal receipts remain asynchronous."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/vpp.md`

# Structured Validation Planning Prompt

## Validation Planning Summary

Validate the completed combined review packet with native card checks, exact nine-child and separate corrective accounting, live GitHub state, Git ancestry, focused document and JSON hygiene, independent exact-head review, and hosted CI after publication.

## Lane Registry Inputs

- Registry path: `docs/validation/pvf_lanes.json`
- Registry template set: `1.0.5`
- Initial PVF lane from issue creation: `docs_diff_check`
- Planned PVF lane for execution: `docs_diff_check`

## Selected Validation Lanes

- Native six-card validation; packet JSON/Markdown hygiene; nine-child plus separate corrective ledger; live issue/PR/check observation; ten merge ancestry checks; seven sprint-review lanes; independent exact-head review; PR CI.

## Parallelization Plan

- Parallel groups: Independent review lanes after final source freeze; no overlapping child edits
- Validation runtime class: `small`
- Validation resource profile: `Local filesystem, Git object database, and bounded read-only GitHub queries; no paid provider calls`
- Validation family: `sprint_review`
- Validation size split: `One nine-child ledger, one separate corrective row, seven lane records`

## Goal Accounting Hooks

- Issue goal ref: `Active Sprint 2 completion objective under #928.`
- Sprint goal ref: `#928`
- Goal metrics rollup ref: `not_collected`

## Proof Cost / Runtime Expectations

- Expected proof cost: `Small local review and hosted CI; no new paid execution`
- Planned validation seconds: `900`
- Planned validation token budget: `6000`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Validation Commands

- csdlc validate; jq empty on packet JSON; git diff --check; deterministic packet-accounting assertion; git merge-base --is-ancestor for each recorded merge; read-only GitHub issue/PR/check views.

## Failure Semantics

- Fail closed for a missing original child, duplicate accounting, stale accepted head, failed applicable check, missing merge ancestry, undispositioned P1/P2 finding, absent lane record, or overclaimed proof. Preserve non-blocking residuals explicitly.

## Handoff

Use this VPP to bridge planning and execution. Keep lane assignment fail-closed, keep blocked or skipped states explicit, and update `SOR` if actual validation differs materially from this plan.

## Notes

No broad child suite rerun by umbrella. Accepted-head CI and issue-level proof are inputs. The combined acquisition-to-store path remains unexecuted; child terminal receipts remain asynchronous.
