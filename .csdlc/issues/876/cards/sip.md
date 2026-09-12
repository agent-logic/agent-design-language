# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0876
Run ID: issue-0876
Version: v0.92.2
Title: [v0.92.2][PLAT-PROVIDER] Consume validated editable provider definitions
Branch: codex/876-provider-definitions
Card Status: ready
Generated: 2026-09-11T23:55:23.353658+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/876
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/876
- Docs: docs/milestones/v0.92.2/
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
  output_card: .csdlc/issues/876/cards/sor.md
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
- Source issue-prompt slug: 876-provider-definitions
- Required outcome type: production-behavior
- Demo required: yes

## Goal

The Runtime provider-definition loader consumes operator-editable endpoint/profile data through the existing adapter boundary, atomically retains the last-known-good snapshot on invalid replacement, and exposes the change to subsequent real production dispatch. Depends on WP-01/#864, RT-COST/#854 and the verified merged provider hot-loading predecessor #622. RT-PROVIDER/#855 separately owns dynamic agent lifecycle; this issue supplies configuration, not attach/detach implementation.

## Required Outcome

The Runtime provider-definition loader consumes operator-editable endpoint/profile data through the existing adapter boundary, atomically retains the last-known-good snapshot on invalid replacement, and exposes the change to subsequent real production dispatch. Depends on WP-01/#864, RT-COST/#854 and the verified merged provider hot-loading predecessor #622. RT-PROVIDER/#855 separately owns dynamic agent lifecycle; this issue supplies configuration, not attach/detach implementation.

## Acceptance Criteria

1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map. 2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption. 3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation. 4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim. 5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer.

## Inputs

Source issue, canonical v0.92.2 contracts and accepted dependency outputs.

## Target Files / Surfaces

Extend `adl/src/provider/reload.rs`, `adl/src/provider/profiles.rs`, and the minimum wiring in `adl/src/provider/mod.rs` and `adl/src/execute/runner.rs`. Read `docs/providers/provider-profile-hot-loading.md` and `docs/provider/inference-profiles.md`: existing `ProviderReloadOwner`/`ProviderReloadSnapshot` already validate a provider-only sidecar and use the kernel watcher. Reuse that production owner; do not add another registry/watcher. Own focused reload/profile tests and accompanying provider docs. Add a named data/schema/example file only after inventorying the current format and recording its exact path; do not expand into a provider rewrite.

## Validation Plan

1. Load valid editable definitions using the production entrypoint, execute a local controlled provider request and prove the observed endpoint/model/profile matches the selected validated snapshot. Edit definitions without rebuilding; a later dispatch uses the new complete snapshot. In-flight dispatch retains its original snapshot and concurrent readers never observe a mixed map. 2. Preserve existing endpoint/profile behavior, declared compatibility and provider generations. Separate instance data from adapter behavior; document supported fields and explicit unsupported cases. No hardcoded instance branch may stand in for consumption. 3. Through the production loader reject malformed, unsupported, incomplete and credential-shaped definitions, including secret values hidden under neutral nested keys. Keep only approved credential references. Invalid initial configuration fails closed; invalid replacement retains the prior whole snapshot and yields a bounded diagnostic. Execute last-known-good readback, not just schema validation. 4. Verify #622 hot-load authority and #854 cost-control behavior are consumed. Reload must not introduce recurring metered inference probes. Tests use a deterministic local provider endpoint and make no hosted-provider qualification claim. 5. Retain executed endpoint/profile parity, reload concurrency, schema negatives and secret-redaction proof. A configuration schema or unused parser is insufficient; real dispatch is the consumer. PVF lane: deterministic local provider integration/contract; role: production definition consumption, atomic reload and invalid-input rejection; resources: bounded local CPU/filesystem and controlled loopback endpoint, no paid inference; gate: required provider-platform completion and downstream RT-PROVIDER input. Live/provider-cost execution needs separate explicit authority. Stop on missing hot-load authority, unresolved owner collision, hardcoded instance data, credential capture, failed or missing proving scenario, or incompatible unreviewed format changes. Exclude provider behavior rewrite, agent lifecycle, MLX/PAIR implementation, benchmark marketing, broad Runtime changes and schema-only completion.

## Demo / Proof Requirements

Execute and retain the complete acceptance/proving cases in the source issue; no schema-only or fixture-only substitution.

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

PVF lane: deterministic local provider integration/contract; role: production definition consumption, atomic reload and invalid-input rejection; resources: bounded local CPU/filesystem and controlled loopback endpoint, no paid inference; gate: required provider-platform completion and downstream RT-PROVIDER input. Live/provider-cost execution needs separate explicit authority. Stop on missing hot-load authority, unresolved owner collision, hardcoded instance data, credential capture, failed or missing proving scenario, or incompatible unreviewed format changes. Exclude provider behavior rewrite, agent lifecycle, MLX/PAIR implementation, benchmark marketing, broad Runtime changes and schema-only completion.

## Notes / Risks

Bound implementation and focused local proof recorded in .csdlc/evidence/876/IMPLEMENTATION_PROOF.md. Independent exact-head review and PR CI remain required; no merge or closeout claim.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
