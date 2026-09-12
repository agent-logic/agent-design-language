---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "879-github-ingestion-execution-plan"
issue: 879
task_id: "issue-0879"
run_id: "issue-0879"
version: "v0.92.2"
title: "[v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet"
branch: "codex/879-github-ingestion"
generated_at: "2026-09-11T23:55:26.355672+00:00"
card_status: "ready"
status: "prepared"
activation_state: "activated"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "Issue #879 acceptance and PVF clauses"
estimate_elapsed_seconds: "Not estimated; record actual child execution metrics."
estimate_total_tokens: "Not estimated; record actual child execution metrics."
estimate_validation_seconds: "Not estimated; record actual child execution metrics."
issue_goal_token_budget: "not specified"
variance_threshold_percent: "Not estimated; record actual child execution metrics."
estimate_confidence: "unestimated"
estimate_data_source: "No child execution baseline measured"
estimate_source_ref: "source issue validation requirements"
issue_goal_ref: "Issue #879 execution goal active: deliver reviewed passing PR without merge or cleanup."
sprint_goal_ref: "Current v0.92.2 Sprint2 execution-readiness preparation goal"
goal_metrics_rollup_ref: "Not measured: no child execution"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/879"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/879"
  - kind: "stp"
    ref: ".csdlc/issues/879/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/879/cards/sip.md"
scope:
  files:
    - "Use the shared product route selected by WP-01 and the predecessor's new `adl/src/codefriend/ingestion/mod.rs`/`local.rs` packet contract. Own proposed `adl/src/codefriend/ingestion/github.rs`, narrow registration in the selected `adl/src/cli/codefriend_cmd.rs`, and focused `adl/tests/codefriend_github_ingestion.rs`. These are new intended paths, resolved against CF-ADAPTER at execution; do not invent another packet schema or credential resolver. Read the adopted contract and portable-adapter feature. Native C-SDLC GitHub lifecycle authority is separate from this product's read-only repository acquisition."
  components:
    - "879-github-ingestion"
  out_of_scope:
    - "PVF: deterministic transport/installed-command integration plus optional separately authorized read-only external corroboration; role: exact pinning, packet parity and transport/credential negatives; resources: bounded local CPU/disk/loopback transport; gate: required Beta ingestion. Record external dependence/nondeterminism honestly if a live check is used. Stop for missing required input, failed proof, contract conflict, credential capture, host-bound packet or unverified revision. Exclude GitHub lifecycle writes, CI adapter, review implementation, broad connectors and scaffold-only completion."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Bound implementation complete;8 GitHub tests,10 inherited tests and8 installed candidate tests pass. Exact-head review/publication/CI remain required."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Accepted merged output required from #878. No sprint-wide barrier or asynchronous closeout dependency."
    expected_output: ".csdlc/issues/879/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Full canonical issue https://github.com/agent-logic/agent-design-language/issues/879; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json."
    expected_output: ".csdlc/issues/879/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: The GitHub ingestion entrypoint resolves a repository revision or pull request to an exact immutable commit and emits the same consumable portable packet as local ingestion. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); WP-01/#864 owns shared opening selections. It does not create or modify GitHub issues/PRs. Include production proof, failure handling and operator documentation required by the source issue."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions. 2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts. 3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion. 4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized. 5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked."
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
    status: "pending"
affected_areas:
  - "879-github-ingestion"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Prerequisite output or exact implementation test targets may change; revalidate before child execution."
test_strategy:
  - "1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions. 2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts. 3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion. 4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized. 5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked. PVF: deterministic transport/installed-command integration plus optional separately authorized read-only external corroboration; role: exact pinning, packet parity and transport/credential negatives; resources: bounded local CPU/disk/loopback transport; gate: required Beta ingestion. Record external dependence/nondeterminism honestly if a live check is used. Stop for missing required input, failed proof, contract conflict, credential capture, host-bound packet or unverified revision. Exclude GitHub lifecycle writes, CI adapter, review implementation, broad connectors and scaffold-only completion."
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
notes: "Implementation and focused local/installed proof complete; independent exact-head review, native publication and required CI pending. No live GitHub or provider calls, merge or closeout claimed."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-ADAPTER-GITHUB] Ingest a pinned GitHub revision or PR into a repository packet`.

Bound implementation complete;8 GitHub tests,10 inherited tests and8 installed candidate tests pass. Exact-head review/publication/CI remain required.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `Issue #879 acceptance and PVF clauses`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `Not estimated; record actual child execution metrics.`
- Estimated total tokens: `Not estimated; record actual child execution metrics.`
- Estimated validation seconds: `Not estimated; record actual child execution metrics.`
- Issue goal token budget: `not specified`
- Variance threshold percent: `Not estimated; record actual child execution metrics.`
- Estimate confidence: `unestimated`
- Estimate data source: `No child execution baseline measured`
- Estimate source ref: `source issue validation requirements`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Accepted merged output required from #878. No sprint-wide barrier or asynchronous closeout dependency.
2. Review repo inputs and scoped surfaces before editing: Full canonical issue https://github.com/agent-logic/agent-design-language/issues/879; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.
3. Implement only the bounded deliverables: The GitHub ingestion entrypoint resolves a repository revision or pull request to an exact immutable commit and emits the same consumable portable packet as local ingestion. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); WP-01/#864 owns shared opening selections. It does not create or modify GitHub issues/PRs. Include production proof, failure handling and operator documentation required by the source issue.
4. Run focused proof gates for acceptance: 1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions. 2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts. 3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion. 4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized. 5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- 879-github-ingestion

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Prerequisite output or exact implementation test targets may change; revalidate before child execution.

## Test Strategy

- 1. For a pinned commit and a PR input, resolve exact repository identity and commit, fetch bounded source through the actual read-only acquisition path, and emit/read the production packet. A moving PR reference is pinned before reading; record original ref and resolved commit and detect disagreement/changed head rather than mixing revisions. 2. For the same revision/scope/content, compare packet semantics and evidence-object identities to local ingestion. Preserve provenance differences explicitly; credentials and machine-local cache paths cannot affect object identity or enter artifacts. 3. Execute controlled transport failures, not-found/forbidden/missing content, wrong repository/revision, rate limit/pagination or truncation and incomplete input. Fail or report explicit partial state with omissions; never silently mark complete. Honor bounded source limits and path/redaction checks inherited from local ingestion. 4. Secret references use the approved resolver and are never captured in URL/log/packet. Hostile repository content cannot authorize tools or GitHub writes. Test credential-shaped values and traversal/symlink inputs; retained errors are bounded and sanitized. 5. Retain nonzero actual transport/entrypoint execution and local parity. Deterministic fixtures use a controlled Git-compatible HTTP transport; any live GitHub readback is separately recorded and cannot be replaced by a hand-authored packet. No provider is invoked. PVF: deterministic transport/installed-command integration plus optional separately authorized read-only external corroboration; role: exact pinning, packet parity and transport/credential negatives; resources: bounded local CPU/disk/loopback transport; gate: required Beta ingestion. Record external dependence/nondeterminism honestly if a live check is used. Stop for missing required input, failed proof, contract conflict, credential capture, host-bound packet or unverified revision. Exclude GitHub lifecycle writes, CI adapter, review implementation, broad connectors and scaffold-only completion.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Implementation and focused local/installed proof complete; independent exact-head review, native publication and required CI pending. No live GitHub or provider calls, merge or closeout claimed.
