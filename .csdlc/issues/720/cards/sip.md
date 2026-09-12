# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0720
Run ID: issue-0720
Version: v0.92.2
Title: [v0.92.2][Observatory] Remove retained-mode demo hazards
Branch: codex/720-observatory-live
Card Status: ready
Generated: 2026-09-12T00:09:24.617447+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/720
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/720
- Docs: docs/milestones/v0.92.2/SPRINT_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml
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
  output_card: .csdlc/issues/720/cards/sor.md
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
- Source issue-prompt slug: 720-observatory-live
- Required outcome type: code_and_regression_proof
- Demo required: false

## Goal

Make Live Observatory incapable of replacing current Runtime telemetry with historical retained snapshots.

## Required Outcome

Live-only mode, no retained polling or fallback telemetry, historical artifacts preserved.

## Acceptance Criteria

No Published/Retained controls; startup, navigation and failure never read retained API telemetry; no retained timer; orphan assignments removed; functional live regression proof; truthful documentation.

## Inputs

{
  "body": "## Goal\n\nMake the production/live Observatory fail-safe for demonstrations by removing one-click paths that silently replace live Runtime v3 telemetry with frozen retained-packet data.\n\n## Required outcome\n\n- Remove the Published and Retained topbar modes from the live Observatory and pin the product surface to Live.\n- Remove or isolate refreshRetained and its three-second static-file polling so the live build cannot continuously fetch v0.91.7 retained JSON.\n- Remove the orphaned setRuntimeTestStatus local assignments currently reported near app.js lines 3293, 4298, and 4371.\n- Preserve historical retained artifacts as repository evidence; do not delete evidence merely to hide it from the product UI.\n- Ensure the live UI cannot silently present historical/frozen data as current polis telemetry.\n\n## Acceptance criteria\n\n1. The live Observatory offers no Published or Retained mode controls.\n2. Normal startup and navigation cannot replace live telemetry with docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/api data.\n3. No obsolete three-second retained refresh timer runs in the live product path.\n4. The three unused status assignments are removed.\n5. Focused tests prove Live remains functional and no retained-mode route is reachable from the live UI.\n6. Documentation distinguishes retained historical evidence from the live Observatory product.\n\n## Non-goals\n\n- Do not delete immutable v0.91.7 evidence.\n- Do not redesign the Observatory.\n- Do not widen issue #512 or delay its publication.\n\n## Scheduling\n\nTarget v0.92.2, now open as milestone 2. This existing issue retains its independently admitted execution authority; WP-01/#864 and the later creation batches add no dependency edge or scope to this task.\n\n<!-- csdlc-github-operation:v0922-observatory-remove-retained-live-demo-hazards -->\n\n## Current milestone execution links\n\nMilestone association is scheduling metadata. This task retains its own owner and readiness authority and has no WP-01 dependency. The global launch requirement below governs new implementation admission.\n\nBefore any new implementation admission, all 69 core milestone tasks must have issue identities and all creation batches must receive independent review. This global launch requirement does not add a WP-01 execution dependency or interrupt already admitted owner work; preserve the existing owner\u2019s readiness and authority.\n\n\n<!-- csdlc-v3-operation:c3a981c0317c5555f7f7563cd33adfe570a6e7d5516ff57ee12012e27fc3203f -->\n\n## Execution sprint assignment\n\n**Sprint 8** in `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. This assignment includes the existing issue in the complete 69-task execution schedule. It creates no new issue or dependency edge; existing prerequisites, same-sprint dependency order, native readiness and the all-69 creation/review startup gate remain in force.\n\n\n<!-- csdlc-v3-operation:4ec2eb5f624c62ad25ac9b00d728bf646e5bc5c179f752ee3ed49ac6b4f7816b -->",
  "number": 720,
  "state": "OPEN",
  "title": "[v0.92.2][Observatory] Remove retained-mode demo hazards"
}


## Target Files / Surfaces

demos/html-observatory/{app.js,index.html,README.md,tests/}; focused Observatory validators and .csdlc/evidence/720.

## Validation Plan

node --test demos/html-observatory/tests/*.test.mjs; focused live-only browser or DOM behavioral proof; existing Observatory validator where applicable; exact-head independent review and hosted CI.

## Demo / Proof Requirements

Local UI regression plus browser Live proof; no cloud deployment

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

No cloud deployment, SDK refactor, UI redesign, historical evidence deletion, or merge.

## Notes / Risks

Disconnected live values must be explicitly stale; initialization must not seed telemetry from an old packet. Historical integration evidence may remain labelled separately.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
