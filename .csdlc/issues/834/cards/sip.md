# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0834
Run ID: issue-0834
Version: v0.92.1
Title: [v0.92.1][TAIL-06.19][review] Reconcile finalized internal-review predecessor
Branch: codex/834-internal-review-reconciliation
Card Status: ready
Generated: 2026-09-11T02:59:50.407672+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/834
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/834
- Docs: docs/milestones/v0.92.1/evidence/release/tail-06/issue-834
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
  output_card: .csdlc/issues/834/cards/sor.md
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
- Source issue-prompt slug: issue-834-internal-review-reconciliation
- Required outcome type: release-evidence
- Demo required: false

## Goal

Reconcile finalized internal-review predecessor #520 into current external-review remediation evidence.

## Required Outcome

Live closed #520 / merged closing PR831 with ancestral merge; exactly14 internal findings mapped to remediation owners and merged semantic evidence; historical report unchanged.

## Acceptance Criteria

Observe closed520 and merged closing831 ancestral to candidate; all14 IDs exactly once with owners/evidence; preserve historical text; reject stale state/wrong closingPR/nonancestor/dropped or duplicatedIDs; independent exact-head review.

## Inputs

Issue834, tail-04 findings and summary, live520/831 closure, owners814-821 and correcting833/835.

## Target Files / Surfaces

docs/milestones/v0.92.1/evidence/release/tail-06/issue-834 and narrowly scoped current external-review/remediation links.

## Validation Plan

Focused deterministic local Python/Git contract validation with captured live GitHub readbacks. Explicit negatives for every required rejection. Required local evidence proof plus independent review; hosted CI integration.

## Demo / Proof Requirements

not_applicable; focused evidence validator is the proving surface

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

No historical report rewrite, runtime changes, closure-only semantic proof, merge or release authorization.

## Notes / Risks

#833 owns external report retention; #835 owns final gate correction. No release approval.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
