# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0967
Run ID: issue-0967
Version: v0.92.2
Title: [v0.92.2][RT-PROVIDER][corrective] Make hosted A2A initiation deterministic across provider output formats
Branch: codex/967-deterministic-hosted-a2a
Card Status: ready
Generated: 2026-09-12

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/967
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/967
- Docs: docs/api/runtime-v3/v1/observatory.openapi.json; merged #855 and PR #964; corrective issue #967.
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
  output_card: .csdlc/issues/967/cards/sor.md
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
- Source issue-prompt slug: issue-967-deterministic-hosted-a2a
- Required outcome type: implementation
- Demo required: true

## Goal

Make authenticated operator-requested A2A initiation independent of provider reply formatting while retaining one initiating provider call and existing signed peer delivery.

## Required Outcome

Make authenticated operator-requested A2A initiation independent of provider reply formatting while retaining one initiating provider call and existing signed peer delivery.

## Acceptance Criteria

Refuse invalid, unknown, self-targeted, empty and over-limit actions before provider calls; dispatch one signed peer exchange after an ordinary reply; coalesce identical model/request actions; reject conflicts before peer dispatch; preserve absent-field model actions and replay fingerprints; align OpenAPI and Runtime boundaries; pass zero-paid 5/5 and 3/3 matrices, separately authorized bounded hosted acceptance, exact-head independent review and required CI.

## Inputs

Live issue #967 acceptance contract and merged #855/PR964 baseline; retained corrective working-tree proof.

## Target Files / Surfaces

adl-runtime-kernel/src/control.rs; adl-runtime-kernel/tests/openapi_contract.rs; adl/tools/issue855_provider_lifecycle.py; docs/api/runtime-v3/v1/observatory.openapi.json; issue-local cards and proof metadata.

## Validation Plan

CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --lib sixth_registered_provider_uses_real_canonical_a2a_dispatch; CARGO_TARGET_DIR=adl/target cargo test --locked --offline --manifest-path adl-runtime-kernel/Cargo.toml --test openapi_contract; bounded zero-paid lifecycle harness using typed requested_agent_action. Exact-head installed fixture command and paid command must be recorded with their source/binary identities before execution. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed.

## Demo / Proof Requirements

Zero-paid typed-wire matrices for five provider families and three hosted topologies; a separately authorized hosted OpenAI/Anthropic/Vertex run within existing cost, call, token and time bounds.

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

No arbitrary JSON extraction from prose; no bypass of signatures, canonical addressing, capability checks or replay protection; no credential, billing, model or cloud configuration changes; no rewriting merged #855/PR964 history.

## Notes / Risks

Preserve failed hosted-live-03 report SHA256 651f9a00fb5b96e2a6b0539d1f9321c4546631a16b78e9c723fe823c0d95d439: OpenAI five successful calls, Anthropic two successful calls, Vertex zero. Raw provider output was not retained; exact response shape is unknown. Commit corrective source under #967, rebuild exact-head binaries, retain and review exact proof, pass current CI, and obtain fresh authorization before one bounded hosted acceptance run. No hosted retry or merge is claimed.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
