# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0894
Run ID: issue-0894
Version: 0.92.2
Title: [v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings
Branch: codex/894-v0922-test-planner
Card Status: ready
Generated: 2026-09-12T00:10:09.925246+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/894
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/894
- Docs: AGENTS.md; docs/csdlc-v3/CURRENT_AUTHORITY.md; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; docs/milestones/v0.92.2/SPRINT_v0.92.2.md; docs/milestones/v0.92.2/features/MULTI_PERSPECTIVE_REVIEW_v0.92.2.md; docs/milestones/v0.92.2/features/GOVERNED_PUBLICATION_v0.92.2.md
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
  output_card: .csdlc/issues/894/cards/sor.md
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
- Source issue-prompt slug: v0922-test-planner
- Required outcome type: implementation_and_executed_proof
- Demo required: true

## Goal

The test planner consumes synthesized findings and maps each selected finding to a concrete behavior, test location, fixture and expected failure without changing source.

Dependencies: CF-SYNTHESIS. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Required Outcome

The test planner consumes synthesized findings and maps each selected finding to a concrete behavior, test location, fixture and expected failure without changing source.

Dependencies: CF-SYNTHESIS. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Acceptance Criteria

1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files.
2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review.
3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings.
4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient.

acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`.

pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.

## Inputs

Full live source contract, retained without dropping requirements:

# [v0.92.2][CF-TESTPLAN] Generate a bounded test plan from review findings

## One complete result

The test planner consumes synthesized findings and maps each selected finding to a concrete behavior, test location, fixture and expected failure without changing source.

Dependencies: CF-SYNTHESIS. Consume accepted merged predecessor output; logical IDs receive canonical issue numbers during creation. Batch membership adds no all-to-all gate.

## Production ownership

Implement `plan tests` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: actions/test_plan.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_testplan.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Acceptance and executed evidence

1. Consume actual CF-SYNTHESIS output and generate a readable test plan mapping every selected finding to behavior under test, source evidence, concrete proposed test location, fixture/input, expected failure before a fix, expected result after it and suitable assertion. Distinguish proposed paths from existing files.
2. Explain why each case detects the reported behavior; reject tests that merely mirror implementation or validate an unrelated schema. Record missing test infrastructure, nondeterminism and unsupported claims explicitly. Include validation lane, role and resource requirements suitable for the repository under review.
3. Run production generation and reader on predecessor output plus known findings; independently check useful behavior/fixture/assertion mapping. Reject missing evidence, incompatible runs, vague test placeholders and unmapped selected findings.
4. Prove generation changes no inspected source and creates no issue or test file in that repository. The product supplies a complete actionable plan; test implementation remains human-controlled. A canned packet or plan-only implementation is insufficient.

## Shared execution boundary

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## PVF and completion

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

## Inherited obligation ledger

acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`.

pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.


## Canonical execution links

Planning owner: #864. Creation/review batch: 4; this grouping adds no execution gate.
Execution prerequisite: #892 (CF-SYNTHESIS); accepted output is required before dependent execution.

Reviewed creation source: `54e5d8e100f39c644ca0aa03e3985ce66beb529e`. This issue records a complete task; creation does not claim execution or acceptance.


<!-- csdlc-v3-operation:982d5a8ecfa8cf361e7bec1c51dbacacba009f8d326e7c9d710f883eeac809d9 -->

## Target Files / Surfaces

Implement `plan tests` behavior under the selected `adl codefriend` entrypoint. Proposed cohesive modules under `adl/src/codefriend/`: actions/test_plan.rs. The CLI registration is `adl/src/cli/codefriend_cmd.rs` (CF-SHELL owns operator-control additions); coordinate shared wiring with predecessor owners. Add focused `adl/tests/codefriend_testplan.rs` and bounded fixture outputs. Exact flags are documented/tested during implementation; command wording denotes the selected behavior, not an already installed command.

## Validation Plan

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

Planned focused commands after the selected new suite is implemented: `cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan`; `cargo fmt --manifest-path adl/Cargo.toml --check`; focused touched CLI/admission-owner regressions selected after merged prerequisites. These do not replace actual installed generator/reader or publication admission execution. Re-resolve suite names after prerequisites land and update VPP for changes; never treat a zero-test filter as proof.

`git diff --check` accompanies focused proof. Selected new test files do not exist yet; their commands are future proving work, not tests run during preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

## Demo / Proof Requirements

Declare each new test in a coupled proof inventory: deterministic local CPU contract/integration lane, isolated source/store and controlled clocks/transport, nonzero success and negative proof, bounded disk/process resources, required Beta feature and CF-INTEGRATE input gate. Provider-generated runs and browser/PDF visual review are separately identified execution/observation evidence, not deterministic guarantees. Run focused installed CLI and consumer tests plus required CI, then independent exact-head review. Redacted human diagnostics stay on stderr and machine output on stdout; test compatibility logging when exposed. Stop on unresolved owner/authority, inconsistent scope/provenance, failed or missing proof, source/credential exposure, stale approval or partial completion claimed as success. No unrelated feature work or customer-scale hosting.

Planned focused commands after the selected new suite is implemented: `cargo test --manifest-path adl/Cargo.toml --test codefriend_testplan`; `cargo fmt --manifest-path adl/Cargo.toml --check`; focused touched CLI/admission-owner regressions selected after merged prerequisites. These do not replace actual installed generator/reader or publication admission execution. Re-resolve suite names after prerequisites land and update VPP for changes; never treat a zero-test filter as proof.

`git diff --check` accompanies focused proof. Selected new test files do not exist yet; their commands are future proving work, not tests run during preparation. Install the issue candidate through the approved installer only at execution with current provenance; do not replace a shared binary during preparation.

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

acceptance: `finding_to_behavior_trace`, `concrete_test_plan_consumed`.

pvf: `finding_to_behavior_trace`, `concrete_test_plan_consumed`, `implementation_mirroring_rejected`, `source_mutation_rejected`, `action_plan_schema`.

stop_conditions: `missing_required_input`, `required_proof_failed`, `scope_or_contract_conflict`, `required_proof_not_executed`, `partial_artifact_claimed_complete`, `source_mutation`, `test_without_behavior_mapping`.

non_goals: `unrelated_scope`, `schema_or_scaffold_only_completion`, `security_tournament`, `autonomous_fixing`.

Completion rejection: `plan_only`, `schema_only`, `scaffold_only`, `test_only_consumer`, `zero_executed_scenarios`.

Canonical source: `docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `WP_ISSUE_WAVE_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, and `ADOPTED_DESIGN_CONTRACTS_v0.92.2.md`.

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

## Notes / Risks

Use `docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md` and the adopted design contract: local `adl codefriend` in this repository, selected macOS/Linux qualification, shared registered OpenAI route with operator-supplied credential reference, and pinned Vector `410da89a0ed42c523143da89fffeb7f6402833e0` scope (seven dnsmsg-parser files plus three root manifest/license files; ten files/600 KiB total/400 KiB per file). Preserve both license notices and read-only source. Existing `adl/src/cli/mod.rs`, `cli/usage.rs` and `lib.rs` are registration owners. New paths are planned implementation, not a claim of existing product behavior. Preserve CF-EVIDENCE's versioned run/evidence/finding/publication semantics and immutable evidence; reuse its canonical test vectors and predecessor artifacts.

Native v3 readiness, a dedicated bound FastWork worktree, an issue-bound goal and current path ownership precede implementation. Supporting tests, error handling and user documentation are part of this task. Record exact candidate/input/output identity and local versus required CI results. No schema/scaffold/unused helper/test-only caller/zero-execution result can close it. No autonomous source changes, paid calls, external publication or cloud resources are authorized merely by creation. Required real provider proof needs scoped execution authority and cannot be replaced by synthetic success.

Live dependency read on 2026-09-16 confirms #892 (CF-SYNTHESIS) is closed/accepted. Before bind, recheck path ownership against active CodeFriend PRs/worktrees and preserve #893/#895 sibling boundaries.

Prepared, not bound. Execution Sprint 4 is a scheduling assignment, not dependency satisfaction. All implementation steps, proving runs, implementation review, PR and terminal state remain unstarted. Shared CLI dispatch/library, fixture directories and evidence/publication admission surfaces require current owner coordination after predecessors land.

Branch codex/894-v0922-test-planner and its proposed FastWork path are unbound planning values only. Issue #926 owns umbrella management for all eleven sprints; it is not the Sprint 4 umbrella or a child execution prerequisite.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
