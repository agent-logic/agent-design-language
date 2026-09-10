# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0818
Run ID: issue-0818
Version: v0.92.1
Title: [v0.92.1][TAIL-06.08a][quality] Close corporate Runtime retained proof gaps
Branch: codex/818-corporate-runtime-retained-proof
Card Status: ready
Generated: 2026-09-10T22:36:29Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/818
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/818
- Docs: docs/milestones/v0.92.1/evidence/release/tail-06
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
  output_card: .csdlc/issues/818/cards/sor.md
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
- Source issue-prompt slug: corporate-runtime-retained-proof
- Required outcome type: quality
- Demo required: false

## Goal

Close exactly the 17 corporate/Runtime retained-proof gaps through exact governed dispositions without fabricating execution proof.

## Required Outcome

A deterministic, candidate-bound reconciliation packet accounts for all 17 rows exactly once and keeps release admission blocked until operator review approves the exact proposals.

## Acceptance Criteria

Exactly 17 corporate/Runtime rows are consumed once; every row has candidate-bound proof or an operator-reviewed governed amendment/removal; source support, issue closure, and ownership never count as execution proof; the reconciled bucket has zero unclassified rows and exact-head review identity.

## Inputs

Issue #818; #522; #520 finding D520-RET-001; #764 retained-proof denominator and retained-corporate-runtime mapping.

## Target Files / Surfaces

Corporate/Runtime retained-proof sources named by the 17-row denominator and narrow issue-818 reconciliation artifacts.

## Validation Plan

Validate the exact 17-row partition and uniqueness, candidate-bind every repository evidence path, reject forged approvals and semantic drift with a negative matrix, then obtain independent exact-head review.

## Demo / Proof Requirements

Replay the plan builder, receipt generator, validator, and negative matrix; no provider mutation or presentation-only demo.

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

No C-SDLC v3, distributed Runtime, TAIL-01, #520, #820, or #821 work; no private instruments; no relabeling documentation or closure as execution proof.

## Notes / Risks

Fail closed on missing, duplicate, stale-candidate, documentation-only, issue-closure-only, private-data, or synthetic approval evidence.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
