# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.4/sip.md`

Task ID: issue-0717
Run ID: issue-0717
Version: 1.0.4
Title: [v0.92.1][Runtime] Teach admitted agents about Polis modules and capabilities
Branch: codex/717-polis-capability-orientation
Card Status: ready
Generated: 2026-09-09T18:30:00Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/717
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/717
- Docs: Operator-promoted v0.92.1 Runtime bugfix lane
- Other: none

## Agent Execution Rules
- This issue is not started yet; do not assume a branch or worktree already exists.
- Do not use v1 wrappers; bind execution with `csdlc-bind` only if execution later becomes necessary.
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
  output_card: .csdlc/issues/717/cards/sor.md
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
- Source issue-prompt slug: polis-capability-orientation
- Required outcome type: runtime_behavior_docs_and_focused_proof
- Demo required: false

## Goal

Give every newly admitted agent a concise, source-grounded map of Polis capabilities before its first model turn.

## Required Outcome

Expand the versioned Welcome Package and validate its capability inventory against the Runtime's canonical operational services while preserving digest provenance and non-authority boundaries.

## Acceptance Criteria

- First-turn orientation covers identity/offices, provider reasoning, ACIP and A2A, UTS and ACC, Freedom Gate, AEE, continuity, memory, observability, Shepherd coordination, scheduling/time, and escalation.
- Agents are told to use admitted Runtime contracts, not internal Rust modules.
- Deterministic validation rejects missing, stale, duplicate, or invented capability inventory entries.
- Capability existence is distinguished from deployment enablement, agent admission, and per-action authorization.
- Existing version and digest delivery remains intact.

## Inputs

- GitHub issue #717.
- Runtime operational adapter registry and architecture/explainer docs.
- Existing #708/#709 orientation resource and delivery path.
- Operator decision promoting #717 and #718 into v0.92.1.

## Target Files / Surfaces

- docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md
- adl-runtime-kernel/src/agent_orientation.rs
- v0.92.1 and v0.92.2 planning reconciliation docs
- Focused Runtime tests

## Validation Plan

Run focused agent_orientation tests, adjacent first-turn delivery tests, formatting, diff hygiene, deterministic inventory-negative tests, and bounded subagent review.

## Demo / Proof Requirements

No live provider or cloud proof; deterministic injected-context and inventory validation is sufficient.

## Constraints / Policies

- Follow `AGENTS.md`.
- Use the typed C-SDLC v2 operator skills and Rust binaries for lifecycle routing.
- Edit cards only with editor skills.
- Work only in the bound issue worktree after `csdlc-bind`.
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

- No new authority or tools.
- No behavior changes to Freedom Gate, ACIP, UTS, ACC, AEE, continuity, providers, or scheduling.
- No #718 canonical-name routing implementation.
- No live Runtime mutation.

## Notes / Risks

Keep model context compact. The inventory must describe externally meaningful capability families without presenting deployment-optional services as currently enabled.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run `csdlc-bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
