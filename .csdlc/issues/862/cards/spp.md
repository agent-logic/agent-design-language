---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-local-command-decomposition-execution-plan"
issue: 862
task_id: "issue-0862"
run_id: "issue-0862"
version: "0.92.2"
title: "[v0.92.2][C-SDLC v3][refactor] Decompose the local command owner"
branch: "codex/862-v0922-local-command-decomposition"
generated_at: "2026-09-16T00:27:06.103043+00:00"
card_status: "ready"
status: "in_progress"
activation_state: "execution_in_progress"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/862 behavior-preserving C-SDLC command-owner decomposition; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "21600"
estimate_total_tokens: "50000"
estimate_validation_seconds: "1800"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "bounded focused validation estimate; implementation owner recalibrates after prerequisite source is available"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/862"
issue_goal_ref: "active Codex goal for Sprint #933 / issue #862"
sprint_goal_ref: "v0.92.2 Sprint 7 coordination issue #933; descriptive only"
goal_metrics_rollup_ref: ".csdlc/evidence/862/goal-metrics.json (planned, absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/862"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/862"
  - kind: "stp"
    ref: ".csdlc/issues/862/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/862/cards/sip.md"
scope:
  files:
    - "- Extract local binding, editing, validation, registry/registration, transaction and state-persistence responsibilities into cohesive modules. - Leave `csdlc-v3/src/commands/local/mod.rs` as thin module wiring and stable dispatch. - Preserve CLI behavior, request/receipt schemas, durable paths, authority checks, operation digests, error codes, idempotency and fail-closed semantics. - Retain focused regressions and boundary tests; record a recursive before/after responsibility and source-size inventory. Own csdlc-v3/src/commands/local/mod.rs and proposed cohesive sibling modules beneath commands/local/ for binding, editing, validation, registry/registration, transactions and state persistence. Coordinate csdlc-v3/tests/local_commands.rs, operational_cli_commands.rs, transactions.rs and affected boundary tests with SIM owners; final file names follow the reviewed responsibility inventory."
  components:
    - "v0922-local-command-decomposition"
  out_of_scope:
    - "Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation. `CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Verify accepted #864 baseline and reconcile SIM/local-owner and installed-writer coordination; record recursive source/responsibility and route/serialized-contract inventories; extract binding, editing, validation, registration, transaction and state-persistence responsibilities into cohesive local modules with thin stable dispatch; run each affected production route and authority/idempotency/digest/crash/error golden negatives against isolated state; independently check dependency direction and complete route migration before current review and native publication."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: #864 WP-01 is accepted via merged PR #865 and the all-69 creation/review gate is satisfied. #849 is accepted via merged PR #952. #862 has no dependency on unfinished SIM-06 through SIM-09 or on an entire earlier sprint."
    expected_output: ".csdlc/issues/862/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Full live source contract, retained without dropping requirements: ## Outcome Decompose the C-SDLC v3 local command owner into cohesive internal components while preserving all public behavior. ## Complete task - Extract local binding, editing, validation, registry/registration, transaction and state-persistence responsibilities into cohesive modules. - Leave `csdlc-v3/src/commands/local/mod.rs` as thin module wiring and stable dispatch. - Preserve CLI behavior, request/receipt schemas, durable paths, authority checks, operation digests, error codes, idempotency and fail-closed semantics. - Retain focused regressions and boundary tests; record a recursive before/after responsibility and source-size inventory. ## Acceptance 1. Every extracted local module has one named responsibility and no command-domain cycles. 2. All local production routes use the decomposition; moving code into a replacement god module does not qualify. 3. Public and serialized contracts, digests, error codes and paths remain compatible. 4. Local/native authority, mutation, terminal and negative-path regression proof passes at the reviewed revision. ## Remote decomposition routing `CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction. ## Non-goals Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation. ## Operator-authorized task split The operator requested one complete task per issue in the #864 / PR #865 planning review. Additional work is preserved in `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.md`. The earlier scope-only edit created no follow-on issues; the operator subsequently authorized their creation and independent review through #864. All implementation tasks include their required tests, failure handling and documentation. <!-- csdlc-v3-operation:59c2d9609840f5ff7993e765cb441904734af432a3c34d4eb8dc388d7c4012e9 --> ## Current milestone execution links Planning owner: #864. Execution prerequisite: #864 (WP-01). Accepted prerequisite output is required before dependent execution. The operator requires all 69 milestone task identities to be created and reviewed before this launch admits new implementation. Each issue then uses its own native readiness and bound execution route; creation/review grouping adds no dependency edge. Separate complete tasks: CSDLC-REMOTE / #907. Their scopes and acceptance remain separate from this issue. <!-- csdlc-v3-operation:cecdae1f02b944b3a812c800f49cd9f1fb7d479564a19c8b70a40a18186201b9 --> ## Execution sprint assignment **Sprint 7** in `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. This assignment includes the existing issue in the complete 69-task execution schedule. It creates no new issue or dependency edge; existing prerequisites, same-sprint dependency order, native readiness and the all-69 creation/review startup gate remain in force. <!-- csdlc-v3-operation:5e863d373f23aba2aaefe31239c730d99cd13de16bef957c05248703e936a708 -->"
    expected_output: ".csdlc/issues/862/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: - Extract local binding, editing, validation, registry/registration, transaction and state-persistence responsibilities into cohesive modules. - Leave `csdlc-v3/src/commands/local/mod.rs` as thin module wiring and stable dispatch. - Preserve CLI behavior, request/receipt schemas, durable paths, authority checks, operation digests, error codes, idempotency and fail-closed semantics. - Retain focused regressions and boundary tests; record a recursive before/after responsibility and source-size inventory. Own csdlc-v3/src/commands/local/mod.rs and proposed cohesive sibling modules beneath commands/local/ for binding, editing, validation, registry/registration, transactions and state persistence. Coordinate csdlc-v3/tests/local_commands.rs, operational_cli_commands.rs, transactions.rs and affected boundary tests with SIM owners; final file names follow the reviewed responsibility inventory. Decompose the C-SDLC v3 local command owner into cohesive internal components while preserving all public behavior."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. Every extracted local module has one named responsibility and no command-domain cycles. 2. All local production routes use the decomposition; moving code into a replacement god module does not qualify. 3. Public and serialized contracts, digests, error codes and paths remain compatible. 4. Local/native authority, mutation, terminal and negative-path regression proof passes at the reviewed revision."
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
    status: "implementation, validation and independent review complete; native publication and CI pending"
affected_areas:
  - "v0922-local-command-decomposition"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation. `CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction. #864 (WP-01) is accepted via merged PR #865 at f1c4e2a915c215797f0d2708cb8b0568f2b80b32; live issue closure and PR merge were verified during preparation. All-69 creation/review gate is satisfied. Shared SIM ownership still requires reconciliation."
test_strategy:
  - "Planned deterministic local tooling/contract/integration proof, bounded local CPU/Git/Rust/disk, isolated primary and linked worktree fixtures plus controlled fake authenticated remote transport where a local boundary consumes remote or terminal facts. Preserve exact CLI/result/serialized bytes, request/receipt digests, durable paths, authority checks, errors, idempotency and fail-closed behavior. Record each route and negative scenario with nonzero denominator; facade line counts or a replacement god module are not acceptance. Current source-grounded planned commands: `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_worktree_binding`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`; `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Enumerate registered tests after predecessors land; preserve serialized-contract goldens and add focused meaningful missing boundary cases. These existing suites are inputs, not proof that every extraction is covered; retain a complete route-to-test inventory and run applicable internal remote merge cases after #849. No zero-test filtered invocation qualifies. Record before/after recursive source/responsibility inventory, one owner per responsibility and module dependency direction; test both admitted and denied routes, mutation recovery and unchanged outputs against the exact candidate. Required CI and independent review are separate from local proof. Build isolated candidate binaries and never replace another session stable operational writer during tests. `git diff --check` accompanies focused proof. Future proving commands are planned only; no Rust tests or product execution run during this preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation."
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
notes: "Implementation is complete in the bound issue worktree. The focused candidate proof is green: 152 tests plus strict clippy, formatting and diff checks. Independent exact-head review completed with no remaining actionable findings after three accepted P2 findings were resolved. The broader owner lane stopped on an unchanged baseline mismatch: test_card_prompt.sh expects template set 1.0.3 while the selected registry is 1.0.5; this issue does not change either surface. Native publication and hosted CI remain pending."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][C-SDLC v3][refactor] Decompose the local command owner`.

Verify accepted #864 baseline and reconcile SIM/local-owner and installed-writer coordination; record recursive source/responsibility and route/serialized-contract inventories; extract binding, editing, validation, registration, transaction and state-persistence responsibilities into cohesive local modules with thin stable dispatch; run each affected production route and authority/idempotency/digest/crash/error golden negatives against isolated state; independently check dependency direction and complete route migration before current review and native publication.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/862 behavior-preserving C-SDLC command-owner decomposition; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `21600`
- Estimated total tokens: `50000`
- Estimated validation seconds: `1800`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `bounded focused validation estimate; implementation owner recalibrates after prerequisite source is available`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/862`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [completed] Inspect repo inputs and target surfaces before editing.
3. [completed] Implement the bounded deliverables only.
4. [completed] Run focused validation and proof gates.
5. [implementation, validation and independent review complete; native publication and CI pending] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: #864 WP-01 is accepted via merged PR #865 and the all-69 creation/review gate is satisfied. #849 is accepted via merged PR #952. #862 has no dependency on unfinished SIM-06 through SIM-09 or on an entire earlier sprint.
2. Review repo inputs and scoped surfaces before editing: Full live source contract, retained without dropping requirements: ## Outcome Decompose the C-SDLC v3 local command owner into cohesive internal components while preserving all public behavior. ## Complete task - Extract local binding, editing, validation, registry/registration, transaction and state-persistence responsibilities into cohesive modules. - Leave `csdlc-v3/src/commands/local/mod.rs` as thin module wiring and stable dispatch. - Preserve CLI behavior, request/receipt schemas, durable paths, authority checks, operation digests, error codes, idempotency and fail-closed semantics. - Retain focused regressions and boundary tests; record a recursive before/after responsibility and source-size inventory. ## Acceptance 1. Every extracted local module has one named responsibility and no command-domain cycles. 2. All local production routes use the decomposition; moving code into a replacement god module does not qualify. 3. Public and serialized contracts, digests, error codes and paths remain compatible. 4. Local/native authority, mutation, terminal and negative-path regression proof passes at the reviewed revision. ## Remote decomposition routing `CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction. ## Non-goals Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation. ## Operator-authorized task split The operator requested one complete task per issue in the #864 / PR #865 planning review. Additional work is preserved in `docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.md`. The earlier scope-only edit created no follow-on issues; the operator subsequently authorized their creation and independent review through #864. All implementation tasks include their required tests, failure handling and documentation. <!-- csdlc-v3-operation:59c2d9609840f5ff7993e765cb441904734af432a3c34d4eb8dc388d7c4012e9 --> ## Current milestone execution links Planning owner: #864. Execution prerequisite: #864 (WP-01). Accepted prerequisite output is required before dependent execution. The operator requires all 69 milestone task identities to be created and reviewed before this launch admits new implementation. Each issue then uses its own native readiness and bound execution route; creation/review grouping adds no dependency edge. Separate complete tasks: CSDLC-REMOTE / #907. Their scopes and acceptance remain separate from this issue. <!-- csdlc-v3-operation:cecdae1f02b944b3a812c800f49cd9f1fb7d479564a19c8b70a40a18186201b9 --> ## Execution sprint assignment **Sprint 7** in `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. This assignment includes the existing issue in the complete 69-task execution schedule. It creates no new issue or dependency edge; existing prerequisites, same-sprint dependency order, native readiness and the all-69 creation/review startup gate remain in force. <!-- csdlc-v3-operation:5e863d373f23aba2aaefe31239c730d99cd13de16bef957c05248703e936a708 -->
3. Implement only the bounded deliverables: - Extract local binding, editing, validation, registry/registration, transaction and state-persistence responsibilities into cohesive modules. - Leave `csdlc-v3/src/commands/local/mod.rs` as thin module wiring and stable dispatch. - Preserve CLI behavior, request/receipt schemas, durable paths, authority checks, operation digests, error codes, idempotency and fail-closed semantics. - Retain focused regressions and boundary tests; record a recursive before/after responsibility and source-size inventory. Own csdlc-v3/src/commands/local/mod.rs and proposed cohesive sibling modules beneath commands/local/ for binding, editing, validation, registry/registration, transactions and state persistence. Coordinate csdlc-v3/tests/local_commands.rs, operational_cli_commands.rs, transactions.rs and affected boundary tests with SIM owners; final file names follow the reviewed responsibility inventory. Decompose the C-SDLC v3 local command owner into cohesive internal components while preserving all public behavior.
4. Run focused proof gates for acceptance: 1. Every extracted local module has one named responsibility and no command-domain cycles. 2. All local production routes use the decomposition; moving code into a replacement god module does not qualify. 3. Public and serialized contracts, digests, error codes and paths remain compatible. 4. Local/native authority, mutation, terminal and negative-path regression proof passes at the reviewed revision.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-local-command-decomposition

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Remote command decomposition in this issue, lifecycle feature changes, schema redesign, weakened guards, or unrelated cleanup. Coordinate shared paths and installed-binary ownership with the SIM sprint before implementation. `CSDLC-REMOTE` separately completes the remote command owner, after this local extraction and CSDLC-MERGE/#849. It retains authority/credentials, GitHub issue/PR operations, publication/readback, mutation reconciliation, receipts and validation decomposition plus its own unchanged-contract proof. No remote work is discarded when #862 closes, and no child is created by this scope correction. #864 (WP-01) is accepted via merged PR #865 at f1c4e2a915c215797f0d2708cb8b0568f2b80b32; live issue closure and PR merge were verified during preparation. All-69 creation/review gate is satisfied. Shared SIM ownership still requires reconciliation.

## Test Strategy

- Planned deterministic local tooling/contract/integration proof, bounded local CPU/Git/Rust/disk, isolated primary and linked worktree fixtures plus controlled fake authenticated remote transport where a local boundary consumes remote or terminal facts. Preserve exact CLI/result/serialized bytes, request/receipt digests, durable paths, authority checks, errors, idempotency and fail-closed behavior. Record each route and negative scenario with nonzero denominator; facade line counts or a replacement god module are not acceptance. Current source-grounded planned commands: `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_worktree_binding`; `cargo test --manifest-path csdlc-v3/Cargo.toml --test terminal_cleanup_cutover_commands`; `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check`. Enumerate registered tests after predecessors land; preserve serialized-contract goldens and add focused meaningful missing boundary cases. These existing suites are inputs, not proof that every extraction is covered; retain a complete route-to-test inventory and run applicable internal remote merge cases after #849. No zero-test filtered invocation qualifies. Record before/after recursive source/responsibility inventory, one owner per responsibility and module dependency direction; test both admitted and denied routes, mutation recovery and unchanged outputs against the exact candidate. Required CI and independent review are separate from local proof. Build isolated candidate binaries and never replace another session stable operational writer during tests. `git diff --check` accompanies focused proof. Future proving commands are planned only; no Rust tests or product execution run during this preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

Implementation is complete in the bound issue worktree. The focused candidate proof is green: 152 tests plus strict clippy, formatting and diff checks. Independent exact-head review completed with no remaining actionable findings after three accepted P2 findings were resolved. The broader owner lane stopped on an unchanged baseline mismatch: test_card_prompt.sh expects template set 1.0.3 while the selected registry is 1.0.5; this issue does not change either surface. Native publication and hosted CI remain pending.
