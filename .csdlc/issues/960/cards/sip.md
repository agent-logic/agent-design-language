# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0960
Run ID: issue-0960
Version: 1.0.5
Title: Fix Runtime shutdown barrier acknowledgment race (v0.92.2)
Branch: codex/960-shutdown-barrier
Card Status: ready
Generated: 2026-09-12T05:59:59.590095+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/960
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/960
- Docs: Not applicable to this bounded Runtime regression.
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
  output_card: Not applicable to this bounded Runtime regression.
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
- Source issue-prompt slug: 960-shutdown-barrier
- Required outcome type: runtime regression repair
- Demo required: false

## Goal

Fix event-specific Runtime shutdown sink acknowledgment under concurrent heartbeats; preserve fail-closed sink errors and retain daemon exit/stderr in smoke failures. Separate regression #960, part of #928, diagnosed while PR #957 was red; original CI cause remains unproven.

## Required Outcome

Fix event-specific Runtime shutdown sink acknowledgment under concurrent heartbeats; preserve fail-closed sink errors and retain daemon exit/stderr in smoke failures. Separate regression #960, part of #928, diagnosed while PR #957 was red; original CI cause remains unproven.

## Acceptance Criteria

Exact barrier acknowledgment survives subsequent heartbeat updates; required sink write failures still reject shutdown; deterministic concurrent-writer and failure injection tests plus actual local CLI notice/disposition proof; stderr/exit retained. No sleeps, longer timeouts, weakened assertions or CodeFriend changes.

## Inputs

Not applicable to this bounded Runtime regression.

## Target Files / Surfaces

adl/src/long_lived_agent.rs; adl/src/cli/observability.rs (also included by adl/src/observability.rs); adl/tests/cli_smoke/agent.rs; focused proof under .csdlc/evidence/960.

## Validation Plan

Exact barrier acknowledgment survives subsequent heartbeat updates; required sink write failures still reject shutdown; deterministic concurrent-writer and failure injection tests plus actual local CLI notice/disposition proof; stderr/exit retained. No sleeps, longer timeouts, weakened assertions or CodeFriend changes.

## Demo / Proof Requirements

Actual local CLI shutdown with notice and disposition; no paid calls.

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

No CodeFriend changes, paid calls, longer waits or weaker assertions; no claim that original CI cause is proven.

## Notes / Risks

Supported heartbeat race proven; original CI cause unknown. Local sink acknowledgment is not remote OTLP delivery.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
