# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.4/sip.md`

Task ID: issue-0758
Run ID: issue-0758
Version: 1.0.4
Title: [v0.92.1][TAIL-06.02][runtime] Persist and recover admission-triggered A2A initiation
Branch: codex/758-admission-a2a-outbox
Card Status: ready
Generated: 2026-09-09T00:00:00Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/758
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/758
- Docs: Internal review #520 findings A520-ARCH-002 and C520-CODE-002; parent remediation ledger #522
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
  output_card: .csdlc/issues/758/cards/sor.md
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
- Source issue-prompt slug: admission-a2a-outbox
- Required outcome type: runtime_behavior_and_recovery_proof
- Demo required: true

## Goal

Make every successful dynamic-agent admission durably own the required autonomous A2A greeting until a visible terminal disposition exists.

## Required Outcome

Persist an idempotent admission-triggered initiation intent before admission completes; recover and retry it after refusal, interruption, restart, or repeated admission; deduplicate against the conversation ledger; and expose pending, retrying, completed, and terminal-failure states.

## Acceptance Criteria

- Admission durably records the greeting obligation before acknowledging terminal workflow completion.
- Startup and already-present admission replay resume pending work.
- Refusal and interruption cannot silently suppress initiation.
- Retry is bounded and shares a stable idempotency key with the conversation ledger.
- Health/evidence exposes all lifecycle dispositions.
- A real autonomous demo completes without operator prompting or duplicate greetings.

## Inputs

- GitHub issue #758.
- Internal review #520 findings A520-ARCH-002 and C520-CODE-002.
- Existing A2A transport, dynamic admission, runtime persistence, health, and conversation-ledger contracts.

## Target Files / Surfaces

- adl-runtime-kernel/src/control.rs
- Narrowly coupled runtime persistence/outbox and health projection modules
- Focused Runtime tests and PVF inventory entries
- Issue-local proof artifacts and lifecycle cards

## Validation Plan

Run targeted Rust tests for durable creation, restart recovery, refusal retry, interruption recovery, repeated-admission rescheduling, duplicate suppression, and terminal status projection; then run the focused Runtime owner lane and the autonomous real-agent demo required by #758.

## Demo / Proof Requirements

Admit a real agent, force the first greeting attempt to refuse or be interrupted, restart the Runtime, and prove one autonomous deduplicated greeting reaches a terminal ledger result without an operator prompt.

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

- No exactly-once provider-boundary guarantee.
- No general workflow engine.
- No unrelated A2A transport redesign.

## Notes / Risks

Crash windows must be closed at the admission commit boundary. Retry must be bounded, deterministic, and unable to emit duplicate conversation turns after uncertain completion.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run `csdlc-bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
