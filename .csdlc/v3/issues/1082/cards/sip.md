# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1082
Run ID: issue-1082
Version: 1.0.5
Title: [v0.92.2][Runtime][Bedrock] Adopt Converse and migrate unreliable OpenRouter residents
Branch: codex/1082-bedrock-converse-resident-bindings
Card Status: ready
Generated: 2026-09-19

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1082
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/1082
- Docs: docs/provider/inference-profiles.md; docs/runtime/RESIDENT_PROVIDER_MONITORING.md; docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md
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
  output_card: .csdlc/issues/1082/cards/sor.md
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
- Source issue-prompt slug: bedrock-converse-resident-bindings
- Required outcome type: runtime_provider_and_identity_contract
- Demo required: true

## Goal

Use the provider-neutral Bedrock Converse API for current authorized text models and migrate two unreliable residents to durable identities that remain independent from their provider and model bindings.

## Required Outcome

The Runtime admits current provider-spec definitions for Kimi K2.5 and Nemotron Super 3 120B, calls both through Bedrock Converse, preserves model-independent resident continuity through an explicit identity migration, and proves bounded live inference without recurring paid probes.

## Acceptance Criteria

Bedrock uses typed Converse for Nova and non-Nova text models; stable model references remain distinct from native provider IDs; current provider-spec limits, capabilities, codec and failure categories materialize before dispatch; harbor.axioma and quill.axioma preserve legacy continuity while model/provider swaps remain ordinary binding changes; welcome, operator conversation, governed A2A, health and Observatory proof pass; live calls are bounded and non-recurring.

## Inputs

Issue #1082; provider-neutral lifecycle #855; provider definitions #876; resident monitoring #854; typed failure projection #1079; current Bedrock authorization; existing nexus.axioma and nemotron.axioma continuity; Axioma Polis welcome package.

## Target Files / Surfaces

adl-provider-core Bedrock, profile, provider-substrate and registry surfaces; adl provider adapter compatibility path; adl-runtime-kernel resident admission, identity migration, continuity, health and Observatory projection; provider and resident monitoring documentation; issue-local proof records.

## Validation Plan

Run provider-core component tests and strict Clippy, focused ADL Converse tests, Runtime identity and provider tests, formatting and diff hygiene, two bounded direct live calls, isolated or live Runtime conversation/A2A proof, native proof, independent exact-head review and required CI.

## Demo / Proof Requirements

One bounded generated response for each exact Bedrock model, one operator conversation for each durable resident, one governed A2A exchange involving each, welcome digest verification, and provider-health plus Observatory readiness readback.

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

No migration of the healthy DeepSeek resident, model-derived agent names, blanket Bedrock catalog enablement, recurring generated-content probes, credential or IAM changes, silent history loss, or claim that catalog metadata proves inference.

## Notes / Risks

Keep provider/model identity separate from agent identity; fail closed on AWS account mismatch, ambiguous dispatch, unsupported controls, stale legacy-name migration, or missing current review truth.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
