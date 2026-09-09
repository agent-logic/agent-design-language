# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0815
Run ID: issue-0815
Version: 1.0.5
Title: [v0.92.1][TAIL-06.10][security] Authenticate AWS and GCP mutation authorization
Branch: codex/815-cloud-authorization-authenticity
Card Status: ready
Generated: 2026-09-09T20:50:00Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/815
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/815
- Docs: Issue #520 second-review remediation under parent #522
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
  output_card: .csdlc/issues/815/cards/sor.md
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
- Source issue-prompt slug: cloud-authorization-authenticity
- Required outcome type: security_control_and_local_negative_proof
- Demo required: false

## Goal

Prevent a repo-writing process with provider credentials from manufacturing cloud-mutation approval and bind AWS approval to the exact saved plan bytes.

## Required Outcome

GCP mutation entrypoints require a detached operator signature anchored outside the repository; AWS authorization hashes the actual tfplan, proves its inspected JSON derives from those bytes, and binds authenticated account identity.

## Acceptance Criteria

- Forged or unsigned GCP approval is rejected before provider mutation.
- GCP signatures bind repository, issue, project, resource bounds, expiry, and exact plan digest.
- AWS hashes the actual saved tfplan and compares its Terraform JSON projection with the reviewed projection.
- AWS authorization binds the observed business-account identity rather than a self-asserted boolean.
- Negative fixtures reject forged approval, altered plan, altered digest sidecar, expiry, and cross-account/project replay.
- Read-only proof remains available without mutation authority; no paid cloud mutation runs.

## Inputs

- GitHub issue #815 and parent #522.
- Internal-review findings D520-SEC-001 and D520-SEC-002.
- Existing #727 AWS and #731 GCP authorization validators and guarded mutation scripts.

## Target Files / Surfaces

- .csdlc/prepared/issues/727 authorization validator
- .csdlc/prepared/issues/731 authorization validator and mutation entrypoints
- .csdlc/prepared/issues/815 shared verifier and negative fixtures
- Coupled AWS/GCP authorization templates and runbooks

## Validation Plan

Run deterministic local signature, plan-byte, projection, expiry, account/project replay, and read-only-without-authority fixtures; shell/Python syntax, diff hygiene, and independent exact-head review.

## Demo / Proof Requirements

No live demo or cloud mutation; deterministic local cryptographic and Terraform-mock fixtures are the proving surface.

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

- No paid AWS or GCP mutation.
- No broad credential-management redesign.
- No change to historical live evidence.
- No provider-resource expansion.

## Notes / Risks

Do not treat self-asserted JSON, a mutable digest sidecar, or a test signer as production authority. Keep the trusted signer anchor outside the repository and fail before any provider mutation.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
