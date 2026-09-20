# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1083
Run ID: issue-1083
Version: 1.0.5
Title: [v0.92.2][C-SDLC v3][defect] Reconcile preserved historical terminal identities
Branch: codex/1083-historical-terminal-identity-reconciliation
Card Status: ready
Generated: <timestamp>

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1083
- PR:
- Source Issue Prompt: <source_issue_prompt>
- Docs: <docs_context>
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
  output_card: <output_card>
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
- Source issue-prompt slug: <slug>
- Required outcome type: csdlc_defect_repair
- Demo required: false

## Goal

Reconcile the four preserved historical terminal identities blocking complete v0.92.2 native closeout without rewriting publication history or weakening authenticated guards.

## Required Outcome

Native finish and cleanup can truthfully reconcile #866, #873, #1028, and #1069 from their retained historical evidence, while ambiguous or mismatched identities remain fail-closed.

## Acceptance Criteria

All four named historical shapes reconcile through bounded native paths; ordinary finish remains unchanged; stale, wrong, ambiguous, unmerged, non-closing, cross-repository, and replay-mismatch cases fail without effects; exact evidence is preserved before cleanup.

## Inputs

Issue #1083; retained native state and GitHub readback for #866, #873, #1028, and #1069; terminal target selection, publication recovery, coordination compatibility, cleanup admission, and current operator manuals.

## Target Files / Surfaces

csdlc-v3 terminal and remote intent application/owner paths, historical recovery and cleanup admission, installed tests, operator documentation, and issue-local lifecycle evidence.

## Validation Plan

Run copied-state positive and adversarial regressions for all four shapes, focused terminal/remote suites, full C-SDLC v3 tests, formatting, strict Clippy, diff hygiene, installed-candidate verification, exact-head independent review, and real native finish/clean proof.

## Demo / Proof Requirements

Deterministic copied retained-state fixtures plus authenticated real terminal readback; no product demo is required.

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

No raw GitHub lifecycle writes, receipt synthesis, remote branch rewind, history deletion, generic publication override, release approval, AWS/provider usage, or weakening of exact-head/authenticated-readback rules.

## Notes / Risks

Historical identities overlap but are not interchangeable. The repair must preserve immutable evidence, reject uncertain effects, and distinguish closing delivery from checkpoints or later branch movement.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
