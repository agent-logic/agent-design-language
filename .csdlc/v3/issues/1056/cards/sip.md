# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1056
Run ID: issue-1056
Version: v0.92.2
Title: [v0.92.2][CF-SERVER] Execute hosted reviews and provide governed model access
Branch: codex/1056-v0922-server-review-model-access
Card Status: ready
Generated: 2026-09-16T20:16:52.878587+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1056
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/1056
- Docs: AGENTS.md; docs/adr/0084-cf-09.md; docs/milestones/v0.92.2/adr/issue-945/BETA1_DELIVERY.md
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
  output_card: .csdlc/issues/1056/cards/sor.md
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
- Source issue-prompt slug: v0922-server-review-model-access
- Required outcome type: document
- Demo required: false

## Goal

invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

## Required Outcome

invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

## Acceptance Criteria

real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.

## Inputs

## Complete implementation outcome

Result: invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

Acceptance: real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.

Dependencies and interfaces: existing review pipeline consumers; shared provider definitions and privacy/retention contracts; website identity contract. Agree the model-access and run-control contracts before the local agent or website depend on them. Contract fixtures are not real-provider acceptance. Current pre-extraction code remains in ADL until separately authorized repository migration.


## Scope and execution authority

Operator-authorized prerequisite under Sprint #936 for integration #914, created after accepted ADR0084 / PR1054. Separate from the historical69-task denominator. #914 consumes accepted implemented output; #915 independently qualifies the integrated candidate. No component-only proof can close either integration or qualification.

All three prerequisites retain both Beta1 website modes, invitation-only GitHub sign-in, Agent Logic model access for both modes, CLI support and BYOK deferral. Google sign-in is not approved. Preserve evidence/privacy/retention and exact-artifact approval; no automatic external publication or source mutation. Use native cards, an issue-bound worktree/goal, focused PVF classification, meaningful positive/negative tests, independent exact-head review and required checks. Deployment/provider execution needs explicit bounded operational authority.

## Validation contract

Record exact candidate, source, environment and actual artifact identities. Declare new tests lane, proof role, determinism, resource bounds and release-gate status. Local deterministic fixtures cover failure/authorization invariants; separately record real installed/provider/browser observations. No zero-test or synthetic-only product acceptance. Fix actionable findings before publishing implementation PR.


## Target Files / Surfaces

invited users' authorized website requests execute the existing CodeFriend review pipeline on Agent Logic servers, with isolated run/evidence stores and observable success/failure/cancel states. A credential-protecting model-access service also serves authorized installed local agents using Agent Logic-provided model access. Raw provider credentials never reach the website or local agent.

## Validation Plan

PVF: deterministic component authorization/lifecycle negatives plus separately authorized real installed/provider/browser proof. Pin exact source, binary, environment, input/output and nonzero scenario counts. Native semantic_card_projections validator proves preparation tooling only and cannot satisfy implementation acceptance. Replace or supplement it with the authored component validators before proof/publication. real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.

## Demo / Proof Requirements

PVF: deterministic component authorization/lifecycle negatives plus separately authorized real installed/provider/browser proof. Pin exact source, binary, environment, input/output and nonzero scenario counts. Native semantic_card_projections validator proves preparation tooling only and cannot satisfy implementation acceptance. Replace or supplement it with the authored component validators before proof/publication. real bounded server review with source/output/candidate identity, per-user run/artifact authorization, restricted model requests, explicit resource limits, provider failure/cancel/retry behavior and no successful manifest for failed stages. Distinguish server review execution from local review execution using the shared model-access service. Use existing shared Runtime/provider/evidence owners. No BYOK, customer billing or autonomous source changes. Hosting topology and exact operating limits are implementation decisions that must be recorded before deployment or paid effects; preparation grants no such effects.

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

No Google sign-in, BYOK, open signup, unrelated repository migration, automatic merge/release, or claim of whole-product qualification.

## Notes / Risks

Local implementation explicitly authorized in ADL. Execution design and dependencies are recorded in SPP. Deployment and paid provider proof remain separate bounded approvals; no product acceptance claim until they pass.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
