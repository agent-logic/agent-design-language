---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-codefriend-pdf-renderer-execution-plan"
issue: 898
task_id: "issue-0898"
run_id: "issue-0898"
version: "0.92.2"
title: "[v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF"
branch: "codex/898-v0922-codefriend-pdf-renderer"
generated_at: "2026-09-12T00:10:10.730683+00:00"
card_status: "ready"
status: "implemented_pending_review"
activation_state: "bound_implemented_pending_review"
plan_revision: 1
initial_pvf_lane: "owner_binary"
planned_pvf_lane: "owner_binary"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/898 validation contract; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "10800"
estimate_total_tokens: "65000"
estimate_validation_seconds: "3600"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "Conservative design-time estimate for one complete renderer and proof; reestimate after predecessor integration, not an operator budget"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/898"
issue_goal_ref: "Bounded #898 child execution under active Sprint 4 #930 goal; no token budget assigned."
sprint_goal_ref: "Active Sprint 4 #930 execution goal"
goal_metrics_rollup_ref: "Sprint 4 #930"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/898"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/898"
  - kind: "stp"
    ref: ".csdlc/issues/898/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/898/cards/sip.md"
scope:
  files:
    - "Implement `export pdf` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/pdf.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_render_pdf.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command."
  components:
    - "v0922-codefriend-pdf-renderer"
  out_of_scope:
    - ""
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Implemented a Rust-owned PDF renderer with pinned printpdf 0.12.8, supplied TrueType font identity, exact governed-source and approval binding, deterministic bounded pagination, semantic parity checks, create-only anchored output, text extraction, every-page raster inspection, retained hashes and five-page visual proof. Current-main reconciliation preserves merged #897 HTML behavior. Fresh exact-head review and publication remain pending."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: Wait for accepted merged output #896 CF-RENDER-MD. HTML and PDF are independent siblings; no cross-renderer dependency. Dependencies are OPEN in fresh snapshots; accepted implementation output is unavailable. Refresh exact predecessor source and resolve shared CLI/path ownership before scheduling or binding. #926 manages all eleven sprints only; it is not a child execution prerequisite."
    expected_output: ".csdlc/issues/898/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF ## One complete result The PDF renderer consumes the governed publication contract and produces a readable visually inspected PDF with intact citations and the same claims as Markdown. Dependencies: CF-RENDER-MD. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate. ## Production ownership Implement `export pdf` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/pdf.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_render_pdf.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command. ## Acceptance and executed evidence 1. Render actual governed inputs as a complete PDF and manifest with the same claim/finding set, scope, citations, uncertainty, disagreements and actionable plans as approved Markdown. Pin the chosen rendering engine/version in the implementation plan and manifest; keep Rust entrypoint ownership even if a bounded renderer subprocess is required. 2. Extract text to check semantic parity and render every representative page for visual inspection. Exercise long findings, long URLs/code, tables, page breaks and non-ASCII text; reject clipped/overlapping/missing content, broken citations and unreadable output. A zero-byte or merely parseable PDF is insufficient. 3. Enforce approval freshness, exact rendering/target identity, immutable provenance and redaction before export. Reject unapproved/tampered input, secret leakage, claim drift and unsafe external-resource inclusion; renderer failures leave explicit failed/withheld state, not a success manifest. 4. Open the emitted PDF and retain visual review plus content/manifest hashes. Produce local output only; no public hosting or submission is implied. Schema, mock or snapshot-only completion is forbidden. ## Shared execution boundary Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. ## PVF and completion Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. ## Inherited obligation ledger acceptance: `pdf_rendered_and_inspected`, `markdown_claim_parity`, `manifest_binding`, `output_parity`. pvf: `pdf_rendered_and_inspected`, `markdown_claim_parity`, `manifest_binding`, `clipped_content_rejected`, `unapproved_render_denied`, `renderer_snapshot`, `redaction_recheck`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `renderer_claim_drift`, `clipped_report_content`, `redaction_check_failed`, `approval_missing_or_stale`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 4; this grouping adds no execution gate. Execution prerequisite: #896 (CF-RENDER-MD); accepted output is required before dependent execution. Reviewed creation source: `54e5d8e100f39c644ca0aa03e3985ce66beb529e`. This issue records a complete task; creation does not claim execution or acceptance."
    expected_output: ".csdlc/issues/898/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Implement `export pdf` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/pdf.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_render_pdf.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command. The PDF renderer consumes the governed publication contract and produces a readable visually inspected PDF with intact citations and the same claims as Markdown. Dependencies: CF-RENDER-MD. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Render actual governed inputs as a complete PDF and manifest with the same claim/finding set, scope, citations, uncertainty, disagreements and actionable plans as approved Markdown. Pin the chosen rendering engine/version in the implementation plan and manifest; keep Rust entrypoint ownership even if a bounded renderer subprocess is required. 2. Extract text to check semantic parity and render every representative page for visual inspection. Exercise long findings, long URLs/code, tables, page breaks and non-ASCII text; reject clipped/overlapping/missing content, broken citations and unreadable output. A zero-byte or merely parseable PDF is insufficient. 3. Enforce approval freshness, exact rendering/target identity, immutable provenance and redaction before export. Reject unapproved/tampered input, secret leakage, claim drift and unsafe external-resource inclusion; renderer failures leave explicit failed/withheld state, not a success manifest. 4. Open the emitted PDF and retain visual review plus content/manifest hashes. Produce local output only; no public hosting or submission is implied. Schema, mock or snapshot-only completion is forbidden."
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
  - "v0922-codefriend-pdf-renderer"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Wait for accepted merged output #896 CF-RENDER-MD. HTML and PDF are independent siblings; no cross-renderer dependency. Dependencies are OPEN in fresh snapshots; accepted implementation output is unavailable. Refresh exact predecessor source and resolve shared CLI/path ownership before scheduling or binding. #926 manages all eleven sprints only; it is not a child execution prerequisite."
test_strategy:
  - "Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. Concrete planned commands after implementation: cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check. Execute the installed export against passing and denied fixtures; required nonzero behavior and semantic parity cannot be replaced by a green generic suite. Pin the PDF engine, extract actual PDF text and render representative pages for visual inspection; this evidence is required and not inferred from serialization."
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
notes: "Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. Wait for accepted merged output #896 CF-RENDER-MD. HTML and PDF are independent siblings; no cross-renderer dependency. Dependencies are OPEN in fresh snapshots; accepted implementation output is unavailable. Refresh exact predecessor source and resolve shared CLI/path ownership before scheduling or binding. #926 manages all eleven sprints only; it is not a child execution prerequisite. Preparation only: no implementation, proof success, implementation review, publication or live activation. Proposed renderer/CodeFriend modules are absent in current main; re-resolve the accepted predecessor contracts before editing. Root main remains inspection-only. Shared CLI/mod/lib/usage owners must coordinate changes."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF`.

Implemented a Rust-owned PDF renderer with pinned printpdf 0.12.8, supplied TrueType font identity, exact governed-source and approval binding, deterministic bounded pagination, semantic parity checks, create-only anchored output, text extraction, every-page raster inspection, retained hashes and five-page visual proof. Current-main reconciliation preserves merged #897 HTML behavior. Fresh exact-head review and publication remain pending.

## PVF Lane Plan

- Initial PVF lane from issue creation: `owner_binary`
- Planned PVF lane for execution: `owner_binary`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/898 validation contract; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `10800`
- Estimated total tokens: `65000`
- Estimated validation seconds: `3600`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `Conservative design-time estimate for one complete renderer and proof; reestimate after predecessor integration, not an operator budget`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/898`
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

1. Confirm dependency readiness and starting state: Wait for accepted merged output #896 CF-RENDER-MD. HTML and PDF are independent siblings; no cross-renderer dependency. Dependencies are OPEN in fresh snapshots; accepted implementation output is unavailable. Refresh exact predecessor source and resolve shared CLI/path ownership before scheduling or binding. #926 manages all eleven sprints only; it is not a child execution prerequisite.
2. Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][CF-RENDER-PDF] Render an approved review as a verified PDF ## One complete result The PDF renderer consumes the governed publication contract and produces a readable visually inspected PDF with intact citations and the same claims as Markdown. Dependencies: CF-RENDER-MD. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate. ## Production ownership Implement `export pdf` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/pdf.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_render_pdf.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command. ## Acceptance and executed evidence 1. Render actual governed inputs as a complete PDF and manifest with the same claim/finding set, scope, citations, uncertainty, disagreements and actionable plans as approved Markdown. Pin the chosen rendering engine/version in the implementation plan and manifest; keep Rust entrypoint ownership even if a bounded renderer subprocess is required. 2. Extract text to check semantic parity and render every representative page for visual inspection. Exercise long findings, long URLs/code, tables, page breaks and non-ASCII text; reject clipped/overlapping/missing content, broken citations and unreadable output. A zero-byte or merely parseable PDF is insufficient. 3. Enforce approval freshness, exact rendering/target identity, immutable provenance and redaction before export. Reject unapproved/tampered input, secret leakage, claim drift and unsafe external-resource inclusion; renderer failures leave explicit failed/withheld state, not a success manifest. 4. Open the emitted PDF and retain visual review plus content/manifest hashes. Produce local output only; no public hosting or submission is implied. Schema, mock or snapshot-only completion is forbidden. ## Shared execution boundary Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. ## PVF and completion Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. ## Inherited obligation ledger acceptance: `pdf_rendered_and_inspected`, `markdown_claim_parity`, `manifest_binding`, `output_parity`. pvf: `pdf_rendered_and_inspected`, `markdown_claim_parity`, `manifest_binding`, `clipped_content_rejected`, `unapproved_render_denied`, `renderer_snapshot`, `redaction_recheck`. stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `renderer_claim_drift`, `clipped_report_content`, `redaction_check_failed`, `approval_missing_or_stale`. non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `public_customer_scale`. Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`. Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`. ## Canonical execution links Planning owner: #864. Creation/review batch: 4; this grouping adds no execution gate. Execution prerequisite: #896 (CF-RENDER-MD); accepted output is required before dependent execution. Reviewed creation source: `54e5d8e100f39c644ca0aa03e3985ce66beb529e`. This issue records a complete task; creation does not claim execution or acceptance.
3. Implement only the bounded deliverables: Implement `export pdf` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: publication/pdf.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_render_pdf.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command. The PDF renderer consumes the governed publication contract and produces a readable visually inspected PDF with intact citations and the same claims as Markdown. Dependencies: CF-RENDER-MD. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.
4. Run focused proof gates for acceptance: 1. Render actual governed inputs as a complete PDF and manifest with the same claim/finding set, scope, citations, uncertainty, disagreements and actionable plans as approved Markdown. Pin the chosen rendering engine/version in the implementation plan and manifest; keep Rust entrypoint ownership even if a bounded renderer subprocess is required. 2. Extract text to check semantic parity and render every representative page for visual inspection. Exercise long findings, long URLs/code, tables, page breaks and non-ASCII text; reject clipped/overlapping/missing content, broken citations and unreadable output. A zero-byte or merely parseable PDF is insufficient. 3. Enforce approval freshness, exact rendering/target identity, immutable provenance and redaction before export. Reject unapproved/tampered input, secret leakage, claim drift and unsafe external-resource inclusion; renderer failures leave explicit failed/withheld state, not a success manifest. 4. Open the emitted PDF and retain visual review plus content/manifest hashes. Produce local output only; no public hosting or submission is implied. Schema, mock or snapshot-only completion is forbidden.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-codefriend-pdf-renderer

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Wait for accepted merged output #896 CF-RENDER-MD. HTML and PDF are independent siblings; no cross-renderer dependency. Dependencies are OPEN in fresh snapshots; accepted implementation output is unavailable. Refresh exact predecessor source and resolve shared CLI/path ownership before scheduling or binding. #926 manages all eleven sprints only; it is not a child execution prerequisite.

## Test Strategy

- Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting. Concrete planned commands after implementation: cargo test --manifest-path adl/Cargo.toml --test codefriend_render_pdf; cargo fmt --manifest-path adl/Cargo.toml --check; git diff --check. Execute the installed export against passing and denied fixtures; required nonzero behavior and semantic parity cannot be replaced by a green generic suite. Pin the PDF engine, extract actual PDF text and render representative pages for visual inspection; this evidence is required and not inferred from serialization.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts. Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success. Wait for accepted merged output #896 CF-RENDER-MD. HTML and PDF are independent siblings; no cross-renderer dependency. Dependencies are OPEN in fresh snapshots; accepted implementation output is unavailable. Refresh exact predecessor source and resolve shared CLI/path ownership before scheduling or binding. #926 manages all eleven sprints only; it is not a child execution prerequisite. Preparation only: no implementation, proof success, implementation review, publication or live activation. Proposed renderer/CodeFriend modules are absent in current main; re-resolve the accepted predecessor contracts before editing. Root main remains inspection-only. Shared CLI/mod/lib/usage owners must coordinate changes.
