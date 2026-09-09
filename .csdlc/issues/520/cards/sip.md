# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0520
Run ID: issue-0520
Version: 1.0.5
Title: [v0.92.1][TAIL-04] Internal review
Branch: codex/520-internal-review
Card Status: ready_waiting_on_758
Generated: 2026-09-09T19:19:55Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/520
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/520
- Docs: v0.92.1 TAIL-04 internal review rerun after #718 and #758 merge
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
  output_card: .csdlc/issues/520/cards/sor.md
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
- Source issue-prompt slug: tail-04-internal-review
- Required outcome type: complete_findings_first_internal_review_packet
- Demo required: false

## Goal

Rerun the complete findings-first internal review against the exact v0.92.1 candidate after #718 and #758 merge.

## Required Outcome

A complete review denominator and canonical finding register expose every gap, partial implementation, inert path, unsupported claim, and release blocker at one exact fetched origin/main revision containing both gate merges.

## Acceptance Criteria

- Inventory and disposition every changed production file, proof file, canonical document, milestone issue/PR, and acceptance surface.
- Bind every finding to exact evidence, severity, candidate revision, impact, source lane, and owner.
- Identify partial, inert, unreachable, unproven, and documentation-only outcomes.
- Credit no sampled, empty, zero-test, or CI-only lane as passing proof.
- Validate packet manifests, counts, redaction, portability, and exact-head identity.

## Inputs

- GitHub issue #520 and milestone v0.92.1 planning surfaces.
- Merged #718 via PR #809.
- Merged #758 via PR #805 once terminal.
- Existing TAIL-04 review runbook, validator, and prior packet as historical input only.

## Target Files / Surfaces

- docs/milestones/v0.92.1/evidence/release/tail-04/**
- .csdlc/issues/520/**
- .csdlc/prepared/issues/520/**

## Validation Plan

Freeze exact fetched origin/main after both gates, rebuild complete denominators, rerun all mandatory specialist lanes, synthesize without dropping findings, run the production validator and negative fixtures, then obtain independent exact-head packet review.

## Demo / Proof Requirements

No live provider or cloud demo. Review evidence must be deterministic, candidate-bound, and complete.

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

- Do not fix product findings inside #520; route them through #522.
- Do not perform the external review owned by #521.
- Do not approve the release, merge product work, deploy, restart Runtime, or spend cloud/provider funds.

## Notes / Risks

Execution is blocked until #758/PR #805 is merged. The candidate must be fetched origin/main, equal the reviewed revision, and contain merge commits for both #718 and #758.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
