# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1017
Run ID: issue-1017
Version: v0.92.2
Title: [v0.92.2][corporate][AWS] Migrate v-*.ai domains from personal AWS to the company account
Branch: codex/1017-v0922-company-domain-migration
Card Status: ready
Generated: 2026-09-16T23:15:26.654477+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1017
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/1017
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
- Source issue-prompt slug: v0922-company-domain-migration
- Required outcome type: document
- Demo required: false

## Goal

Migrate the owner-confirmed v-*.ai portfolio registration management, renewal responsibility and applicable DNS authority from the personal AWS account to the Agent Logic company account while preserving existing services.

## Required Outcome

Migrate the owner-confirmed v-*.ai portfolio registration management, renewal responsibility and applicable DNS authority from the personal AWS account to the Agent Logic company account while preserving existing services.

## Acceptance Criteria

Every approved domain has a completed or blocked disposition. Verify company registration control and renewals; company DNS authority or approved external exception; full record preservation; baseline and post-change DNS, TLS, redirects and mail checks or explicit not-applicable; TTL-based observation; recoverable backups and separate transfer/DNS recovery plans. Keep issue open for unresolved migrations. Retain sanitized per-domain evidence without private account/contact/billing/transfer data.

## Inputs

Live issue #1017; AGENTS.md; docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md; agent-logic/agent-logic.ai issues #31 and #32 as context only; owner-confirmed inventory and live registrar/account evidence collected during execution; current official AWS/registrar procedures verified before changes.

## Target Files / Surfaces

.csdlc/evidence/1017/ sanitized inventory, sequence, verification summary and migration disposition; .csdlc/issues/1017/cards and generated projections; private execution backup/account evidence outside Git with restricted access, referenced only by safe logical identifiers.

## Validation Plan

Preparation: native validate and doctor check all six active-template cards. The declared semantic_card_projections Cargo check proves tooling structure only, not domain migration. Execution: independently verify each registration and renewal owner, authoritative NS/DNSSEC, complete zone records, DNS/TLS/HTTP redirects/mail baseline and post-cutover behavior; observe for relevant TTLs, record N/A with reasons, rollback on defined failures. Required private external observations are nondeterministic and separate from docs_only local evidence hygiene; git diff --check and redaction review for sanitized tracked records. No green card test may substitute for cloud/service proof.

## Demo / Proof Requirements

<demo_proof_requirements>

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

No unrelated domains, website redesign, unrelated hosting/email changes, automatic transfer rollback, customer publication or early source deletion. No AWS calls or migration during preparation.

## Notes / Risks

Inventory, source profile, account identities, eligibility, DNSSEC, transfer reversibility and dependent services are not yet verified. Never assume registration transfer moves DNS or hosting. Keep source configuration until destination and observation criteria pass; stop on identity ambiguity, record loss or service regression. No credentials/account IDs/private contacts in tracked artifacts.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
