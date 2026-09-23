---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "codefriend-result-integrity-execution-plan"
issue: 1162
task_id: "issue-1162"
run_id: "issue-1162"
version: "0.92.2"
title: "[v0.92.2][TAIL-06][P1] Repair CodeFriend result integrity and website interoperability"
branch: "codex/1162-codefriend-result-integrity"
generated_at: "2026-09-23T17:29:32.826544+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "Group A recovery and transport contracts; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "14400"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "1800"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "engineering estimate for eight bounded findings and focused crash regression suite; not a measured result"
estimate_source_ref: "issue source and #919 finding inventory"
issue_goal_ref: "issue-1162"
sprint_goal_ref: "issue-921"
goal_metrics_rollup_ref: "not_collected"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1162"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/1162"
  - kind: "stp"
    ref: ".csdlc/issues/1162/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/1162/cards/sip.md"
scope:
  files:
    - "adl/src/codefriend/publication/{markdown.rs,html.rs,pdf.rs,relay.rs}; adl/src/codefriend/operator/mod.rs; focused adl/tests/codefriend_* regressions and coupled PVF records; docs/codefriend/PDF_EXPORT.md; CodeFriend website app/review-assessments.mjs, app/http.mjs, deploy workflow, tests, actual native-v4 fixtures, and PVF inventory."
  components:
    - "codefriend-result-integrity"
  out_of_scope:
    - "No changes to frozen #918/#919 artifacts, per-finding issues, unrelated runtime/provider work, merge, deployment, release, hosted-provider spend, or shared owner-binary replacement."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Both repository components are implemented and locally validated. The website component is independently reviewed and published as draft PR #20. The ADL component includes the remediated PDF and retry-concurrency findings and now awaits final exact-head review, native proof, and draft publication. No merge or deployment is authorized or claimed."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Aggregate issue #1162 is a Group B child of #921. #919 and #918 remain independently owned and read-only. The website fixture step depends on the repaired native producer; other disjoint repairs may proceed in parallel."
    expected_output: ".csdlc/issues/1162/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: AGENTS.md; active prompt registry and 1.0.5 schemas; #1162; #921; immutable #919 finding artifacts; frozen ADL candidate 5c4a6149771c637f3c805985b86231077965eab4; frozen website candidate a45e339c13b24716edbd3fadf29dddff36ffe02e; current source in both repositories."
    expected_output: ".csdlc/issues/1162/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: ADL publication, operator-attempt, PDF verification, focused regression, PVF, and proof-wording repairs; a separate CodeFriend website component PR for v4 compatibility, asynchronous authorization, pinned workflow actions, and actual native-emitted v4 fixtures; one retained finding-to-fix-to-test disposition map for all seven findings."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: CODE-001: multiline, CRLF, and tab-bearing admitted excerpts preserve visible semantics in Markdown, HTML, and extracted PDF. CODE-002: retry refuses an incomplete active provider attempt and cancel/retry concurrency preserves attempt identity. SEC-003: a resealed substituted PDF fails stage verification despite self-consistent hashes and metadata. INTEGRATION-001: the website accepts authentic native v4 results while retaining v2/v3 compatibility and rejecting mixed, stale, altered, false-complete, and future-version inputs. SEC-004: protected mutation and result/download paths re-authorize after awaited work. DEP-001: privileged deployment actions use exact commit SHAs and deploy-only OIDC scope. DEMOS-001: Cargo-built proof is labeled source-built, with installed proof claimed only when actually exercised. Every finding maps to an exact fix and proving test; independent exact-head review and applicable CI are required."
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
    status: "in_progress"
affected_areas:
  - "codefriend-result-integrity"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Cross-repository compatibility can falsely pass with hand-authored fixtures; fixtures must be emitted by the actual native v4 producer. PDF validation must inspect independently reconstructed semantics and reject active/external content. Async reauthorization must retain controlled identity and fail closed after revocation."
test_strategy:
  - "cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_html; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_review; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_agent_publication. Website component runs focused Node regressions and full npm test outside native Cargo proof. New tests declare release/contract/tooling lane as applicable, regression proof role, deterministic local fixtures, local CPU/disk resources, and required gate status. No paid provider, deployment, or live user action."
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
notes: "Cross-repository compatibility can falsely pass with hand-authored fixtures; fixtures must be emitted by the actual native v4 producer. PDF validation must inspect independently reconstructed semantics and reject active/external content. Async reauthorization must retain controlled identity and fail closed after revocation."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][TAIL-06][P1] Repair CodeFriend result integrity and website interoperability`.

Both repository components are implemented and locally validated. The website component is independently reviewed and published as draft PR #20. The ADL component includes the remediated PDF and retry-concurrency findings and now awaits final exact-head review, native proof, and draft publication. No merge or deployment is authorized or claimed.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `Group A recovery and transport contracts; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `14400`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `1800`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `engineering estimate for eight bounded findings and focused crash regression suite; not a measured result`
- Estimate source ref: `issue source and #919 finding inventory`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [in_progress] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Aggregate issue #1162 is a Group B child of #921. #919 and #918 remain independently owned and read-only. The website fixture step depends on the repaired native producer; other disjoint repairs may proceed in parallel.
2. Review repo inputs and scoped surfaces before editing: AGENTS.md; active prompt registry and 1.0.5 schemas; #1162; #921; immutable #919 finding artifacts; frozen ADL candidate 5c4a6149771c637f3c805985b86231077965eab4; frozen website candidate a45e339c13b24716edbd3fadf29dddff36ffe02e; current source in both repositories.
3. Implement only the bounded deliverables: ADL publication, operator-attempt, PDF verification, focused regression, PVF, and proof-wording repairs; a separate CodeFriend website component PR for v4 compatibility, asynchronous authorization, pinned workflow actions, and actual native-emitted v4 fixtures; one retained finding-to-fix-to-test disposition map for all seven findings.
4. Run focused proof gates for acceptance: CODE-001: multiline, CRLF, and tab-bearing admitted excerpts preserve visible semantics in Markdown, HTML, and extracted PDF. CODE-002: retry refuses an incomplete active provider attempt and cancel/retry concurrency preserves attempt identity. SEC-003: a resealed substituted PDF fails stage verification despite self-consistent hashes and metadata. INTEGRATION-001: the website accepts authentic native v4 results while retaining v2/v3 compatibility and rejecting mixed, stale, altered, false-complete, and future-version inputs. SEC-004: protected mutation and result/download paths re-authorize after awaited work. DEP-001: privileged deployment actions use exact commit SHAs and deploy-only OIDC scope. DEMOS-001: Cargo-built proof is labeled source-built, with installed proof claimed only when actually exercised. Every finding maps to an exact fix and proving test; independent exact-head review and applicable CI are required.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- codefriend-result-integrity

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Cross-repository compatibility can falsely pass with hand-authored fixtures; fixtures must be emitted by the actual native v4 producer. PDF validation must inspect independently reconstructed semantics and reject active/external content. Async reauthorization must retain controlled identity and fail closed after revocation.

## Test Strategy

- cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_md; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_html; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_review; cargo test --locked --manifest-path adl/Cargo.toml --test codefriend_agent_publication. Website component runs focused Node regressions and full npm test outside native Cargo proof. New tests declare release/contract/tooling lane as applicable, regression proof role, deterministic local fixtures, local CPU/disk resources, and required gate status. No paid provider, deployment, or live user action.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Cross-repository compatibility can falsely pass with hand-authored fixtures; fixtures must be emitted by the actual native v4 producer. PDF validation must inspect independently reconstructed semantics and reject active/external content. Async reauthorization must retain controlled identity and fail closed after revocation.
