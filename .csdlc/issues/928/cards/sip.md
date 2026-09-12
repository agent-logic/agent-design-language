# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0928
Run ID: issue-0928
Version: v0.92.2
Title: [v0.92.2][Sprint 2] Runtime/provider foundations and ingestion
Branch: codex/928-combined-sprint-review
Card Status: ready
Generated: 2026-09-12T06:54:28.990815+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/928
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/928
- Docs: unknown
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
  output_card: .csdlc/issues/928/cards/sor.md
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
- Source issue-prompt slug: combined-sprint-review
- Required outcome type: review_packet
- Demo required: false

## Goal

Complete the nine-child Sprint 2 combined integration review and reconcile corrective #967 separately.

## Required Outcome

A findings-first combined Sprint 2 review ready for publication with exact residuals and no unsupported terminal or release claim.

## Acceptance Criteria

Preserve all nine original children exactly once; account for #967 separately; bind accepted heads, CI, merges and ancestry to one revision; record seven lane results, limitations and closeout truth; leave merge operator-controlled.

## Inputs

#848, #854, #855, #876, #877, #878, #879, #880, #881

## Target Files / Surfaces

.csdlc/evidence/928 review packet and docs/milestones/v0.92.2/SPRINT_v0.92.2.md

## Validation Plan

Native six-card validation; JSON and Markdown hygiene; exact child and corrective accounting; live issue/PR/check observation; Git ancestry; independent exact-head review; CI after publication.

## Demo / Proof Requirements

Consume the separately approved #967 three-provider hosted acceptance packet; do not rerun paid providers or overstate measured billing.

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

No child implementation, paid execution, historical evidence rewrite, automatic merge, release, public publication, or claim of child terminal closeout.

## Notes / Risks

P3 CLI help discoverability defect; no retained acquisition-to-store combined test; child native finish/cleanup remains asynchronous; #848 is planning evidence; hosted counts are not invoice proof.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
