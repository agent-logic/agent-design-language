# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0900
Run ID: issue-0900
Version: 0.92.2
Title: [v0.92.2][QUAL-RESIDENT] Execute resident workload and signed restore qualification
Branch: codex/900-v0922-six-resident-qualification
Card Status: ready
Generated: 2026-09-12T00:15:07.665153+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/900
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/900
- Docs: AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; docs/milestones/v0.92.2/SPRINT_v0.92.2.md; docs/milestones/v0.92.2/ATOMIC_TASK_CONTRACTS_v0.92.2.json; docs/milestones/v0.92.2/features/SUPPORTING_PLATFORM_TRACKS_v0.92.2.md
- Other: none

## Agent Execution Rules
- This issue is not started yet; do not assume a branch or worktree already exists.
- Do not use v1 wrappers; bind execution with native v3 `csdlc bind` only if execution later becomes necessary.
- Do not delete or recreate cards.
- Do not switch branches unless explicitly instructed.
- Do not work on `main`.
- Only modify files required for the issue.
- Use repository-relative paths; avoid absolute host paths.
- Write the output record to the paired local task bundle `sor.md` path.
- If repository state is unexpected, stop and ask before attempting repository repair.

## Lifecycle Semantics
- Lifecycle stage: `SIP`
- Activation state: active after issue-intent review.
- Next stage: `STP`, where the selected task or solution is made explicit.
- Downstream planning path: `STP -> SPP -> VPP -> SRP -> SOR` once execution planning becomes concrete.
- Legacy compatibility: older references may call this an input card, but new issue work should treat it as the Structured Issue Prompt.

## Prompt Spec
```yaml
prompt_schema: adl.v1
actor:
  role: execution_agent
  name: codex
model:
  id: gpt-5-codex
  determinism_mode: stable
inputs:
  sections:
    - goal
    - required_outcome
    - acceptance_criteria
    - inputs
    - target_files_surfaces
    - validation_plan
    - demo_proof_requirements
    - constraints_policies
    - system_invariants
    - reviewer_checklist
    - non_goals_out_of_scope
    - notes_risks
    - instructions_to_agent
outputs:
  output_card: .csdlc/issues/900/cards/sor.md
  summary_style: concise_structured
constraints:
  include_system_invariants: true
  include_reviewer_checklist: true
  disallow_secrets: true
  disallow_absolute_host_paths: true
automation_hints:
  source_issue_prompt_required: true
  target_files_surfaces_recommended: true
  validation_plan_required: true
  required_outcome_type_supported: true
review_surfaces:
  - card_review_checklist.v1
  - card_review_output.v1
  - card_reviewer_gpt.v1.1
```

## Execution
- Agent:
- Provider:
- Tools allowed:
- Sandbox / approvals:
- Source issue-prompt slug: v0922-six-resident-qualification
- Required outcome type: executed_production_qualification
- Demo required: true

## Goal

Execute six distinct resident tick/ACC/UTS workloads through production paths, then production signed dehydration/restore and resumed work with exact population, lineage and observable-effect evidence. The result is a completed qualification run, not a packet authoring task or six repeated copies of one synthetic action.

## Required Outcome

Execute six distinct resident tick/ACC/UTS workloads through production paths, then production signed dehydration/restore and resumed work with exact population, lineage and observable-effect evidence. The result is a completed qualification run, not a packet authoring task or six repeated copies of one synthetic action.

## Acceptance Criteria

1. Resolve the reviewed #852 failure-event repair (QUAL-RUNTIME) and RT-PROVIDER/#855 first. Execute six actual distinct roles from the bounded plan: shepherd_controller, planner, tool_executor, runtime_observer, recovery_custodian and reviewer_escalation. Bind role/tool authority/model/configuration identities and exact producer revision; placeholder artifact/configuration hashes are rejected before execution.
2. Retain each resident's tick, ACC/UTS operation, request/correlation identity, generated/provider result where applicable and observable workload effect. Measure exact admitted population and prove six distinct workloads. Caller labels or six receipt rows alone do not demonstrate six executions.
3. Exercise the production signed dehydration boundary, validate the retained population/lineage, restore through production verification and execute resumed work. Completed cases must not replay; only the exact pending case resumes. Match pre/post identity, signature, workload effects and population counts.
4. Execute tamper, omission and substitution negatives on isolated copies: changed signature/payload, removed resident, swapped lineage/provider/config or stale snapshot must be rejected before restored work. Capture actual denial and absence of inappropriate effects.
5. Distinguish deterministic local mock-provider regression from live model qualification. Complete the approved qualification profile's real production provider work; a mock trace alone cannot close the operational claim. Pin model/artifact/configuration, environment and resource limits before calls.
6. Independently review exact candidate, run artifacts, full scenario denominator, signatures and effect evidence. Sanitize public references while preserving auditable authorized access to sensitive proof; privately retained historical Run72 detail is not silently substituted for this current run.

- Acceptance: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, actual_execution_receipts, independent_review.
- PVF: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, tamper_rejected, omission_rejected, substitution_rejected, mock_vs_live_explicit, resident_execution.

## Inputs

Full live source contract, retained without dropping requirements:

# [v0.92.2][Runtime qualification][QUAL-RESIDENT] Execute six-resident workload and signed-restore qualification

## One complete result

Execute six distinct resident tick/ACC/UTS workloads through production paths, then production signed dehydration/restore and resumed work with exact population, lineage and observable-effect evidence. The result is a completed qualification run, not a packet authoring task or six repeated copies of one synthetic action.

## Owned implementation and proof paths

Reuse bounded production harnesses `adl/tools/run_issue268_six_resident_uts_cycle.py`, `adl/tools/run_issue268_continuity_uts_qualification.py`, `adl/tools/validate_issue268_six_resident_uts_plan.py`, the plan `adl/tools/issue268_six_resident_uts_plan.json`, and corresponding existing test scripts. Ground workload behavior in `adl/src/long_lived_agent.rs`, its modules/tests and the current signed continuity owner resolved by the harness. Shared Runtime source changes are limited to a demonstrated harness-blocking defect and must be routed rather than absorbing QUAL-RUNTIME/#852's event repair. Historical runner/cloud setup scripts are inputs, not permission to run them against live infrastructure.

## Executed acceptance

1. Resolve the reviewed #852 failure-event repair (QUAL-RUNTIME) and RT-PROVIDER/#855 first. Execute six actual distinct roles from the bounded plan: shepherd_controller, planner, tool_executor, runtime_observer, recovery_custodian and reviewer_escalation. Bind role/tool authority/model/configuration identities and exact producer revision; placeholder artifact/configuration hashes are rejected before execution.
2. Retain each resident's tick, ACC/UTS operation, request/correlation identity, generated/provider result where applicable and observable workload effect. Measure exact admitted population and prove six distinct workloads. Caller labels or six receipt rows alone do not demonstrate six executions.
3. Exercise the production signed dehydration boundary, validate the retained population/lineage, restore through production verification and execute resumed work. Completed cases must not replay; only the exact pending case resumes. Match pre/post identity, signature, workload effects and population counts.
4. Execute tamper, omission and substitution negatives on isolated copies: changed signature/payload, removed resident, swapped lineage/provider/config or stale snapshot must be rejected before restored work. Capture actual denial and absence of inappropriate effects.
5. Distinguish deterministic local mock-provider regression from live model qualification. Complete the approved qualification profile's real production provider work; a mock trace alone cannot close the operational claim. Pin model/artifact/configuration, environment and resource limits before calls.
6. Independently review exact candidate, run artifacts, full scenario denominator, signatures and effect evidence. Sanitize public references while preserving auditable authorized access to sensitive proof; privately retained historical Run72 detail is not silently substituted for this current run.

## Dependencies and preserved scope

Execution prerequisites: QUAL-RUNTIME, RT-PROVIDER, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

## Evidence limits that must remain explicit

Historical #268 Run72 at `8947231b6549f6b43f76c5d4656b333868a032ae` is real past execution, with detailed population/restore receipts privately retained; it is not current-candidate acceptance. #341's trace is local-reference-only and rejects supplied flags before provider invocation. #602 retains a reviewed Wuji summary with `observability_not_ready`, not complete event/response proof. Existing focused component tests and historical 50-to-54 source declarations do not close the operational claims. Re-resolve source facts before execution without rewriting historical evidence.

## Required specification obligations

- Acceptance: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, actual_execution_receipts, independent_review.
- PVF: six_distinct_workload_effects, production_signed_restore, resumed_work_effects, tamper_rejected, omission_rejected, substitution_rejected, mock_vs_live_explicit, resident_execution.

## Validation profile and resource authority

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

## Completion and non-goals

Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.

## Source and lifecycle contract

Use native C-SDLC v3 and current authenticated authority in the bound issue FastWork worktree; root main stays inspection-only. Create an issue-bound session goal before implementation, preserve active ownership, validate required lanes and obtain independent exact-head review before publication. This draft authorizes no remote write, live activation or issue closure.

Sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` (including baseline obligation ownership and criterion boundary), and `.csdlc/evidence/864/task-scope-revision/issue-scope-before.json` / `issue-scope-proposed.json`. Retained source text provides constraints, not claims that production proof has already run.

Global implementation gate: all 69 milestone issue identities must be created and all creation-batch reviews must pass before any implementation starts. Creation batch grouping adds no execution dependencies; the declared task prerequisites still apply.


## Canonical execution links

Planning owner: #864. Creation/review batch: 5; this grouping adds no execution gate.
Execution prerequisite: #852 (QUAL-RUNTIME); accepted output is required before dependent execution.
Execution prerequisite: #855 (RT-PROVIDER); accepted output is required before dependent execution.

Reviewed creation source: `b31eb6904d66bdb006b6eaeda6cdcd5f2a401260`. This issue records a complete task; creation does not claim execution or acceptance.


<!-- csdlc-v3-operation:aac547abc415bde1279d8330d978b867a5c619040b21fde4a1894d20a86c7e7a -->

## Target Files / Surfaces

Reuse bounded production harnesses `adl/tools/run_issue268_six_resident_uts_cycle.py`, `adl/tools/run_issue268_continuity_uts_qualification.py`, `adl/tools/validate_issue268_six_resident_uts_plan.py`, the plan `adl/tools/issue268_six_resident_uts_plan.json`, and corresponding existing test scripts. Ground workload behavior in `adl/src/long_lived_agent.rs`, its modules/tests and the current signed continuity owner resolved by the harness. Shared Runtime source changes are limited to a demonstrated harness-blocking defect and must be routed rather than absorbing QUAL-RUNTIME/#852's event repair. Historical runner/cloud setup scripts are inputs, not permission to run them against live infrastructure.

## Validation Plan

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

Source-grounded planned local commands: `python3 adl/tools/validate_issue268_six_resident_uts_plan.py`; `python3 adl/tools/test_run_issue268_six_resident_uts_cycle.py`; `python3 adl/tools/test_run_issue268_continuity_uts_qualification.py`. The two current tests use fake subprocess harnesses and prove local regression only. Production execution must separately run the six-resident pre/replay/post runner and continuity qualification orchestrator with exact approved runtime/continuity binaries, isolated state/evidence roots, agent-spec/configuration and signed volume identity; resolve full current argv at execution. Never substitute historical scripts, mock receipts or the prior r7i.2xlarge plan for current environment authorization.

Formatting and touched-owner regressions use the manifest of each actually changed Rust component. Enumerate actual tests and replan absent/renamed ones explicitly; zero-test filters never prove acceptance. These are future execution commands, not tests run during preparation.

`git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Demo / Proof Requirements

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

Source-grounded planned local commands: `python3 adl/tools/validate_issue268_six_resident_uts_plan.py`; `python3 adl/tools/test_run_issue268_six_resident_uts_cycle.py`; `python3 adl/tools/test_run_issue268_continuity_uts_qualification.py`. The two current tests use fake subprocess harnesses and prove local regression only. Production execution must separately run the six-resident pre/replay/post runner and continuity qualification orchestrator with exact approved runtime/continuity binaries, isolated state/evidence roots, agent-spec/configuration and signed volume identity; resolve full current argv at execution. Never substitute historical scripts, mock receipts or the prior r7i.2xlarge plan for current environment authorization.

Formatting and touched-owner regressions use the manifest of each actually changed Rust component. Enumerate actual tests and replan absent/renamed ones explicitly; zero-test filters never prove acceptance. These are future execution commands, not tests run during preparation.

`git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Constraints / Policies

- Follow `AGENTS.md`.
- Use authenticated native C-SDLC v3 for lifecycle routing. Retained typed v2 requires explicit issue-scoped rollback or remediation approval.
- Edit cards only with editor skills.
- Work only in the bound issue worktree after native v3 `csdlc bind`.
- Keep validation focused on the touched surface.

## System Invariants (must remain true)

- Deterministic execution for identical inputs.
- No hidden state or undeclared side effects.
- Artifacts remain replay-compatible with the replay runner.
- Trace artifacts contain no secrets, prompts, tool arguments, or absolute host paths.
- Artifact schema changes are explicit and approved.

## Reviewer Checklist (machine-readable hints)
```yaml
determinism_required: true
network_allowed: false
artifact_schema_change: false
replay_required: true
security_sensitive: true
ci_validation_required: true
```

## Card Automation Hooks (prompt generation)
- Prompt source fields:
  - Goal
  - Required Outcome
  - Acceptance Criteria
  - Inputs
  - Target Files / Surfaces
  - Validation Plan
  - Demo / Proof Requirements
  - Constraints / Policies
  - System Invariants
  - Reviewer Checklist
- Generation requirements:
  - Deterministic output for identical SIP content
  - No secrets, tokens, or absolute host paths in generated prompt text
  - Preserve traceability back to the source issue prompt
  - Preserve explicit required-outcome and demo/proof requirements

## Non-goals / Out of scope

Completion requires the one actual result above, all positive/negative obligations, current independent exact-head review and required checks. Plans, schemas, scaffolds, test-only consumers, packets without execution or zero-test invocations cannot satisfy it. No broad Runtime refactor, paid-cloud mutation, release approval, automatic #522/#833 closure or unrelated backlog. Retain stop conditions: required_proof_not_executed, partial_artifact_claimed_complete, synthetic_only_proof, stale_revision, missing_scenario. Also stop for missing authority, ownership collision, sensitive-data leakage or unsafe effect scope; route separate defects explicitly.

## Notes / Risks

Execution prerequisites: QUAL-RUNTIME, RT-PROVIDER, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

Historical #268 Run72 at `8947231b6549f6b43f76c5d4656b333868a032ae` is real past execution, with detailed population/restore receipts privately retained; it is not current-candidate acceptance. #341's trace is local-reference-only and rejects supplied flags before provider invocation. #602 retains a reviewed Wuji summary with `observability_not_ready`, not complete event/response proof. Existing focused component tests and historical 50-to-54 source declarations do not close the operational claims. Re-resolve source facts before execution without rewriting historical evidence.

Use native C-SDLC v3 and current authenticated authority in the bound issue FastWork worktree; root main stays inspection-only. Create an issue-bound session goal before implementation, preserve active ownership, validate required lanes and obtain independent exact-head review before publication. This draft authorizes no remote write, live activation or issue closure.

Sources: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json` (including baseline obligation ownership and criterion boundary), and `.csdlc/evidence/864/task-scope-revision/issue-scope-before.json` / `issue-scope-proposed.json`. Retained source text provides constraints, not claims that production proof has already run.

Global implementation gate: all 69 milestone issue identities must be created and all creation-batch reviews must pass before any implementation starts. Creation batch grouping adds no execution dependencies; the declared task prerequisites still apply.

#852 (QUAL-RUNTIME) and #855 (RT-PROVIDER) accepted merged outputs are required and not satisfied in this preparation.

Prepared, not bound. Execution Sprint 5 is a scheduling assignment, not dependency satisfaction. All implementation steps, proving runs, implementation review, PR and terminal state remain unstarted. Source inspection found the registered #851 worktree still dirty on governed_operations.rs, conversation_sessions_tests.rs, ingress.rs, telemetry.rs and long_lived_agent tests. Preserve those bytes; reconcile actual owner before any shared-path edits. Required production resource and provider authority is not granted by this preparation; budget/environment approval remains an execution blocker.

Branch codex/900-v0922-six-resident-qualification and its proposed FastWork path are unbound planning values only. Issue #926 owns umbrella management for all eleven sprints; it is not the Sprint 5 umbrella or a child execution prerequisite.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
