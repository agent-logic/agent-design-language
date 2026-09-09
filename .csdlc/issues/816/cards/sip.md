# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0816
Run ID: issue-0816
Version: v0.92.1
Title: [v0.92.1][TAIL-06.11][quality] Repair redaction and hot-reload validation integrity
Branch: codex/816-validation-integrity
Card Status: ready
Generated: 2026-09-09T20:50:00Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/816
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/816
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
  output_card: .csdlc/issues/816/cards/sor.md
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
- Source issue-prompt slug: validation-integrity
- Required outcome type: quality
- Demo required: false

## Goal

Replace vacuous or scheduler-dependent validation with deterministic proof for Observatory redaction and Runtime hot-reload cancellation.

## Required Outcome

Both validation surfaces prove the intended security and cancellation behavior deterministically across repeated focused runs.

## Acceptance Criteria

Redaction validates actual publication-shaped Runtime/UI/evidence data and rejects representative secrets and provider payloads; cancellation tests wait for watcher receipt and pending debounce before revert or deletion; repeated focused proof is deterministic without fixed scheduler sleeps.

## Inputs

Issue #816 and the two exact internal-review findings.

## Target Files / Surfaces

.csdlc/prepared/issues/512/validate-obs-b-redaction.sh; adl-runtime/tests/config_reload.rs; tightly coupled fixtures or testability seams only

## Validation Plan

Positive and negative OBS-B publication-shaped fixtures; repeated focused hot-reload cancellation tests; formatting and diff hygiene.

## Demo / Proof Requirements

Deterministic local validation proof only; no live cloud or UI demo.

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

No Observatory redesign and no general watcher refactor beyond the smallest deterministic testability seam.

## Notes / Risks

A test that only observes generation zero can pass without exercising cancellation; a redaction scan over planning prose can pass without scanning publication data.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
