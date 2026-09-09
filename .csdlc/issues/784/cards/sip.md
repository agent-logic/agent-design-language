# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.4/sip.md`

Task ID: issue-0784
Run ID: issue-0784
Version: 1.0.4
Title: [v0.92.1][Runtime] Return completed A2A replies to the initiating agent
Branch: codex/784-runtime-a2a-closed-loop
Card Status: ready
Generated: 2026-09-09T16:40:00Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/784
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/784
- Docs: Live cooperative Runtime testing on Wuji
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
  output_card: .csdlc/issues/784/cards/sor.md
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
- Source issue-prompt slug: runtime-a2a-closed-loop
- Required outcome type: runtime_behavior_and_focused_proof
- Demo required: true

## Goal

Return a completed governed peer result to the initiating agent's next model turn without operator relay.

## Required Outcome

Project each completed A2A success or typed terminal failure into the initiating conversation's provider context exactly once per prompt with its causal identifiers preserved.

## Acceptance Criteria

- An agent delegates to a peer and receives the peer result in its next model turn.
- The peer result appears once with stable conversation, turn, correlation, and work identifiers.
- Typed refusal, failure, cancellation, and timeout dispositions remain available.
- Existing history, checkpoint, and restore behavior is preserved.
- The implementation is uniform for all agents and has no Shepherd-specific branch.

## Inputs

- GitHub issue #784.
- Existing governed A2A result and conversation-history structures.
- Live Nova/Lattice and Relay/Signal cooperation evidence.

## Target Files / Surfaces

- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/assembly.rs
- Focused Runtime conversation/A2A tests

## Validation Plan

Run the focused three-call provider test proving delegate, peer reply, and initiator synthesis; run adjacent A2A and conversation-continuity tests; obtain bounded subagent review.

## Demo / Proof Requirements

On the live Runtime, one resident agent delegates to another and then summarizes the returned peer response without the operator copying it back.

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

- No canonical-name routing change (#718).
- No cross-node transport work.
- No Shepherd identity repair.
- No provider-budget changes.

## Notes / Risks

Do not add a parallel protocol. Reuse persisted terminal A2A fields and keep peer results bounded and causally labeled.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run `csdlc-bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
