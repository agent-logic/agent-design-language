# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0901
Run ID: issue-0901
Version: 0.92.2
Title: [v0.92.2][QUAL-PROVIDER] Execute real provider failure and recovery qualification
Branch: codex/901-v0922-provider-recovery-qualification
Card Status: ready
Generated: 2026-09-12T00:15:14.473149+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/901
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/901
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
  output_card: .csdlc/issues/901/cards/sor.md
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
- Source issue-prompt slug: v0922-provider-recovery-qualification
- Required outcome type: executed_production_qualification
- Demo required: true

## Goal

Induce real provider subprocess loss, timeout and interruption through the production provider harness and demonstrate bounded recovery with a healthy configured provider. This task qualifies actual effects and recovery; caller-set failure flags and named reference traces are not execution proof.

## Required Outcome

Induce real provider subprocess loss, timeout and interruption through the production provider harness and demonstrate bounded recovery with a healthy configured provider. This task qualifies actual effects and recovery; caller-set failure flags and named reference traces are not execution proof.

## Acceptance Criteria

1. RT-PROVIDER/#855 must deliver its registered-provider lifecycle before this qualification. Select a scoped owned test subprocess/session through that production route, observe actual start, kill/loss, deadline timeout and cancellation/interruption, and retain process/request identity, timestamps, exit state, Runtime outcome and correlation.
2. Restore service through a healthy configured production provider and execute successful subsequent work. Prove bounded retry/recovery, no duplicate accepted work and no stale provider identity reused. A ready flag without a real healthy result is insufficient.
3. Confirm real failure triggers reached provider invocation. Negative fixtures reject supplied failure flags, static reference-trace metadata, absent execution logs, wrong process/request identity and timeout classifications without elapsed/deadline evidence.
4. Exercise loss, timeout and interruption separately; do not infer one from another. Preserve statuses, cancellation, truncation and redaction at adapter boundaries. Where WSS failure-event evidence is consumed, require the reviewed #852 output rather than reimplementing or claiming it here; the exact graph remains RT-PROVIDER as this task's execution prerequisite, with full cross-producer joining in QUAL-EVIDENCE.
5. Use only task-owned processes and the explicitly approved environment; never kill shared resident/provider services. Independently review exact source/run/profile and all scenario evidence. Local mock transport proof is labeled separately and cannot replace required production provider execution.

- Acceptance: actual_process_loss, actual_timeout_interrupt, healthy_recovery, actual_execution_receipts, independent_review.
- PVF: actual_process_loss, actual_timeout_interrupt, healthy_recovery, caller_failure_flags_not_proof, reference_trace_not_execution, provider_failure_recovery.

## Inputs

Full live source contract, retained without dropping requirements:

# [v0.92.2][Runtime qualification][QUAL-PROVIDER] Execute provider loss, interruption and recovery qualification

## One complete result

Induce real provider subprocess loss, timeout and interruption through the production provider harness and demonstrate bounded recovery with a healthy configured provider. This task qualifies actual effects and recovery; caller-set failure flags and named reference traces are not execution proof.

## Owned implementation and proof paths

Own bounded qualification harness/test changes around `adl-runtime-kernel/tests/governed_operations.rs` and its modules, the actual provider adapters they invoke, and `adl-runtime/src/qualification/mod.rs` when used by the production proof path. Re-resolve the current registered test names before running: the historical `parity_c_live_governance::real_provider_process_loss_timeout_and_recovery_are_classified` name is a source-recovery starting point, not a verified current test. `adl-runtime-kernel/src/conversation_sessions_tests.rs::resident_agent_conversation_uses_canonical_agent_runtime_wss_ingress` is current ingress regression context. Never count a zero-match exact filter as proof. Keep #852's dispatch event implementation separately owned.

## Executed acceptance

1. RT-PROVIDER/#855 must deliver its registered-provider lifecycle before this qualification. Select a scoped owned test subprocess/session through that production route, observe actual start, kill/loss, deadline timeout and cancellation/interruption, and retain process/request identity, timestamps, exit state, Runtime outcome and correlation.
2. Restore service through a healthy configured production provider and execute successful subsequent work. Prove bounded retry/recovery, no duplicate accepted work and no stale provider identity reused. A ready flag without a real healthy result is insufficient.
3. Confirm real failure triggers reached provider invocation. Negative fixtures reject supplied failure flags, static reference-trace metadata, absent execution logs, wrong process/request identity and timeout classifications without elapsed/deadline evidence.
4. Exercise loss, timeout and interruption separately; do not infer one from another. Preserve statuses, cancellation, truncation and redaction at adapter boundaries. Where WSS failure-event evidence is consumed, require the reviewed #852 output rather than reimplementing or claiming it here; the exact graph remains RT-PROVIDER as this task's execution prerequisite, with full cross-producer joining in QUAL-EVIDENCE.
5. Use only task-owned processes and the explicitly approved environment; never kill shared resident/provider services. Independently review exact source/run/profile and all scenario evidence. Local mock transport proof is labeled separately and cannot replace required production provider execution.

## Dependencies and preserved scope

Execution prerequisites: RT-PROVIDER, with accepted merged producer outputs before dependent proof. QUAL-RUNTIME is existing #852 and remains the actual dispatch failure-event repair; RT-PROVIDER is existing #855 and retains its full five-route lifecycle matrix. Neither is recreated here. Qualification does not claim every #855 route is exercised unless recorded scenario coverage proves it.

The operator split the former #852 aggregate into the repair plus four complete follow-ons. Preserve the five criterion IDs, original 19-finding denominator, separate five cloud-control and two execution-proof gaps, and closed #522/#833 historical-consumer boundaries. QUAL-EVIDENCE joins results; it cannot absorb unfinished repairs or unexecuted scenarios. Retained partial #851 work on `codex/851-reconcile-five-proof-links` was unreviewed/unmerged at source capture: preserve its bytes and resolve actual owner/state before reuse.

## Evidence limits that must remain explicit

Historical #268 Run72 at `8947231b6549f6b43f76c5d4656b333868a032ae` is real past execution, with detailed population/restore receipts privately retained; it is not current-candidate acceptance. #341's trace is local-reference-only and rejects supplied flags before provider invocation. #602 retains a reviewed Wuji summary with `observability_not_ready`, not complete event/response proof. Existing focused component tests and historical 50-to-54 source declarations do not close the operational claims. Re-resolve source facts before execution without rewriting historical evidence.

## Required specification obligations

- Acceptance: actual_process_loss, actual_timeout_interrupt, healthy_recovery, actual_execution_receipts, independent_review.
- PVF: actual_process_loss, actual_timeout_interrupt, healthy_recovery, caller_failure_flags_not_proof, reference_trace_not_execution, provider_failure_recovery.

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
Execution prerequisite: #855 (RT-PROVIDER); accepted output is required before dependent execution.

Reviewed creation source: `b31eb6904d66bdb006b6eaeda6cdcd5f2a401260`. This issue records a complete task; creation does not claim execution or acceptance.


<!-- csdlc-v3-operation:c4b507df4bce97d379f65ac081da01222073a4452e7c14041f834ebe578dc87e -->

## Target Files / Surfaces

Own new, non-overlapping issue paths: `adl/tools/run_issue901_provider_recovery_qualification.py`, `adl/tools/test_run_issue901_provider_recovery_qualification.py`, and `docs/milestones/v0.92.2/evidence/qual-provider-901/`. Invoke the merged production provider adapter and registered-provider lifecycle read-only through their public command/configuration surfaces. Do not edit the #851-owned dirty files `adl-runtime-kernel/tests/governed_operations.rs`, `adl-runtime-kernel/src/conversation_sessions_tests.rs`, `adl-runtime-kernel/src/ingress.rs`, `adl-runtime-kernel/src/telemetry.rs`, or `adl/src/long_lived_agent/tests.rs`.

## Validation Plan

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

Source-grounded planned registration check: `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test governed_operations -- --list`; select exact nonzero registered cases only after inspecting their production effects. Current source contains in_flight_cancellation_kills_provider_and_releases_capacity and provider_timeout_auth_quota_and_malformed_output_are_classified, but the latter supplies condition flags and is not real timeout proof. Historical parity_c_live_governance::real_provider_process_loss_timeout_and_recovery_are_classified is absent from current main. Implement or recover reviewed actual loss/timeout/interruption/recovery harness coverage before claiming qualification. WSS ingress context exists in conversation_sessions_tests.rs, not proof of full provider recovery.

Formatting and touched-owner regressions use the manifest of each actually changed Rust component. Enumerate actual tests and replan absent/renamed ones explicitly; zero-test filters never prove acceptance. These are future execution commands, not tests run during preparation.

`git diff --check` accompanies focused proof. Required qualification harness repairs and scenario proofs are future work; existing source alone is not acceptance. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Demo / Proof Requirements

Classify new tests at authoring time with lane, proof role, determinism, resource profile and release-gate status in a tightly coupled manifest. Local deterministic contract negatives use isolated copies, controlled clocks and owned subprocesses; role is evidence-integrity/behavior regression, local CPU/Rust/Python/Git, required issue gate. Run the authored focused tests and actual tool/harness invocations with nonzero scenario counts; list exact registered Rust test names first rather than trusting historical filters. Run formatting and relevant shared-owner regressions when source changes. Warm only trusted same-host dependency artifacts; distinguish local evidence from required CI.

Resident/provider qualification additionally requires the declared production execution lane in an explicitly approved environment. Before effects, pin candidate/runtime/provider/model/artifact/configuration identities, process ownership, bounds, duration/cost, credential reference and the scope of operator authorization. This creation does not authorize paid cloud/GPU mutation, account changes, service disruption or provider charges. If approved execution cannot be obtained, record not-proven and keep the task incomplete; do not replace it with mock proof. Any ADL AWS observation uses `agent-logic-admin` with verified business identity; credentials and account secrets never enter artifacts. No broad host process scans or killing shared services.

Source-grounded planned registration check: `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test governed_operations -- --list`; select exact nonzero registered cases only after inspecting their production effects. Current source contains in_flight_cancellation_kills_provider_and_releases_capacity and provider_timeout_auth_quota_and_malformed_output_are_classified, but the latter supplies condition flags and is not real timeout proof. Historical parity_c_live_governance::real_provider_process_loss_timeout_and_recovery_are_classified is absent from current main. Implement or recover reviewed actual loss/timeout/interruption/recovery harness coverage before claiming qualification. WSS ingress context exists in conversation_sessions_tests.rs, not proof of full provider recovery.

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

#855 is satisfied: PR #964 merged and issue #855 closed, delivering the registered-provider lifecycle. #852 is also accepted through merged PR #963 for any WSS failure-event evidence consumed. #851 remains dirty and unmerged in its separate registered worktree; its bytes are preserved and its paths are excluded. Own new, non-overlapping issue paths: `adl/tools/run_issue901_provider_recovery_qualification.py`, `adl/tools/test_run_issue901_provider_recovery_qualification.py`, and `docs/milestones/v0.92.2/evidence/qual-provider-901/`. Invoke the merged production provider adapter and registered-provider lifecycle read-only through their public command/configuration surfaces. Do not edit the #851-owned dirty files `adl-runtime-kernel/tests/governed_operations.rs`, `adl-runtime-kernel/src/conversation_sessions_tests.rs`, `adl-runtime-kernel/src/ingress.rs`, `adl-runtime-kernel/src/telemetry.rs`, or `adl/src/long_lived_agent/tests.rs`. Operator-approved task-owned local macOS provider qualification: isolated loopback endpoints and only subprocesses started by the #901 harness; existing local model artifacts may be used without download. No paid/cloud services, shared Ollama mutation, account change, or broad host process control. The harness must record exact adapter/provider/model/configuration identity, request identity, PIDs it owns, timestamps, deadline/elapsed evidence, exit state, recovery result, and duplicate/stale-identity checks. Execution and acceptance remain pending until the bound issue goal and proving runs complete.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
