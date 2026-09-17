---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-company-domain-migration-execution-plan"
issue: 1017
task_id: "issue-1017"
run_id: "issue-1017"
version: "v0.92.2"
title: "[v0.92.2][corporate][AWS] Migrate v-*.ai domains from personal AWS to the company account"
branch: "codex/1017-v0922-company-domain-migration"
generated_at: "2026-09-16T23:15:26.654477+00:00"
card_status: "ready"
status: "executed"
activation_state: "ready_for_binding"
plan_revision: 1
initial_pvf_lane: "docs_only"
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
issue_goal_ref: "Create issue-bound execution goal before implementation or cloud discovery."
sprint_goal_ref: "<sprint_goal_ref>"
goal_metrics_rollup_ref: "<goal_metrics_rollup_ref>"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1017"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1017"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - ".csdlc/evidence/1017/ sanitized inventory, sequence, verification summary and migration disposition; .csdlc/issues/1017/cards and generated projections; private execution backup/account evidence outside Git with restricted access, referenced only by safe logical identifiers."
  components:
    - "v0922-company-domain-migration"
  out_of_scope:
    - "No unrelated domains, website redesign, unrelated hosting/email changes, automatic transfer rollback, customer publication or early source deletion. No AWS calls or migration during preparation."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "1. Discover and confirm exact domains and account/profile authorization. 2. Inventory registrar, renewals, zones, NS, DNSSEC and dependencies; verify current official procedures. 3. Back up configuration, capture service baseline, prepare destination access/contacts/renewals and separate recovery plans. 4. After inventory approval, transfer registrations one domain at a time and verify control; separately migrate DNS where needed without losing records. 5. Observe baseline parity through relevant TTLs, independently review evidence, then retire only verified obsolete resources and record every completed/blocked disposition."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Owner confirmed default source and exact three-domain inventory. Source/destination identities verified distinct. Private backups retained; all transfers succeeded. Operator-approved retained-source-DNS exception allows registration delivery to proceed to PR."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Live issue #1017; AGENTS.md; docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md; agent-logic/agent-logic.ai issues #31 and #32 as context only; owner-confirmed inventory and live registrar/account evidence collected during execution; current official AWS/registrar procedures verified before changes."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Approved inventory and per-domain migration/recovery sequence; private recoverable backups; verified registration/renewal ownership; preserved DNS/services; sanitized observation and cleanup dispositions."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: Every approved domain has a completed or blocked disposition. Verify company registration control and renewals; company DNS authority or approved external exception; full record preservation; baseline and post-change DNS, TLS, redirects and mail checks or explicit not-applicable; TTL-based observation; recoverable backups and separate transfer/DNS recovery plans. Keep issue open for unresolved migrations. Retain sanitized per-domain evidence without private account/contact/billing/transfer data."
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
  - "v0922-company-domain-migration"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "<risks_inline>"
test_strategy:
  - "Preparation: native validate and doctor check all six active-template cards. The declared semantic_card_projections Cargo check proves tooling structure only, not domain migration. Execution: independently verify each registration and renewal owner, authoritative NS/DNSSEC, complete zone records, DNS/TLS/HTTP redirects/mail baseline and post-cutover behavior; observe for relevant TTLs, record N/A with reasons, rollback on defined failures. Required private external observations are nondeterministic and separate from docs_only local evidence hygiene; git diff --check and redaction review for sanitized tracked records. No green card test may substitute for cloud/service proof."
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
notes: "Independent evidence/privacy review confirmed the migration. Historical handoff marked superseded; company contacts and standard registrar renewal-notification path documented. All registration steps complete; source DNS retained under approved exception pending v0.93 sites. Exact-head review/publication remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][corporate][AWS] Migrate v-*.ai domains from personal AWS to the company account`.

1. Discover and confirm exact domains and account/profile authorization. 2. Inventory registrar, renewals, zones, NS, DNSSEC and dependencies; verify current official procedures. 3. Back up configuration, capture service baseline, prepare destination access/contacts/renewals and separate recovery plans. 4. After inventory approval, transfer registrations one domain at a time and verify control; separately migrate DNS where needed without losing records. 5. Observe baseline parity through relevant TTLs, independently review evidence, then retire only verified obsolete resources and record every completed/blocked disposition.

## PVF Lane Plan

- Initial PVF lane from issue creation: `docs_only`
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

1. Confirm dependency readiness and starting state: Owner confirmed default source and exact three-domain inventory. Source/destination identities verified distinct. Private backups retained; all transfers succeeded. Operator-approved retained-source-DNS exception allows registration delivery to proceed to PR.
2. Review repo inputs and scoped surfaces before editing: Live issue #1017; AGENTS.md; docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md; agent-logic/agent-logic.ai issues #31 and #32 as context only; owner-confirmed inventory and live registrar/account evidence collected during execution; current official AWS/registrar procedures verified before changes.
3. Implement only the bounded deliverables: Approved inventory and per-domain migration/recovery sequence; private recoverable backups; verified registration/renewal ownership; preserved DNS/services; sanitized observation and cleanup dispositions.
4. Run focused proof gates for acceptance: Every approved domain has a completed or blocked disposition. Verify company registration control and renewals; company DNS authority or approved external exception; full record preservation; baseline and post-change DNS, TLS, redirects and mail checks or explicit not-applicable; TTL-based observation; recoverable backups and separate transfer/DNS recovery plans. Keep issue open for unresolved migrations. Retain sanitized per-domain evidence without private account/contact/billing/transfer data.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-company-domain-migration

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- <risks_inline>

## Test Strategy

- Preparation: native validate and doctor check all six active-template cards. The declared semantic_card_projections Cargo check proves tooling structure only, not domain migration. Execution: independently verify each registration and renewal owner, authoritative NS/DNSSEC, complete zone records, DNS/TLS/HTTP redirects/mail baseline and post-cutover behavior; observe for relevant TTLs, record N/A with reasons, rollback on defined failures. Required private external observations are nondeterministic and separate from docs_only local evidence hygiene; git diff --check and redaction review for sanitized tracked records. No green card test may substitute for cloud/service proof.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Independent evidence/privacy review confirmed the migration. Historical handoff marked superseded; company contacts and standard registrar renewal-notification path documented. All registration steps complete; source DNS retained under approved exception pending v0.93 sites. Exact-head review/publication remain pending.
