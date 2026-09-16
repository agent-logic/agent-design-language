# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0970
Run ID: issue-0970
Version: 1.0.5
Title: [provider architecture] Make declared AProvider inference configuration effective and observable
Branch: codex/970-provider-architecture-effective-configuration
Card Status: ready
Generated: 2026-09-15

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/970
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/970
- Docs: docs/provider/inference-profiles.md and docs/providers/provider-profile-hot-loading.md
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
  output_card: .csdlc/issues/970/cards/sor.md
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
- Source issue-prompt slug: aprovider-effective-inference-configuration
- Required outcome type: provider_runtime_contract
- Demo required: false

## Goal

Make every declared bounded AProvider inference control either reach the selected built-in codec request through one canonical effective configuration or fail before dispatch.

## Required Outcome

Provider profiles and explicit definitions normalize through one typed effective configuration; GeneralProvider binds and fingerprints the consumed values; unsupported or executable configuration fails closed; Ollama HTTP wire capture proves all supported controls.

## Acceptance Criteria

Canonical typed effective controls cover context window, output limit, temperature, top-p, seed, timeout, reasoning/think, and keep-alive; profiles and explicit definitions share normalization; codecs declare consumed controls and reject supplied unsupported controls before execution; executable authority fields are rejected; Ollama HTTP forwards every supported field; a redacted projection and stable fingerprint bind effective values; focused positive and negative local tests pass.

## Inputs

Issue #970; completed dependencies #514, #876, and #855; provider recovery boundary #901; the #900 Ollama context-window gap; current adl-provider-core profile, candidate, substrate, and codec implementation.

## Target Files / Surfaces

adl-provider-core/src/provider_substrate.rs; adl-provider-core/src/candidate.rs; adl-provider-core/src/http_family.rs; adl-provider-core/src/http_family/tests.rs; adl-provider-core/src/profiles.rs; docs/provider/inference-profiles.md; issue-local lifecycle and proof records.

## Validation Plan

Run focused provider-substrate normalization tests, candidate rejection tests, Ollama HTTP wire-capture tests, profile parity tests, component formatting and diff hygiene, native proof, and one independent exact-head pre-PR review. No live provider or paid call is required.

## Demo / Proof Requirements

Deterministic local wire capture is the execution proof; no external provider demo is required.

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

No executable provider definitions, provider-specific binaries or plugins, lifecycle redesign, failure/recovery qualification, model downloads, paid provider calls, benchmark claims, hardware provisioning, or changes to #900 evidence.

## Notes / Risks

Existing codecs consume different subsets of loose config keys. Normalization must preserve existing supported defaults while rejecting only explicitly supplied controls a selected codec cannot honor.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
