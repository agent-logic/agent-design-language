# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.4/sip.md`

Task ID: issue-0776
Run ID: issue-0776
Version: 1.0.4
Title: [v0.92.1][TAIL-06.17][csdlc] Resolve structure schemas beside versioned templates
Branch: codex/776-versioned-template-structure-schemas
Card Status: ready
Generated: <timestamp>

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/776
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/776
- Docs: docs/csdlc-v3/CURRENT_AUTHORITY.md and docs/templates/prompts/current.json
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
  output_card: .csdlc/issues/776/cards/sor.md
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
- Source issue-prompt slug: versioned-template-structure-schemas
- Required outcome type: defect_fix
- Demo required: true

## Goal

Repair native C-SDLC v3 card structure validation so each versioned template resolves its structure schema from the same versioned directory.

## Required Outcome

All six cards in the current 1.0.4 registry pass structure validation without weakening schema checks or changing template/schema content.

## Acceptance Criteria

AC-1: Resolve schemas beside the versioned template directory.
AC-2: Prove SIP, STP, SPP, VPP, SRP, and SOR against current.json.
AC-3: Preserve existing focused native local-command behavior.
AC-4: Keep the diff limited to production path resolution, its focused regression, and typed lifecycle records.

## Inputs

Issue #776, blocking issue #758, parent ledger #522, current prompt registry 1.0.4, native local validation implementation and tests.

## Target Files / Surfaces

csdlc-v3/src/commands/local/mod.rs; csdlc-v3/tests/local_commands.rs; issue-local typed lifecycle records.

## Validation Plan

Run the exact all-six-card regression, then the complete local_commands integration target and git diff --check.

## Demo / Proof Requirements

The regression must consume docs/templates/prompts/current.json and demonstrate successful structure validation for all six rendered cards.

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

Template/schema content changes; prompt registry redesign; Runtime changes owned by #758; unrelated cleanup.

## Notes / Risks

The primary risk is accidentally making fixture layout pass while the real versioned current registry remains untested.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run `csdlc-bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
