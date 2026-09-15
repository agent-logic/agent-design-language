---
schema_version: "0.1"
artifact_type: "structured_planning_prompt"
name: "v0922-derived-card-projections-execution-plan"
issue: 871
task_id: "issue-0871"
run_id: "issue-0871"
version: "0.92.2"
title: "[v0.92.2][SIM-05] Derived cards and precise evidence invalidation"
branch: "codex/871-v0922-derived-card-projections"
generated_at: "2026-09-15T16:31:32Z"
card_status: "ready"
status: "planned"
activation_state: "prepared_not_bound"
plan_revision: 1
initial_pvf_lane: "tooling"
planned_pvf_lane: "tooling"
planned_pvf_lane_source: "https://github.com/agent-logic/agent-design-language/issues/871 validation contract; docs/validation/pvf_lanes.json"
estimate_elapsed_seconds: "unknown"
estimate_total_tokens: "unknown"
estimate_validation_seconds: "unknown"
issue_goal_token_budget: "unknown"
variance_threshold_percent: "10"
estimate_confidence: "low"
estimate_data_source: "not_estimated; execution owner estimates after predecessor baseline is available"
estimate_source_ref: "https://github.com/agent-logic/agent-design-language/issues/871"
issue_goal_ref: "active sprint goal #866 in Codex task 01a092ce-eb9e-7cb3-b7ac-fc1d46ca4788 covers child #871"
sprint_goal_ref: "issue-866"
goal_metrics_rollup_ref: ".csdlc/evidence/871/goal-metrics.json (planned; absent until execution)"
source_refs:
  - kind: "issue"
    ref: "https://github.com/agent-logic/agent-design-language/issues/871"
  - kind: "source_issue_prompt"
    ref: "https://github.com/agent-logic/agent-design-language/issues/871"
  - kind: "stp"
    ref: ".csdlc/issues/871/cards/stp.md"
  - kind: "sip"
    ref: ".csdlc/issues/871/cards/sip.md"
scope:
  files:
    - "- Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation). - Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`. - Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources. - Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents."
  components:
    - "v0922-derived-card-projections"
  out_of_scope:
    - "Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot."
constraints:
  - "design_time_plan_must_be_reviewed_before_execution"
  - "runtime_execution_must_update_spp_if_plan_changes"
  - "no_hidden_scope_expansion"
confidence: "medium"
plan_summary: "Bind #871 from clean merged main through the active pre-conversion native owner. Implement deterministic six-card projection rebuild, read-only drift diagnosis, the complete executable amendment/invalidation table, and interruption recovery. Test merged semantic behavior only with isolated candidate binaries and repositories; do not convert live records or replace the shared owner."
assumptions:
  - "The linked source issue prompt, STP, and SIP remain the canonical design-time inputs."
proposed_steps:
  - id: "step-1"
    description: "Confirm dependency readiness and starting state: SIM-04/#870 is accepted and closed: PR #969 head 25b84230e8acdabc7affebcd372f0f8cbfd687e8 merged as 41f6132aa0bff2ec264a00276d237b936bc5006d; native finish authenticated terminal truth. Existing umbrella remains #866."
    expected_output: ".csdlc/issues/871/cards/sip.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-2"
    description: "Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][SIM-05] Rebuild six card projections from the semantic issue record ## Outcome An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation. ## Dependencies and execution boundary - Depends on SIM-04's merged application/transaction owner and semantic record. - Supplies deterministic projections and mapping semantics to SIM-06. Do not perform live record conversion or activate the new writer. - Reconcile ownership of command, renderer and schema paths with current work before binding this issue through native v3. Use an issue-bound goal before implementation. ## Current source and bounded implementation - Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation). - Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`. - Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources. - Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents. ## Acceptance and executed proof 1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged. 2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner. 3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table. 4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation. 5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions. 6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout. 7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task. ## Validation / PVF Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer. ## Stop conditions and non-goals Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot. ## Canonical sprint links Planning and issue-creation owner: #864. Sprint umbrella: #866 (coordination; does not gate SIM-01 startup). Execution prerequisite: #870 (SIM-04), with accepted merged output before dependent execution. Creation contract reviewed at `ed2a93338c92fda62ba63761d56af21b50483eb4`. Issue creation is not implementation start or live activation."
    expected_output: ".csdlc/issues/871/cards/stp.md"
    allowed_mode: "design_review_then_execution"
  - id: "step-3"
    description: "Implement only the bounded deliverables: - Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation). - Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`. - Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources. - Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents. An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation."
    expected_output: "tracked issue work product"
    allowed_mode: "execution_after_approval"
  - id: "step-4"
    description: "Run focused proof gates for acceptance: 1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged. 2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner. 3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table. 4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation. 5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions. 6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout. 7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task."
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
    status: "pending"
  - step: "Implement the bounded deliverables only."
    status: "pending"
  - step: "Run focused validation and proof gates."
    status: "pending"
  - step: "Record issue-specific SRP findings and VPP/SOR outcome truth."
    status: "not_started"
affected_areas:
  - "v0922-derived-card-projections"
invariants_to_preserve:
  - "Keep SPP issue-local; do not turn it into sprint orchestration."
  - "Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth."
risks_and_edge_cases:
  - "Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot. Wait for accepted merged output of #870; then refresh this plan against that exact source revision and re-resolve path ownership before binding."
test_strategy:
  - "Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer. Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient): - `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions` - `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check` - `git diff --check` Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof."
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
notes: "- Depends on SIM-04's merged application/transaction owner and semantic record. - Supplies deterministic projections and mapping semantics to SIM-06. Do not perform live record conversion or activate the new writer. - Reconcile ownership of command, renderer and schema paths with current work before binding this issue through native v3. Use an issue-bound goal before implementation. Wait for accepted merged output of #870; then refresh this plan against that exact source revision and re-resolve path ownership before binding. Preparation only: no implementation, proof success, independent implementation review, publication or live activation is claimed. Preserve existing owners #849/#862/#907 and the older worktree change to csdlc-v3/README.md. Recheck ownership before shared-path edits."
---

Canonical Template Source: `docs/templates/prompts/1.0.5/spp.md`

# Structured Plan Prompt

## Plan Summary

Design-time operative plan for `[v0.92.2][SIM-05] Derived cards and precise evidence invalidation`.

Bind #871 from clean merged main through the active pre-conversion native owner. Implement deterministic six-card projection rebuild, read-only drift diagnosis, the complete executable amendment/invalidation table, and interruption recovery. Test merged semantic behavior only with isolated candidate binaries and repositories; do not convert live records or replace the shared owner.

## PVF Lane Plan

- Initial PVF lane from issue creation: `tooling`
- Planned PVF lane for execution: `tooling`
- Planning lane source: `https://github.com/agent-logic/agent-design-language/issues/871 validation contract; docs/validation/pvf_lanes.json`
- Revision rule: change `planned_pvf_lane` only when planning discovers a better explicit lane; keep `needs_planning_lane_assignment` fail-closed until that happens.

## Estimate Plan

- Estimated elapsed seconds: `unknown`
- Estimated total tokens: `unknown`
- Estimated validation seconds: `unknown`
- Issue goal token budget: `unknown`
- Variance threshold percent: `10`
- Estimate confidence: `low`
- Estimate data source: `not_estimated; execution owner estimates after predecessor baseline is available`
- Estimate source ref: `https://github.com/agent-logic/agent-design-language/issues/871`
- Unknown-value rule: record `unknown`, never `0`, when the estimate is unavailable or intentionally deferred.

## Goal Accounting Plan

Carry `issue_goal_ref`, `sprint_goal_ref`, and `goal_metrics_rollup_ref` in frontmatter so later tooling can roll planning and outcome metrics up without duplicating machine-local goal details in prose.

## Codex Plan

1. [completed] Confirm dependencies and starting state from the source issue prompt.
2. [pending] Inspect repo inputs and target surfaces before editing.
3. [pending] Implement the bounded deliverables only.
4. [pending] Run focused validation and proof gates.
5. [not_started] Record issue-specific SRP findings and VPP/SOR outcome truth.

## Assumptions

- The linked source issue prompt, STP, and SIP remain the canonical design-time inputs.

## Proposed Steps

1. Confirm dependency readiness and starting state: SIM-04/#870 is accepted and closed: PR #969 head 25b84230e8acdabc7affebcd372f0f8cbfd687e8 merged as 41f6132aa0bff2ec264a00276d237b936bc5006d; native finish authenticated terminal truth. Existing umbrella remains #866.
2. Review repo inputs and scoped surfaces before editing: Complete source contract (live issue snapshot; preserve all requirements): # [v0.92.2][SIM-05] Rebuild six card projections from the semantic issue record ## Outcome An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation. ## Dependencies and execution boundary - Depends on SIM-04's merged application/transaction owner and semantic record. - Supplies deterministic projections and mapping semantics to SIM-06. Do not perform live record conversion or activate the new writer. - Reconcile ownership of command, renderer and schema paths with current work before binding this issue through native v3. Use an issue-bound goal before implementation. ## Current source and bounded implementation - Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation). - Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`. - Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources. - Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents. ## Acceptance and executed proof 1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged. 2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner. 3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table. 4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation. 5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions. 6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout. 7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task. ## Validation / PVF Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer. ## Stop conditions and non-goals Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot. ## Canonical sprint links Planning and issue-creation owner: #864. Sprint umbrella: #866 (coordination; does not gate SIM-01 startup). Execution prerequisite: #870 (SIM-04), with accepted merged output before dependent execution. Creation contract reviewed at `ed2a93338c92fda62ba63761d56af21b50483eb4`. Issue creation is not implementation start or live activation.
3. Implement only the bounded deliverables: - Extend projection validation and record integration in `csdlc-v3/src/application/mod.rs` (`IssueProjection`, `Projection`, card/record digest validation). - Route rendering/rebuild through `csdlc-v3/src/commands/local/mod.rs` and the SIM-04 application owner, with invalidation semantics in `csdlc-v3/src/lifecycle/mod.rs` and durable ordering/recovery in `csdlc-v3/src/storage/mod.rs`. - Resolve the active registry from `docs/templates/prompts/current.json`; current `docs/templates/prompts/1.0.5/` templates and `schemas/*.structure.json` describe the baseline only. Use the active native renderer/schema path; never hand-patch locked prose or maintain six independent semantic value sources. - Preserve SIP → STP → SPP → VPP → SRP → SOR as generated views, with semantic and projection digests stored separately. The record indexes immutable evidence identity and disposition rather than copying or modifying evidence contents. An operator can diagnose and explicitly rebuild all six C-SDLC cards from the canonical semantic issue record, while formatting drift and semantic amendments produce the correct distinct integrity and evidence-invalidation results. Deliver this usable production operation with its classification rules, tests and operator documentation.
4. Run focused proof gates for acceptance: 1. The installed candidate's explicit rebuild operation generates all six cards and their required value/projection artifacts from one semantic record using the active registry. Repeated rebuild from unchanged inputs produces identical projection bytes/digests and leaves semantic facts and evidence references unchanged. 2. Installed status/validate observes healthy, missing, altered and interrupted projections without writing files, journals, registration or repairs. A missing or modified card yields an explicit projection finding; the separate rebuild command repairs it through the transaction owner. 3. Publish an executable amendment table for scope/acceptance, plan, proof/validator, binding, implementation, review and display-only changes. Each class defines source states, prerequisites, resulting semantic state, affected evidence and causal invalidation records. Execute at least one admitted and one refused/inapplicable case per class rather than merely writing the table. 4. Formatting-only projection drift does not invalidate semantic evidence. Typed scope/plan/acceptance/validator/binding/implementation changes invalidate the correct dependent proof/review/publication evidence. A new Git commit still requires fresh exact-head review, even when the change only affects presentation. 5. Rebuild cannot invent approvals, accepted findings, proof success, publication or terminal completion; it preserves original artifact provenance and immutable receipt bytes. Reject stale/mismatched semantic versions, corrupted evidence, wrong issue/checkout and unapproved state transitions. 6. Interrupt rebuild around projection writes and restart through explicit recovery without advancing semantic state or silently blessing partial output. Ordinary inspection remains read-only throughout. 7. Tests exercise real application/installed-command paths and validate values, rendered structure and schema parity. When schemas change, include Python-readable schema smoke coverage. Required focused proof and CI pass at independent exact-head review; a renderer scaffold or authored example cards cannot close the task.
5. Record issue-specific review findings in SRP, validation-planning truth in VPP, issue outcome truth in SOR, and refresh this SPP if execution diverges.

## Affected Areas

- v0922-derived-card-projections

## Invariants To Preserve

- Keep SPP issue-local; do not turn it into sprint orchestration.
- Keep VPP as validation-planning truth, SRP as review-result truth, and SOR as output truth.

## Risks And Edge Cases

- Stop on unresolved shared-path ownership, missing execution authority, unsupported schema/record ambiguity, hidden diagnostic mutation, fabricated lifecycle truth, stale-evidence admission, failed/unexecuted proof or unfenced live activation. Record tooling anomalies durably. No new lifecycle generation, removal of any of the six cards, independent Markdown edits as authority, general documentation redesign, Runtime work, paid operations, live conversion or activation. SIM-06 performs copied-record conversion; SIM-09 controls separately authorized activation/pilot. Wait for accepted merged output of #870; then refresh this plan against that exact source revision and re-resolve path ownership before binding.

## Test Strategy

- Extend `csdlc-v3/tests/foundation.rs`, `local_commands.rs`, `operational_cli_commands.rs`, and `transactions.rs` where their paths are touched; retain current registry/schema checks. New tests carry a coupled validation inventory: lane = deterministic local CPU contract/integration; proof role = projection derivation, amendment/invalidation and explicit recovery; determinism = fixed source records, registry and golden projections; resource = bounded local CPU/disk with no network or paid calls; release gate = required SIM-07 pre-resume qualification input. Preserve stdout/stderr separation and redaction in command/error evidence. Use isolated candidate binaries and repositories, not a replacement stable live writer. Concrete baseline commands (retain required new installed scenarios from the acceptance contract; existing suites alone are insufficient): - `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test operational_cli_commands` - `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions` - `cargo fmt --manifest-path csdlc-v3/Cargo.toml --check` - `git diff --check` Re-resolve suite names after merged predecessors; a renamed or absent suite requires a documented VPP update, not a skipped proof.

## Execution Handoff

Use this SPP as the design-time plan-of-record, then hand validation-planning specifics into VPP and update both cards whenever the real execution path diverges.

## Stop Conditions

- Stop and re-plan if dependencies are unmet or materially different from this design-time plan.
- Stop and update SPP if touched files, proof gates, or validation commands change materially.
- Stop and route follow-on work if acceptance requires scope outside this issue.

## Notes

- Depends on SIM-04's merged application/transaction owner and semantic record. - Supplies deterministic projections and mapping semantics to SIM-06. Do not perform live record conversion or activate the new writer. - Reconcile ownership of command, renderer and schema paths with current work before binding this issue through native v3. Use an issue-bound goal before implementation. Wait for accepted merged output of #870; then refresh this plan against that exact source revision and re-resolve path ownership before binding. Preparation only: no implementation, proof success, independent implementation review, publication or live activation is claimed. Preserve existing owners #849/#862/#907 and the older worktree change to csdlc-v3/README.md. Recheck ownership before shared-path edits.
