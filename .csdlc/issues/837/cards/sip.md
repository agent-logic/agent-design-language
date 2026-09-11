# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0837
Run ID: issue-0837
Version: v0.92.1
Title: [v0.92.1][TAIL-06.22][architecture] Publish active boot paths and retire stale control-plane guidance
Branch: codex/837-active-boot-paths-control-plane-guidance
Card Status: ready
Generated: 2026-09-10T23:00:00Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/837
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/837
- Docs: docs/milestones/v0.92.1/evidence/release/tail-04
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
  output_card: .csdlc/issues/837/cards/sor.md
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
- Source issue-prompt slug: active-boot-paths-control-plane-guidance
- Required outcome type: architecture
- Demo required: false

## Goal

Publish and enforce one source-backed active boot path per subsystem and retire stale ordinary C-SDLC v2 guidance.

## Required Outcome

Active docs, CLI help, selector, installed layout, and tests agree that native v3 is the sole ordinary lifecycle path while product/runtime and rollback-only surfaces are correctly classified.

## Acceptance Criteria

Exactly one ordinary lifecycle path; ADL and Runtime correctly classified; retained v2 has no ordinary route; docs/help/selector/layout/tests agree; focused validator and exact-head review pass.

## Inputs

Issue #837; parent #522; TPR-005; #505 / PR #591 authority cutover.

## Target Files / Surfaces

Current lifecycle/boot-path documentation, focused source-backed inventory, and issue-837 enforcement artifacts only.

## Validation Plan

Derive the boot-path table from source and selectors, repair only stale current guidance, and run a focused validator with negative ambiguity fixtures.

## Demo / Proof Requirements

Deterministic source-backed inventory and negative stale-guidance fixture; no live demo required.

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

No deletion of retained rollback source, authority change, Runtime/lifecycle combination, or unrelated docs cleanup.

## Notes / Risks

Historical evidence and explicit rollback docs must remain allowed; broad scans must not confuse product binaries with lifecycle generations.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
