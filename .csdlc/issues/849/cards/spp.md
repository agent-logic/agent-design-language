---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-merge-linkage-admission-execution-plan"
issue: 849
task_id: "issue-0849"
run_id: "issue-0849"
version: "0.92.2"
title: "[v0.92.2][C-SDLC v3][P2] Preserve publication linkage during native PR merge"
branch: "codex/849-v0922-merge-linkage-admission"
generated_at: "2026-09-12T00:22:05.416044+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "bound"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/849 validation contract; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "14400"
estimate_total_tokens: "35000"
estimate_validation_seconds: "2400"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "Conservative planning estimate for bounded Rust implementation plus deterministic installed-consumer fixtures, assuming warm cache; recalibrate against accepted upstream and focused-regression availability; not an imposed token limit"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/849"
issue_goal_ref: "Issue #849 session active: reviewed implementation and truthful PR publication through green CI; no merge authorization"
sprint_goal_ref: "v0.92.2 Sprint 7 umbrella #933"
goal_metrics_rollup_ref: ".csdlc/evidence/849/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/849"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/849"
  - kind: "stp"
    ref: ".csdlc/issues/849/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/849/cards/sip.md"
scope:
  files:
    - "Own merge-linkage admission in csdlc-v3/src/commands/remote/mod.rs, remote/merge.rs and remote/tests/merge_cases.rs; bounded request/intent/review-linkage types and exact affected documentation/release-criterion mappings only. Coordinate with SIM transaction/CLI owners and #907; do not refactor unrelated remote routes."
  components:
    - "v0922-merge-linkage-admission"
  out_of_scope:
    - "Repair merge linkage only. No authority-generation change, blanket release approval, unrelated runtime work, or rewriting closed #835/#844 records. Preserve the findings in #522/#833 until verified correction or an explicit release disposition."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Recheck current merge source and SIM/remote-owner edits against MERGE-LINKAGE-001; bind reviewed qualified PublicationLinkage and publication mode into admission and durable intent; authenticate current PR body relation and reject same-head drift or missing/mixed/ambiguous/wrong-target/repository/mode mismatch before dispatch; preserve valid Closing and PartOf checkpoint/terminal distinctions, uncertain replay and authenticated reconciliation; execute fake-transport rejection/no-dispatch and positive merge fixtures, update affected current mappings while retaining historical failure evidence, and document the remaining body/base/policy race beyond head CAS before independent exact-head review."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added."
    expected_output: ".csdlc/issues/849/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): ## Problem At reviewed revision `25498d7709cb5fe13242aac81988ecdb344db968` (PR #847, issue #844), native PR merge admission does not bind or validate the PR's current closing/non-closing relation to the reviewed `PublicationLinkage`. A `PartOf` PR body can change to `Closes #<parent>` without a head change. Exact-head review and check admission can still succeed, allowing merge to close the parent. Conversely, removing the closing relation from a `Closing` PR can leave its target open. Later `finish` validation cannot undo the merge or issue closure. Independent reviewer: `review_835`; severity P2; finding `MERGE-LINKAGE-001`. Discovered during #835 release proof and handed to #522/#833. Operator explicitly allocated this repair to **v0.92.2**. This allocation is not a behavioral pass or v0.92.1 release approval. ## Evidence - `csdlc-v3/src/commands/remote/mod.rs:205`: `PullRequestMerge` lacks `PublicationLinkage` and mode. - `csdlc-v3/src/commands/remote/merge.rs:38`: authenticated query omits PR body/closing relation. - `csdlc-v3/src/commands/remote/merge.rs:345`: review admission binds repository, issue and revision, but not the current PR relation. Existing publication and finish guards retain their prior guarantees; the defect is the new merge path. The exact merged revision passed 240 unfiltered C-SDLC tests, strict all-target Clippy and the V3-A contract check; these checks do not cover this missing guard. ## Required change Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations. ## Acceptance criteria - [ ] Merge consumes the reviewed linkage value bound to the exact source revision and qualified target issue. - [ ] Same-head body drift from `PartOf` to a closing relation is rejected before mutation; the parent remains open. - [ ] A `Closing` PR with its closing relation removed or changed is rejected before mutation. - [ ] Missing, mixed, wrong-target, wrong-repository and split-repository unqualified linkage are rejected. - [ ] A valid non-closing merge proves checkpoint completion while its parent remains open; a valid closing merge preserves the existing terminal contract. - [ ] Durable intent, uncertain replay and exact authenticated reconciliation retain their existing safeguards. - [ ] Documentation retains the remote CAS limitation: head CAS does not atomically freeze body, base or policy. Do not claim that preflight alone eliminates a later remote race. - [ ] Independent exact-head review accepts the repair and the affected release mappings are updated without rewriting historical failed evidence. ## Focused validation PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof. Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission. ## Scope Repair merge linkage only. No authority-generation change, blanket release approval, unrelated runtime work, or rewriting closed #835/#844 records. Preserve the findings in #522/#833 until verified correction or an explicit release disposition. ## Current milestone execution links Planning owner: #864. Execution prerequisite: #864 (WP-01). Accepted prerequisite output is required before dependent execution. The operator requires all 69 milestone task identities to be created and reviewed before this launch admits new implementation. Each issue then uses its own native readiness and bound execution route; creation/review grouping adds no dependency edge. ## Execution sprint assignment **Sprint 7** in `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. This assignment includes the existing issue in the complete 69-task execution schedule. It creates no new issue or dependency edge; existing prerequisites, same-sprint dependency order, native readiness and the all-69 creation/review startup gate remain in force."
    expected_output: ".csdlc/issues/849/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: Own merge-linkage admission in csdlc-v3/src/commands/remote/mod.rs, remote/merge.rs and remote/tests/merge_cases.rs; bounded request/intent/review-linkage types and exact affected documentation/release-criterion mappings only. Coordinate with SIM transaction/CLI owners and #907; do not refactor unrelated remote routes. Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: - [ ] Merge consumes the reviewed linkage value bound to the exact source revision and qualified target issue. - [ ] Same-head body drift from `PartOf` to a closing relation is rejected before mutation; the parent remains open. - [ ] A `Closing` PR with its closing relation removed or changed is rejected before mutation. - [ ] Missing, mixed, wrong-target, wrong-repository and split-repository unqualified linkage are rejected. - [ ] A valid non-closing merge proves checkpoint completion while its parent remains open; a valid closing merge preserves the existing terminal contract. - [ ] Durable intent, uncertain replay and exact authenticated reconciliation retain their existing safeguards. - [ ] Documentation retains the remote CAS limitation: head CAS does not atomically freeze body, base or policy. Do not claim that preflight alone eliminates a later remote race. - [ ] Independent exact-head review accepts the repair and the affected release mappings are updated without rewriting historical failed evidence."
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
    status: "in_progress"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "in_progress"
affected_areas:
  - "v0922-merge-linkage-admission"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Repair merge linkage only. No authority-generation change, blanket release approval, unrelated runtime work, or rewriting closed #835/#844 records. Preserve the findings in #522/#833 until verified correction or an explicit release disposition. #864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added."
test_strategy:
  - "PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof. Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission. Planned commands: cargo test --manifest-path csdlc-v3/Cargo.toml --lib -- --list to enumerate actual merge_cases tests; run exact registered module/filter with nonzero same-head drift, qualified Closing/PartOf, malformed/mixed/wrong-target/repository and uncertain-reconciliation cases. Assert zero mutation dispatch on rejected inputs. Run cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands and --test operational_cli_commands for affected public route coverage, cargo fmt --manifest-path csdlc-v3/Cargo.toml --check and git diff --check. Extend scope only to touched semantic owner regressions, then required CI. Fake authenticated transport proves contract semantics; no live destructive merge is required or authorized. Record exact fixture denominators and coupled tooling-PVF role/determinism/local CPU/Git/process resource/release-gate inventory."
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
notes: "#864 prerequisite accepted via merged #865 and all69 startup gate passed. Native bound #849 worktree and active goal under Sprint7 #933. Planning #4.4 confirmed shared remote paths available; preserve unmerged #948 pending-receipt assertions and latest03c5aead adapter curl-config guard when integrating. Implementation0264f321ac independently reviewed, local focused proof passed; full owner suite has separately recorded UTS release-inventory failure. No live merge or shared binary installation authorized."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC v3][P2] Preserve publication linkage during native PR merge`.

Recheck current merge source and SIM/remote-owner edits against MERGE-LINKAGE-001; bind reviewed qualified PublicationLinkage and publication mode into admission and durable intent; authenticate current PR body relation and reject same-head drift or missing/mixed/ambiguous/wrong-target/repository/mode mismatch before dispatch; preserve valid Closing and PartOf checkpoint/terminal distinctions, uncertain replay and authenticated reconciliation; execute fake-transport rejection/no-dispatch and positive merge fixtures, update affected current mappings while retaining historical failure evidence, and document the remaining body/base/policy race beyond head CAS before independent exact-head review.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/849 validation contract; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `14400`
- Estimated total tokens: `35000`
- Estimated validation seconds: `2400`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `Conservative planning estimate for bounded Rust implementation plus deterministic installed-consumer fixtures, assuming warm cache; recalibrate against accepted upstream and focused-regression availability; not an imposed token limit`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/849`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [in_progress] Run focused validation and proof gates.
5. [in_progress] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added.
2. Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): ## Problem At reviewed revision `25498d7709cb5fe13242aac81988ecdb344db968` (PR #847, issue #844), native PR merge admission does not bind or validate the PR's current closing/non-closing relation to the reviewed `PublicationLinkage`. A `PartOf` PR body can change to `Closes #<parent>` without a head change. Exact-head review and check admission can still succeed, allowing merge to close the parent. Conversely, removing the closing relation from a `Closing` PR can leave its target open. Later `finish` validation cannot undo the merge or issue closure. Independent reviewer: `review_835`; severity P2; finding `MERGE-LINKAGE-001`. Discovered during #835 release proof and handed to #522/#833. Operator explicitly allocated this repair to **v0.92.2**. This allocation is not a behavioral pass or v0.92.1 release approval. ## Evidence - `csdlc-v3/src/commands/remote/mod.rs:205`: `PullRequestMerge` lacks `PublicationLinkage` and mode. - `csdlc-v3/src/commands/remote/merge.rs:38`: authenticated query omits PR body/closing relation. - `csdlc-v3/src/commands/remote/merge.rs:345`: review admission binds repository, issue and revision, but not the current PR relation. Existing publication and finish guards retain their prior guarantees; the defect is the new merge path. The exact merged revision passed 240 unfiltered C-SDLC tests, strict all-target Clippy and the V3-A contract check; these checks do not cover this missing guard. ## Required change Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations. ## Acceptance criteria - [ ] Merge consumes the reviewed linkage value bound to the exact source revision and qualified target issue. - [ ] Same-head body drift from `PartOf` to a closing relation is rejected before mutation; the parent remains open. - [ ] A `Closing` PR with its closing relation removed or changed is rejected before mutation. - [ ] Missing, mixed, wrong-target, wrong-repository and split-repository unqualified linkage are rejected. - [ ] A valid non-closing merge proves checkpoint completion while its parent remains open; a valid closing merge preserves the existing terminal contract. - [ ] Durable intent, uncertain replay and exact authenticated reconciliation retain their existing safeguards. - [ ] Documentation retains the remote CAS limitation: head CAS does not atomically freeze body, base or policy. Do not claim that preflight alone eliminates a later remote race. - [ ] Independent exact-head review accepts the repair and the affected release mappings are updated without rewriting historical failed evidence. ## Focused validation PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof. Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission. ## Scope Repair merge linkage only. No authority-generation change, blanket release approval, unrelated runtime work, or rewriting closed #835/#844 records. Preserve the findings in #522/#833 until verified correction or an explicit release disposition. ## Current milestone execution links Planning owner: #864. Execution prerequisite: #864 (WP-01). Accepted prerequisite output is required before dependent execution. The operator requires all 69 milestone task identities to be created and reviewed before this launch admits new implementation. Each issue then uses its own native readiness and bound execution route; creation/review grouping adds no dependency edge. ## Execution sprint assignment **Sprint 7** in `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. This assignment includes the existing issue in the complete 69-task execution schedule. It creates no new issue or dependency edge; existing prerequisites, same-sprint dependency order, native readiness and the all-69 creation/review startup gate remain in force.
3. Implement only the bounded deliverables: Own merge-linkage admission in csdlc-v3/src/commands/remote/mod.rs, remote/merge.rs and remote/tests/merge_cases.rs; bounded request/intent/review-linkage types and exact affected documentation/release-criterion mappings only. Coordinate with SIM transaction/CLI owners and #907; do not refactor unrelated remote routes. Bind the reviewed, qualified `PublicationLinkage` and publication mode into merge admission and durable intent. Authenticate and validate the current PR relation before dispatch. Reject absent, mixed, ambiguous, wrong-target, wrong-repository and mode-incompatible relations. Reconciliation must preserve truthful linkage and issue-state observations.
4. Run focused proof gates for acceptance: - [ ] Merge consumes the reviewed linkage value bound to the exact source revision and qualified target issue. - [ ] Same-head body drift from `PartOf` to a closing relation is rejected before mutation; the parent remains open. - [ ] A `Closing` PR with its closing relation removed or changed is rejected before mutation. - [ ] Missing, mixed, wrong-target, wrong-repository and split-repository unqualified linkage are rejected. - [ ] A valid non-closing merge proves checkpoint completion while its parent remains open; a valid closing merge preserves the existing terminal contract. - [ ] Durable intent, uncertain replay and exact authenticated reconciliation retain their existing safeguards. - [ ] Documentation retains the remote CAS limitation: head CAS does not atomically freeze body, base or policy. Do not claim that preflight alone eliminates a later remote race. - [ ] Independent exact-head review accepts the repair and the affected release mappings are updated without rewriting historical failed evidence.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-merge-linkage-admission

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Repair merge linkage only. No authority-generation change, blanket release approval, unrelated runtime work, or rewriting closed #835/#844 records. Preserve the findings in #522/#833 until verified correction or an explicit release disposition. #864 WP-01 prerequisite delivered by merged PR #865; all69 identity/creation reviews completed. Recheck accepted evidence and current path ownership before bind; no dependency on entire earlier sprints is added.

## Test Strategy

- PVF: required deterministic native C-SDLC owner contract; local Git/filesystem and fake authenticated transport; small CPU; no paid cloud resources or live destructive merge required. Add same-head body-drift negative cases and valid Closing/PartOf cases in the existing merge tests. Assert no mutation dispatch for rejected inputs. Run focused remote tests and the native C-SDLC suite appropriate to the final diff; keep live GitHub proof separate from fake-transport proof. Affected retained criteria: `V3-A:retained-161-ac-8`, `V3-E:retained-175-ac-10`, `V3-E:retained-175-ac-11`, `V3-E:retained-177-ac-3`. Historical `V3-E:V3-E-ac-2` remains qualified when extended to merge admission. Planned commands: cargo test --manifest-path csdlc-v3/Cargo.toml --lib -- --list to enumerate actual merge_cases tests; run exact registered module/filter with nonzero same-head drift, qualified Closing/PartOf, malformed/mixed/wrong-target/repository and uncertain-reconciliation cases. Assert zero mutation dispatch on rejected inputs. Run cargo test --manifest-path csdlc-v3/Cargo.toml --test remote_publication_commands and --test operational_cli_commands for affected public route coverage, cargo fmt --manifest-path csdlc-v3/Cargo.toml --check and git diff --check. Extend scope only to touched semantic owner regressions, then required CI. Fake authenticated transport proves contract semantics; no live destructive merge is required or authorized. Record exact fixture denominators and coupled tooling-PVF role/determinism/local CPU/Git/process resource/release-gate inventory.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

#864 prerequisite accepted via merged #865 and all69 startup gate passed. Native bound #849 worktree and active goal under Sprint7 #933. Planning #4.4 confirmed shared remote paths available; preserve unmerged #948 pending-receipt assertions and latest03c5aead adapter curl-config guard when integrating. Implementation0264f321ac independently reviewed, local focused proof passed; full owner suite has separately recorded UTS release-inventory failure. No live merge or shared binary installation authorized.
