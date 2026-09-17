---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "<slug>-execution-plan"
issue: 898
task_id: "issue-0898"
run_id: "issue-0898"
version: "1.0.5"
title: "[v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF"
branch: "codex/898-v0922-codefriend-pdf-renderer"
generated_at: "<timestamp>"
card_status: "ready"
status: "implemented_pending_review"
activation_state: "bound_implemented_pending_review"
plan_revision: 1
initial_pvf_lane: "<initial_pvf_lane>"
planned_pvf_lane: "<planned_pvf_lane>"
planned_pvf_lane_source: "<planned_pvf_lane_source>"
estimate_elapsed_seconds: "<estimate_elapsed_seconds>"
estimate_total_tokens: "<estimate_total_tokens>"
estimate_validation_seconds: "<estimate_validation_seconds>"
issue_goal_token_budget: "<issue_goal_token_budget>"
variance_threshold_percent: "<variance_threshold_percent>"
estimate_confidence: "<estimate_confidence>"
estimate_data_source: "<estimate_data_source>"
estimate_source_ref: "<estimate_source_ref>"
issue_goal_ref: "Bounded #898 child execution under active Sprint 4 #930 goal; no token budget assigned."
sprint_goal_ref: "Active Sprint 4 #930 execution goal"
goal_metrics_rollup_ref: "Sprint 4 #930"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/898"
  - kind: "source_issue_prompt"
    ref: "<source_issue_prompt>"
  - kind: "stp"
    ref: "<stp_card>"
  - kind: "sip"
    ref: "<sip_card>"
scope:
  files:
    - "adl/src/codefriend/publication/pdf.rs; minimal shared publication helpers required for semantic parity; adl/src/codefriend/publication/mod.rs; adl/src/cli/codefriend_cmd.rs; adl/tests/codefriend_render_pdf.rs; docs/validation/pvf_lanes.json; issue-local retained PDF, text-extraction and rendered-page evidence."
  components:
    - "<slug>"
  out_of_scope:
    - "<non_goals_inline>"
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implemented a Rust-owned PDF renderer with pinned printpdf 0.12.8, supplied TrueType font identity, exact governed-source and approval binding, deterministic bounded pagination, semantic parity checks, create-only anchored output, text extraction, every-page raster inspection, retained hashes and six-page visual proof. Current-main reconciliation preserves merged #897 HTML behavior. Fresh exact-head review and publication remain pending."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Issue #896 is satisfied by merged PR #1032 at merge commit 7c09a6526714e562ef8a458652b289ad0e7b63b2. Issue #897 is an independent sibling, not a dependency; serialize only the shared CLI/publication wiring while PR #1038 remains open."
    expected_output: "<sip_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Issue #898; accepted #896 Markdown publication contract and fixtures; governed review, synthesis, remediation and test-plan artifacts; current native C-SDLC authority."
    expected_output: "<stp_card>"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Installed local PDF export, bound manifest, complete semantic parity with the approved Markdown baseline, pinned renderer/version provenance, focused positive and denial tests, actual extracted-text and rendered-page inspection evidence, and truthful review/publication records."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: PDF contains the complete governed claim set and stable finding/evidence/action/test identities; manifest binds exact inputs, renderer/version, approval and output; stale, unapproved, tampered, redaction-failing or external-resource inputs fail closed; empty, clipped, unreadable or renderer-failed output cannot receive a success manifest; actual output text is extracted and every representative page rendered and visually inspected; no public hosting, HTML renderer or unrelated sibling scope is absorbed."
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
    status: "pending_fresh_exact_head_review"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending_publication_ci_merge"
affected_areas:
  - "<slug>"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "<risks_inline>"
test_strategy:
  - "Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf`, strict relevant Clippy, formatting, diff hygiene, installed CLI positive/denial proof, `pdftotext` semantic inspection, `pdftoppm` rendering of every representative page and visual inspection, then independent exact-head review and required CI."
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
notes: "Preserve #896 create-only anchored output and retained-byte validation invariants. Serialize shared CLI/publication wiring with #897 while PR #1038 is open. Keep the PDF implementation and tests cohesive and independently reviewable; do not duplicate a second publication authority model or weaken approval, redaction, path confinement, output freshness or provenance checks."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF`.

Implemented a Rust-owned PDF renderer with pinned printpdf 0.12.8, supplied TrueType font identity, exact governed-source and approval binding, deterministic bounded pagination, semantic parity checks, create-only anchored output, text extraction, every-page raster inspection, retained hashes and six-page visual proof. Current-main reconciliation preserves merged #897 HTML behavior. Fresh exact-head review and publication remain pending.

## PVF Lane Plan

- Initial PVF lane from issue creation: `<initial_pvf_lane>`
- Planned PVF lane for execution: `<planned_pvf_lane>`
- Planning lane source: `<planned_pvf_lane_source>`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `<estimate_elapsed_seconds>`
- Estimated total tokens: `<estimate_total_tokens>`
- Estimated validation seconds: `<estimate_validation_seconds>`
- Issue goal token budget: `<issue_goal_token_budget>`
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
4. [pending_fresh_exact_head_review] Run focused validation and proof gates.
5. [pending_publication_ci_merge] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Issue #896 is satisfied by merged PR #1032 at merge commit 7c09a6526714e562ef8a458652b289ad0e7b63b2. Issue #897 is an independent sibling, not a dependency; serialize only the shared CLI/publication wiring while PR #1038 remains open.
2. Review repo inputs and scoped surfaces before editing: Issue #898; accepted #896 Markdown publication contract and fixtures; governed review, synthesis, remediation and test-plan artifacts; current native C-SDLC authority.
3. Implement only the bounded deliverables: Installed local PDF export, bound manifest, complete semantic parity with the approved Markdown baseline, pinned renderer/version provenance, focused positive and denial tests, actual extracted-text and rendered-page inspection evidence, and truthful review/publication records.
4. Run focused proof gates for acceptance: PDF contains the complete governed claim set and stable finding/evidence/action/test identities; manifest binds exact inputs, renderer/version, approval and output; stale, unapproved, tampered, redaction-failing or external-resource inputs fail closed; empty, clipped, unreadable or renderer-failed output cannot receive a success manifest; actual output text is extracted and every representative page rendered and visually inspected; no public hosting, HTML renderer or unrelated sibling scope is absorbed.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- <slug>

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- <risks_inline>

## Test Strategy

- Run `cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf`, strict relevant Clippy, formatting, diff hygiene, installed CLI positive/denial proof, `pdftotext` semantic inspection, `pdftoppm` rendering of every representative page and visual inspection, then independent exact-head review and required CI.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Preserve #896 create-only anchored output and retained-byte validation invariants. Serialize shared CLI/publication wiring with #897 while PR #1038 is open. Keep the PDF implementation and tests cohesive and independently reviewable; do not duplicate a second publication authority model or weaken approval, redaction, path confinement, output freshness or provenance checks.
