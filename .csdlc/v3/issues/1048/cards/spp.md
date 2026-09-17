---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "publication-metadata-amendment-execution-plan"
issue: 1048
task_id: "issue-1048"
run_id: "issue-1048"
version: "v0.92.2"
title: "[C-SDLC] Allow typed correction of accepted publication metadata before dispatch"
branch: "codex/1048-publication-metadata-amendment"
generated_at: "2026-09-17T00:17:50.059444+00:00"
card_status: "ready"
status: "pre_execution"
activation_state: "<activation_state>"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "<planned_pvf_lane>"
planned_pvf_lane_source: "<planned_pvf_lane_source>"
estimate_elapsed_seconds: "<estimate_elapsed_seconds>"
estimate_total_tokens: "<estimate_total_tokens>"
estimate_validation_seconds: "<estimate_validation_seconds>"
issue_goal_token_budget: "not_set"
variance_threshold_percent: "<variance_threshold_percent>"
estimate_confidence: "<estimate_confidence>"
estimate_data_source: "<estimate_data_source>"
estimate_source_ref: "<estimate_source_ref>"
issue_goal_ref: "Issue #1048 implementation and reviewed publication goal"
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1048"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1048"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "csdlc-v3/src/application/intent; csdlc-v3/src/lifecycle/semantic.rs; csdlc-v3/src/storage/semantic; csdlc-v3/tests; docs/csdlc-v3; issue-local generated cards and sanitized evidence."
  components:
    - "publication-metadata-amendment"
  out_of_scope:
    - "No #1042 edits, no cloud/DNS changes, no raw GitHub bypass, no shared owner replacement without coordination, no retroactive alteration of receipts or hand editing semantic state."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "1. Reproduce inline-closing publication rejection and inspect amendment/recovery model. 2. Add minimal typed publication amendment plus prepare admission preserving identities and invalidating downstream evidence. 3. Add deterministic positive/negative regressions including interruption/recovery and duplicate guards. 4. Correct supported manual examples. 5. Run focused checks, independent exact-head review, and native PR publication."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: No prerequisite issue. Preserve #1017 accepted evidence and #1049 raw-transport audit; repair metadata independently before any later reconciliation. Current main baseline; no existing #1048 binding."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #1048 and retained #1017 publication rejection; AGENTS.md; csdlc-v3/AGENTS.md; current native selector; lifecycle/semantic.rs; storage/semantic.rs; application/intent/local.rs and remote.rs; docs/csdlc-v3/INTENT_COMMANDS.md and operator manual."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Typed publication amendment, initial metadata admission, deterministic regression tests, current operator docs, exact-head review and draft PR."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Typed publication amendment preserves issue/binding identity, validates title/body/base and exact closing linkage, invalidates proof/review/publication as required, and allows fresh review then publication. Malformed initial metadata rejected before durable preparation. Negative fixtures prove no remote dispatch on failed admission, base/head/issue and duplicate-create guards retained. Current manual examples use supported routes."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "completed"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "completed"
  - step: "Implement the bounded deliverables only."
    status: "completed"
  - step: "Run focused validation and proof gates."
    status: "completed"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "completed"
affected_areas:
  - "publication-metadata-amendment"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "<risks_inline>"
test_strategy:
  - "Focused Rust semantic amendment/storage/intent command tests and cargo fmt; command-manifest/manual contract checks for touched documentation. Fixtures simulate remote effects and assert zero dispatch on rejected admission. PVF deterministic local tooling/contract proof, small CPU/local Git, no live cloud writes. Actual migration evidence is outside this issue."
execution_handoff: "Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges."
required_permissions:
  - "workspace-write after execution approval"
stop_conditions:
  - "Stop and re-plan if dependencies are unmet or materially different from this design-time plan."
  - "Stop and update SPP if touched files, proof gates, or validation commands change materially."
  - "Stop and route follow-on work if acceptance requires scope outside this issue."
alternatives_considered:
  - description: "Rely only on transient chat planning."
    reason_not_chosen: "Chat-only planning is not durable or reviewable enough for this workflow surface."
review_hooks:
  - "Check dependency truth, scope truthfulness, touched-file truthfulness, validation sufficiency, and re-plan triggers."
notes: "Implementation adds typed publication amendment, initial metadata admission, pending-amendment proof rejection, and immutable proof-bound reviews so renewed review at the same HEAD does not overwrite prior evidence. Preserve legacy review receipt dispatch paths. Focused regression and independent review in progress; no remote publication yet."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[C-SDLC] Allow typed correction of accepted publication metadata before dispatch`.

1. Reproduce inline-closing publication rejection and inspect amendment/recovery model. 2. Add minimal typed publication amendment plus prepare admission preserving identities and invalidating downstream evidence. 3. Add deterministic positive/negative regressions including interruption/recovery and duplicate guards. 4. Correct supported manual examples. 5. Run focused checks, independent exact-head review, and native PR publication.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `<planned_pvf_lane>`
- Planning lane source: `<planned_pvf_lane_source>`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `<estimate_elapsed_seconds>`
- Estimated total tokens: `<estimate_total_tokens>`
- Estimated validation seconds: `<estimate_validation_seconds>`
- Issue goal token budget: `not_set`
- Variance threshold percent: `<variance_threshold_percent>`
- Estimate confidence: `<estimate_confidence>`
- Estimate data source: `<estimate_data_source>`
- Estimate source ref: `<estimate_source_ref>`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [completed] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: No prerequisite issue. Preserve #1017 accepted evidence and #1049 raw-transport audit; repair metadata independently before any later reconciliation. Current main baseline; no existing #1048 binding.
2. Review repo inputs and scoped surfaces before editing: Issue #1048 and retained #1017 publication rejection; AGENTS.md; csdlc-v3/AGENTS.md; current native selector; lifecycle/semantic.rs; storage/semantic.rs; application/intent/local.rs and remote.rs; docs/csdlc-v3/INTENT_COMMANDS.md and operator manual.
3. Implement only the bounded deliverables: Typed publication amendment, initial metadata admission, deterministic regression tests, current operator docs, exact-head review and draft PR.
4. Run focused proof gates for acceptance: Typed publication amendment preserves issue/binding identity, validates title/body/base and exact closing linkage, invalidates proof/review/publication as required, and allows fresh review then publication. Malformed initial metadata rejected before durable preparation. Negative fixtures prove no remote dispatch on failed admission, base/head/issue and duplicate-create guards retained. Current manual examples use supported routes.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- publication-metadata-amendment

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- <risks_inline>

## Test Strategy

- Focused Rust semantic amendment/storage/intent command tests and cargo fmt; command-manifest/manual contract checks for touched documentation. Fixtures simulate remote effects and assert zero dispatch on rejected admission. PVF deterministic local tooling/contract proof, small CPU/local Git, no live cloud writes. Actual migration evidence is outside this issue.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Implementation adds typed publication amendment, initial metadata admission, pending-amendment proof rejection, and immutable proof-bound reviews so renewed review at the same HEAD does not overwrite prior evidence. Preserve legacy review receipt dispatch paths. Focused regression and independent review in progress; no remote publication yet.
