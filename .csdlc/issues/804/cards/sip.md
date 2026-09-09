# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.4/sip.md`

Task ID: issue-0804
Run ID: issue-0804
Version: v0.92.1
Title: [v0.92.1][defect] Align active SOR template authority notice with C-SDLC v3
Branch: codex/804-sor-authority-notice
Card Status: ready
Generated: 2026-09-09T18:25:45.944998+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/804
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/804
- Docs: docs/csdlc-v3/CURRENT_AUTHORITY.md
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
  output_card: .csdlc/issues/804/cards/sor.md
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
- Source issue-prompt slug: sor-authority-notice
- Required outcome type: documentation
- Demo required: false

## Goal

Align active SOR template authority notice with operational C-SDLC v3.

## Required Outcome

New native SOR renders state current v3 authority and pass active structure validation.

## Acceptance Criteria

Active SOR template and schema match v3 authority; fresh native render validates; unrelated/historical cards and runtime unchanged.

## Inputs

Active registry authority notice and SOR template/schema.

## Target Files / Surfaces

docs/templates/prompts/1.0.4/sor.md and schemas/sor.structure.json; issue-local cards and bounded proof.

## Validation Plan

Native six-card render/structure validation, schema parity for changed SOR, Python schema smoke check, negative stale-notice check. No broad runtime tests.

## Demo / Proof Requirements

Native rendered SOR and focused schema proof; no product demo required.

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

No historical template versions, unrelated issue cards, runtime behavior or #759 changes.

## Notes / Risks

Historical cards retain source-time text; no historical rewrite or runtime authority change.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run `csdlc-bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
