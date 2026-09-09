# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.4/sip.md`

Task ID: issue-0759
Run ID: issue-0759
Version: 1.0.4
Title: [v0.92.1][TAIL-06.03][runtime] Isolate dynamic-agent health task failures
Branch: codex/759-dynamic-agent-health-task-failures
Card Status: ready
Generated: <timestamp>

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/759
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/759
- Docs: Root AGENTS.md native C-SDLC v3 workflow and issue #759 live contract.
- Other: none

## Agent Execution Rules
- This issue is not started yet; do not assume a branch or worktree already exists.
- Do not use v1 wrappers; bind execution with `csdlc-bind` only if execution later becomes necessary.
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
  output_card: .csdlc/issues/759/cards/sor.md
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
- Source issue-prompt slug: dynamic-agent-health-task-failures
- Required outcome type: runtime defect remediation with deterministic regression proof
- Demo required: deterministic multi-agent health sweep regression

## Goal

Repair the dynamic-agent health sweep so a single asynchronous task panic or cancellation does not abort remaining readiness checks.

## Required Outcome

The sweep must explicitly handle successful task output, task failure, and end-of-set exhaustion while preserving a stable failed-agent identity and draining all remaining checks.

## Acceptance Criteria

- One failed or cancelled health task does not terminate the remainder of the sweep.
- Every remaining check is drained and every successful peer projection is retained.
- The failed check is associated with a stable agent identifier and surfaced as a failure.
- Task failures are not hidden, downgraded, or converted to successful health.

## Inputs

- GitHub issue #759 live prompt.
- Source finding C520-CODE-003 from internal review #520.
- Exact reviewed candidate c24f8fa65ce445b03ce6cd69007307291d78b60c.
- Evidence: adl-runtime-kernel/src/control.rs:3600-3659.

## Target Files / Surfaces

- adl-runtime-kernel/src/control.rs
- Narrowly coupled Runtime dynamic-agent health tests in the same crate.

## Validation Plan

- Run deterministic focused Runtime dynamic-agent health tests proving multi-agent drain behavior and stable failed identity.
- Run cargo fmt for the Runtime crate.
- Run strict clippy/check appropriate to the touched Runtime surface before review/publication.

## Demo / Proof Requirements

Focused regression must exercise multiple dynamic agents where one forced task failure does not prevent successful peers from updating.

## Constraints / Policies

- Follow `AGENTS.md`.
- Use the typed C-SDLC v2 operator skills and Rust binaries for lifecycle routing.
- Edit cards only with editor skills.
- Work only in the bound issue worktree after `csdlc-bind`.
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

- Do not change provider health semantics.
- Do not hide task panics or cancellations.
- Do not widen beyond dynamic-agent health sweep handling.

## Notes / Risks

JoinError does not carry application identity by itself; the implementation must bind task identity explicitly before spawning or through a stable join-id map.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run `csdlc-bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
