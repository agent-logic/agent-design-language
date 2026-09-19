# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1079
Run ID: issue-1079
Version: 1.0.5
Title: [v0.92.2][Runtime][OpenRouter] Honor reasoning effort for DeepSeek review turns
Branch: codex/1079-deepseek-openrouter-reasoning
Card Status: ready
Generated: 2026-09-18T23:46:34Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1079
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/1079
- Docs: docs/provider/inference-profiles.md and current Runtime provider/admission contracts
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
  output_card: .csdlc/issues/1079/cards/sor.md
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
- Source issue-prompt slug: deepseek-openrouter-reasoning
- Required outcome type: runtime_provider_behavior_and_live_proof
- Demo required: true

## Goal

Make OpenRouter reasoning effort explicit and model-specific so DeepSeek V4 Flash completes substantive Runtime reviews without changing other OpenRouter residents.

## Required Outcome

OpenRouter consumes normalized reasoning_effort, DeepSeek uses a separate low-effort bounded profile, provider failures remain typed at conversation boundaries, and installed Runtime proof covers a full issue review plus governed A2A continuation.

## Acceptance Criteria

Capture low reasoning on the DeepSeek request; reject conflicting controls before dispatch; complete an untruncated full-issue review and A2A continuation; preserve Nexus and Nemotron profiles; distinguish timeout, invalid-response, quota, transport, and cancellation failures.

## Inputs

Issue #1079; adl-provider-core OpenRouter codec/adapter; Runtime ingress/control conversation paths; docs/provider/inference-profiles.md; merged provider lifecycle and hot-reload contracts.

## Target Files / Surfaces

adl-provider-core/src/provider_substrate.rs; adl-provider-core/src/http_family.rs; adl-provider-core/src/http_family/tests.rs; adl-runtime-kernel/src/ingress.rs; adl-runtime-kernel/src/control.rs; adl-runtime-kernel/src/conversation_sessions_tests.rs; docs/provider/inference-profiles.md; installed local Runtime provider definition and retained redacted proof.

## Validation Plan

Run focused provider request-capture and invalid-control tests, conversation-level failure projection tests, provider-core suite and strict Clippy, Runtime focused tests, native card validation, independent exact-head review, then one installed full-issue review and one governed A2A exchange.

## Demo / Proof Requirements

Use DeepSeek V4 Flash through the installed Runtime with low reasoning, bounded output and timeout. Retain model identity, non-secret controls, elapsed time, finish/status, response presence, typed failures, and peer continuation. Do not retain hidden reasoning.

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

No global shared OpenRouter budget change, recurring inference, model replacement, credential or billing change, PAIR change, or unrelated Runtime repair.

## Notes / Risks

Real-provider proof is bounded and single-attempt after ambiguous outcomes. Preserve last-known-good provider state and keep Nexus/Nemotron effective profiles unchanged.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
