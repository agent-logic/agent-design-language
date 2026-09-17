# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0945
Run ID: issue-0945
Version: v0.92.2
Title: [v0.92.2][ARCH-ADR] Reconcile proposed ADRs with implementation and obtain decision approval
Branch: codex/945-adr-decision-reconciliation
Card Status: ready
Generated: 2026-09-17T01:12:51.241410+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/945
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/945
- Docs: Issue #945; docs/architecture/adr/issue-911/; docs/milestones/v0.92.2/adr/issue-911/; current CodeFriend, provider and C-SDLC implementation; #925 TAIL-10 acceptance contract.
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
  output_card: .csdlc/issues/945/cards/sor.md
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
- Source issue-prompt slug: adr-decision-reconciliation
- Required outcome type: document
- Demo required: false

## Goal

Reconcile all twelve issue-911 Proposed ADRs with current implementation and planning; correct stale claims and prepare an exact-text operator decision packet for #945. Preserve all69 task dispositions and separate #848/#910 obligations. Formal acceptance remains pending explicit per-candidate decisions.

## Required Outcome

Reconcile all twelve issue-911 Proposed ADRs with current implementation and planning; correct stale claims and prepare an exact-text operator decision packet for #945. Preserve all69 task dispositions and separate #848/#910 obligations. Formal acceptance remains pending explicit per-candidate decisions.

## Acceptance Criteria

All12 candidates have current source evidence and explicit recommended disposition, owner, rationale, consequences and reversibility; all69 mappings retained; hashes and focused validation pass; independent exact-head review complete. Actual accepted/revised/rejected/deferred decisions require explicit operator or designated decision-owner evidence. No acceptance or closure claimed while required decisions remain pending.

## Inputs

Issue #945; docs/architecture/adr/issue-911/; docs/milestones/v0.92.2/adr/issue-911/; current CodeFriend, provider and C-SDLC implementation; #925 TAIL-10 acceptance contract.

## Target Files / Surfaces

docs/architecture/adr/issue-911/; docs/milestones/v0.92.2/adr/issue-945/; source-linked ADR planning references; .csdlc/evidence/945/; native issue cards only

## Validation Plan

PVF docs_only, deterministic local CPU/file checks. Run historical issue-911 validate_packet.py --self-test to preserve source snapshot; run new issue-945 source/hash/link/decision-coverage validator and negative cases. Native semantic_card_projections tests are tooling-only, not architectural acceptance. Independent substantive exact-head review is required. No runtime/provider/cloud execution.

## Demo / Proof Requirements

Documentation-only reconciliation; no runtime demo.

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

No implementation changes, live writer activation, paid effects, repository split, deployment, automatic ADR acceptance/numeric promotion, merge, release, or historical evidence rewriting.

## Notes / Risks

Source review does not prove runtime behavior. Proposed ADRs remain pending operator decision; preserve historical records and separate #848/#910 obligations.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
