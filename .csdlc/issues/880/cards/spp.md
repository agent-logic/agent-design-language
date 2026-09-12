---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "880-ci-ingestion-execution-plan"
issue: 880
task_id: "issue-0880"
run_id: "issue-0880"
version: "v0.92.2"
title: "[v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet"
branch: "codex/880-ci-ingestion"
generated_at: "2026-09-11T23:55:27.199943+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "not_activated"
plan_revision: 1
initial_pvf_lane: "runtime"
planned_pvf_lane: "runtime"
planned_pvf_lane_source: "Issue #880 acceptance and PVF clauses"
estimate_elapsed_seconds: "Not estimated; record actual child execution metrics."
estimate_total_tokens: "Not estimated; record actual child execution metrics."
estimate_validation_seconds: "Not estimated; record actual child execution metrics."
issue_goal_token_budget: "not specified"
variance_threshold_percent: "Not estimated; record actual child execution metrics."
estimate_confidence: "unestimated"
estimate_data_source: "No child execution baseline measured"
estimate_source_ref: "source issue validation requirements"
issue_goal_ref: "Create a separate issue-bound implementation goal for #880 before source edits."
sprint_goal_ref: "Current v0.92.2 Sprint2 execution-readiness preparation goal"
goal_metrics_rollup_ref: "Not measured: no child execution"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/880"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/880"
  - kind: "stp"
    ref: ".csdlc/issues/880/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/880/cards/sip.md"
scope:
  files:
    - "Reuse the predecessor's selected installed command and `adl/src/codefriend/ingestion/` contract. Own proposed `adl/src/codefriend/ingestion/ci.rs`, narrow CLI registration, focused `adl/tests/codefriend_ci_ingestion.rs` and one named `.github/workflows/` smoke job chosen in the issue plan. The workflow must call the installed product path; it is not a competing ingestion implementation. Read the portable-adapter feature and adopted contracts before execution."
  components:
    - "880-ci-ingestion"
  out_of_scope:
    - "PVF: deterministic local contract/installed integration followed by required CI smoke; role: declared revision, packet parity and partial/credential negatives; resources: bounded runner CPU/disk, isolated fixture repository, no metered inference; gate: required Beta CI input route. Retain exact command and nonzero outcome in CI independently from local proof. Stop on missing input, failed required proof, contract conflict, credential capture, host-bound packet or unexecuted CI gate. Exclude CF-GOV-CI, GitHub API adapter, review/features, broad connectors and schema/scaffold-only closure."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implement CI acquisition by reusing local immutable Git capture and AdmissionInput readback. Keep allowlisted run metadata in a separate packet-bound receipt; explicit source and candidate revisions remain distinct. Add independent ingest/ci CLI dispatch. Required codefriend-ci-acquisition hosted job installs the candidate, consumes a bounded fixture, uploads proof with error-on-missing and validates artifact digest. Wire selection and success/skip into adl-ci. Focused negative/parity tests precede independent exact-head review and native publication; hosted proof remains pending until actual run."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Accepted merged output required from #878. No sprint-wide barrier or asynchronous closeout dependency."
    expected_output: ".csdlc/issues/880/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Full canonical issue https://github.com/agent-logic/agent-design-language/issues/880; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json."
    expected_output: ".csdlc/issues/880/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: The CI ingestion entrypoint consumes declared checkout/revision/scope inputs and emits a usable common packet with truthful partial-input state. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); shared command/path decisions belong to WP-01/#864. This task provides CI input acquisition, not architecture fitness gating (CF-GOV-CI). Include production proof, failure handling and operator documentation required by the source issue."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope. 2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets. 3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials. 4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data. 5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success."
    expected_output: "validation evidence recorded in VPP/SOR"
    allowed_mode: "execution_after_approval"
  - id: "step-5"
    description: "Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges."
    expected_output: "reviewed SRP and truthful VPP/SOR"
    allowed_mode: "execution_after_approval"
codex_plan:
  - step: "Confirm dependencies and starting state from the source issue prompt."
    status: "pending"
  - step: "Inspect repo inputs and target surfaces before editing."
    status: "pending"
  - step: "Implement the bounded deliverables only."
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "pending"
affected_areas:
  - "880-ci-ingestion"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Prerequisite output or exact implementation test targets may change; revalidate before child execution."
test_strategy:
  - "1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope. 2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets. 3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials. 4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data. 5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success. PVF: deterministic local contract/installed integration followed by required CI smoke; role: declared revision, packet parity and partial/credential negatives; resources: bounded runner CPU/disk, isolated fixture repository, no metered inference; gate: required Beta CI input route. Retain exact command and nonzero outcome in CI independently from local proof. Stop on missing input, failed required proof, contract conflict, credential capture, host-bound packet or unexecuted CI gate. Exclude CF-GOV-CI, GitHub API adapter, review/features, broad connectors and schema/scaffold-only closure."
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
notes: "Implementation and local proof complete; independent source review passed. Required hosted proof and publication remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet`.

Implement CI acquisition by reusing local immutable Git capture and AdmissionInput readback. Keep allowlisted run metadata in a separate packet-bound receipt; explicit source and candidate revisions remain distinct. Add independent ingest/ci CLI dispatch. Required codefriend-ci-acquisition hosted job installs the candidate, consumes a bounded fixture, uploads proof with error-on-missing and validates artifact digest. Wire selection and success/skip into adl-ci. Focused negative/parity tests precede independent exact-head review and native publication; hosted proof remains pending until actual run.

## PVF Lane Plan

- Initial PVF lane from issue creation: `runtime`
- Planned PVF lane for execution: `runtime`
- Planning lane source: `Issue #880 acceptance and PVF clauses`
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

1. [pending] Confirm dependencies and starting state from the source issue prompt.
2. [pending] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: Accepted merged output required from #878. No sprint-wide barrier or asynchronous closeout dependency.
2. Review repo inputs and scoped surfaces before editing: Full canonical issue https://github.com/agent-logic/agent-design-language/issues/880; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.
3. Implement only the bounded deliverables: The CI ingestion entrypoint consumes declared checkout/revision/scope inputs and emits a usable common packet with truthful partial-input state. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); shared command/path decisions belong to WP-01/#864. This task provides CI input acquisition, not architecture fitness gating (CF-GOV-CI). Include production proof, failure handling and operator documentation required by the source issue.
4. Run focused proof gates for acceptance: 1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope. 2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets. 3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials. 4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data. 5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- 880-ci-ingestion

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Prerequisite output or exact implementation test targets may change; revalidate before child execution.

## Test Strategy

- 1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope. 2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets. 3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials. 4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data. 5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success. PVF: deterministic local contract/installed integration followed by required CI smoke; role: declared revision, packet parity and partial/credential negatives; resources: bounded runner CPU/disk, isolated fixture repository, no metered inference; gate: required Beta CI input route. Retain exact command and nonzero outcome in CI independently from local proof. Stop on missing input, failed required proof, contract conflict, credential capture, host-bound packet or unexecuted CI gate. Exclude CF-GOV-CI, GitHub API adapter, review/features, broad connectors and schema/scaffold-only closure.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Implementation and local proof complete; independent source review passed. Required hosted proof and publication remain pending.
