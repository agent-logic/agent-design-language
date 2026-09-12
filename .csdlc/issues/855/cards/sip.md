# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0855
Run ID: issue-0855
Version: v0.92.2
Title: [v0.92.2][RT-PROVIDER] Provider-neutral dynamic agent lifecycle
Branch: codex/855-provider-neutral-lifecycle
Card Status: ready
Generated: 2026-09-11T23:55:22.557345+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/855
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/855
- Docs: docs/milestones/v0.92.2/
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
  output_card: .csdlc/issues/855/cards/sor.md
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
- Source issue-prompt slug: 855-provider-neutral-lifecycle
- Required outcome type: production-behavior
- Demo required: yes

## Goal

Allow an operator to add, inspect, communicate with, checkpoint, migrate, rehydrate, and remove an agent backed by any provider adapter registered with the Runtime, without restarting the Runtime or editing its canonical initialization file. Dynamic agent lifecycle operations dispatch through the Runtime's provider registry and capability contract rather than provider-name match statements. Provider-specific authentication, endpoint validation, request execution, tool capability, streaming, accounting, and failure classification remain owned by the selected provider adapter. - Hard-coding only the four providers used by the initial acceptance demonstration. - Embedding provider secrets in agent configuration. - Requiring all providers to support identical optional capabilities. - Restarting Runtime merely to add, replace, or remove an agent. - Changing subscription or cloud billing configuration.

## Required Outcome

Allow an operator to add, inspect, communicate with, checkpoint, migrate, rehydrate, and remove an agent backed by any provider adapter registered with the Runtime, without restarting the Runtime or editing its canonical initialization file. Dynamic agent lifecycle operations dispatch through the Runtime's provider registry and capability contract rather than provider-name match statements. Provider-specific authentication, endpoint validation, request execution, tool capability, streaming, accounting, and failure classification remain owned by the selected provider adapter. - Hard-coding only the four providers used by the initial acceptance demonstration. - Embedding provider secrets in agent configuration. - Requiring all providers to support identical optional capabilities. - Restarting Runtime merely to add, replace, or remove an agent. - Changing subscription or cloud billing configuration.

## Acceptance Criteria

- `csmctl agent add --config <agent.yaml>` accepts every provider registered with the running Runtime and rejects only an unknown provider or a provider whose declared capability requirements are unsatisfied. - Adding or replacing a dynamic agent does not require a Runtime or Guardian restart and does not require editing the Runtime initialization file. - OpenAI/ChatGPT, Anthropic/Claude, Gemini through the appropriate Google provider route, Ollama local, and an OpenAI-compatible local endpoint each pass add, generated conversation, roster projection, checkpoint, removal, and rehydration tests. - Agent-to-agent communication uses the same provider-neutral execution path and canonical agent names for every admitted provider. - Provider adapters declare whether tools, streaming, model discovery, token accounting, and health checks are supported; the Runtime does not infer these capabilities from provider names. - Hosted providers require HTTPS and approved credential references. Local plaintext endpoints remain limited to loopback, private, or explicitly trusted local-network bindings. - Admission performs no recurring paid inference. Health checks and model validation follow provider capabilities and the metered-call safeguards tracked by #854. - Provider failures retain actionable classifications such as credentials, quota, unsupported capability, model unavailable, transport, timeout, and invalid response. - The API and Observatory report the effective provider/model and current capability/readiness state without exposing credentials. - Deterministic tests prove that a newly registered fixture provider works without modifying Runtime admission or conversation match statements.

## Inputs

Source issue, canonical v0.92.2 contracts and accepted dependency outputs.

## Target Files / Surfaces

- `adl-runtime-kernel/src/control.rs` - `adl-runtime-kernel/src/config.rs` - `adl/src/cli/csmctl_cmd.rs` - Existing provider adapter and capability architecture - `docs/architecture/PROVIDER_CAPABILITY_AND_TRANSPORT_ARCHITECTURE.md` - Issue #602 dynamic agent lifecycle behavior - Issue #854 metered cloud inference safeguards

## Validation Plan

Execute every proving case in the source issue; nonzero production scenarios, focused negative tests, independent exact-head review and required CI.

## Demo / Proof Requirements

Execute and retain the complete acceptance/proving cases in the source issue; no schema-only or fixture-only substitution.

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

- Hard-coding only the four providers used by the initial acceptance demonstration. - Embedding provider secrets in agent configuration. - Requiring all providers to support identical optional capabilities. - Restarting Runtime merely to add, replace, or remove an agent. - Changing subscription or cloud billing configuration.

## Notes / Risks

Prepared only. Child implementation and its review have not run.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
