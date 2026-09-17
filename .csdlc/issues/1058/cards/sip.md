# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1058
Run ID: issue-1058
Version: v0.92.2
Title: [v0.92.2][CF-AGENT] Run website-controlled reviews through an installed local agent
Branch: codex/1058-v0922-installed-local-review-agent
Card Status: ready
Generated: 2026-09-16T20:16:52.878587+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1058
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/1058
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
  output_card: .csdlc/issues/1058/cards/sor.md
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
- Source issue-prompt slug: v0922-installed-local-review-agent
- Required outcome type: document
- Demo required: false

## Goal

a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.

## Required Outcome

a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.

## Acceptance Criteria

authenticated scoped pairing/revocation, correct execution location, bounded source consent and privacy filtering, non-secret user/agent credentials, no embedded provider keys, reconnect and disconnect/cancellation/retry outcomes without duplicate successful dispatch, truthful status/artifact forwarding, and cross-user/expired-agent denial. Preserve evidence identity, retention and exact-artifact approval. BYOK and Google sign-in are excluded from Beta1. Do not claim local execution means no selected evidence is sent to the model service; disclose the actual scoped model input.

## Inputs

## Complete implementation outcome

Result: a user installs CodeFriend on the selected macOS/Linux environment, pairs it with their invited website session, and starts/observes/cancels local reviews from the website. Review execution occurs on the user's computer; model requests use Agent Logic's authorized model-access service. Installation and pairing must be reproducible from a clean environment.

Acceptance: authenticated scoped pairing/revocation, correct execution location, bounded source consent and privacy filtering, non-secret user/agent credentials, no embedded provider keys, reconnect and disconnect/cancellation/retry outcomes without duplicate successful dispatch, truthful status/artifact forwarding, and cross-user/expired-agent denial. Preserve evidence identity, retention and exact-artifact approval. BYOK and Google sign-in are excluded from Beta1. Do not claim local execution means no selected evidence is sent to the model service; disclose the actual scoped model input.

Dependencies and interfaces: accepted shared model-access and authenticated run-control contracts, existing local operator/review consumers, website pairing controls. Test deterministic failure cases and separately prove a real model-backed local review. #914 connects accepted components; this issue owns agent functionality, not whole-product integration or independent qualification.


## Scope and execution authority

Operator-authorized prerequisite under Sprint #936 for integration #914, created after accepted ADR0084 / PR1054. Separate from the historical69-task denominator. #914 consumes accepted implemented output; #915 independently qualifies the integrated candidate. No component-only proof can close either integration or qualification.

All three prerequisites retain both Beta1 website modes, invitation-only GitHub sign-in, Agent Logic model access for both modes, CLI support and BYOK deferral. Google sign-in is not approved. Preserve evidence/privacy/retention and exact-artifact approval; no automatic external publication or source mutation. Use native cards, an issue-bound worktree/goal, focused PVF classification, meaningful positive/negative tests, independent exact-head review and required checks. Deployment/provider execution needs explicit bounded operational authority.

## Validation contract

Record exact candidate, source, environment and actual artifact identities. Declare new tests lane, proof role, determinism, resource bounds and release-gate status. Local deterministic fixtures cover failure/authorization invariants; separately record real installed/provider/browser observations. No zero-test or synthetic-only product acceptance. Fix actionable findings before publishing implementation PR.

Execution prerequisite: #1056 accepted shared server/model-access and run-control contracts. Coordinate disjoint work after contract acceptance; #914 consumes the completed component.


## Target Files / Surfaces

adl/src/codefriend/agent.rs; adl/src/bin/codefriend_agent.rs; adl/src/codefriend/review/runner.rs; adl/src/codefriend/mod.rs; adl/Cargo.toml; adl/tests/codefriend_agent.rs; adl/tests/fixtures/codefriend/agent/PVF.json; docs/codefriend/LOCAL_AGENT.md

## Validation Plan

cargo test --manifest-path adl/Cargo.toml --test codefriend_agent; existing codefriend_review_runner regression tests; focused rustfmt and clippy. Deterministic pairing, expiry/revocation, cross-user, consent, path restriction, durable duplicate, crash/reconnect, cancellation, result forwarding and no-secret-output cases. Separate installed macOS/Linux and real-provider/browser journeys are required for acceptance and await bounded operational authorization.

## Demo / Proof Requirements

cargo test --manifest-path adl/Cargo.toml --test codefriend_agent; existing codefriend_review_runner regression tests; focused rustfmt and clippy. Deterministic pairing, expiry/revocation, cross-user, consent, path restriction, durable duplicate, crash/reconnect, cancellation, result forwarding and no-secret-output cases. Separate installed macOS/Linux and real-provider/browser journeys are required for acceptance and await bounded operational authorization.

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

Sprint #936 goal remains active by explicit operator instruction; no replacement issue goal. Implement locally now. No paid calls, deployment or merge authority inferred. Website/agent credentials are private secrets even though they are not provider keys; never print them.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
