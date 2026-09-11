# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0820
Run ID: issue-0820
Version: v0.92.1
Title: [v0.92.1][TAIL-06.08c][quality] Close distributed Runtime retained proof gaps
Branch: codex/820-distributed-runtime-retained-proof
Card Status: ready
Generated: 2026-09-10T00:15:00Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/820
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/820
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
  output_card: .csdlc/issues/820/cards/sor.md
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
- Source issue-prompt slug: distributed-runtime-retained-proof
- Required outcome type: quality
- Demo required: false

## Goal

Close exactly the 25 distributed-Runtime retained-proof gaps with candidate-bound proof or exact governed disposition.

## Required Outcome

Exactly 25 distributed-Runtime rows receive candidate-bound execution proof or an explicit criterion-specific governed disposition, with release blocked until required operator approval.

## Acceptance Criteria

Exactly 25 DRT rows are consumed once; each is candidate-proven or has an exact operator-reviewed amendment/removal; hardware/provider proof stays explicitly classified; zero rows are missing, duplicated, or silently promoted; exact-head review binds the result.

## Inputs

Issue #820; parent #522; source review #520 at fb6cbc7f619daa54f901fd2d12f480add682ace3; D520-RET-001; #764 denominator.

## Target Files / Surfaces

Only the 25 DRT-prefixed non-proving rows and narrow issue-820 plan, validation, evidence, and lifecycle artifacts.

## Validation Plan

Freeze the exact candidate denominator/source bytes, validate exactly 25 DRT non-proving rows, run the smallest causal local Runtime proof, generate criterion-specific dispositions for non-provable live requirements, execute adversarial mutation tests, and review the exact head.

## Demo / Proof Requirements

Replay local candidate proof only where it causally covers a criterion; retain live-provider requirements as explicit governed dispositions.

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

No corporate Runtime, C-SDLC v3, or TAIL-01 rows; no live cloud spend; no promotion of deterministic contract fixtures into production behavior proof.

## Notes / Risks

Hardware/provider-dependent claims remain non-proving without authentic live receipts; release must fail closed while governed dispositions await operator approval.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
