# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0932
Run ID: issue-0932
Version: v0.92.2
Title: [v0.92.2][Sprint 6] Hardware/provider qualification
Branch: codex/932-sprint6-coordination
Card Status: ready
Generated: 2026-09-15

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/932
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/932
- Docs: docs/milestones/v0.92.2/SPRINT_v0.92.2.md
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
  output_card: .csdlc/issues/932/cards/sor.md
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
- Source issue-prompt slug: 932-sprint6-coordination
- Required outcome type: sprint_review_closeout
- Demo required: false

## Goal

Close Sprint 6 only after #903, #904 and #905 are each accounted for by source acceptance, exact reviewed head, green CI, merged ancestry, terminal receipt and cleanup.

## Required Outcome

One independently reviewed Sprint 6 result with 12 of 12 child acceptance criteria reconciled and every retained qualification limit explicit.

## Acceptance Criteria

1. Account for #903, #904 and #905 exactly once. 2. Verify all 12 source acceptance criteria. 3. Verify exact reviewed heads, green CI, merge ancestry, native terminal receipts and cleanup. 4. Independently review the combined result and preserve residual limits. 5. Publish, merge, finish and clean the umbrella.

## Inputs

{
  "body": "## Outcome\n\nCoordinate Sprint 6 \u2014 Hardware/provider qualification through one complete reviewed execution result.\n\n## Existing child roster\n\n- #903 \u2014 PLAT-MLX\n- #904 \u2014 PLAT-PAIR\n- #905 \u2014 SPEC-RETEST\n\n## Management authority\n\nPart of #926. Created by explicit operator instruction that every execution sprint needs an umbrella. This management issue is outside the unchanged 69-core-task denominator. Source: execution_sprints in `.csdlc/evidence/864/all-issue-launch.json` and `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. Membership changes require explicit recorded scope and dependency reconciliation; never silently drop unfinished work.\n\n## Execution\n\nUse each child's existing numeric prerequisites and accepted output gates. Sprint numbering does not require completion of every earlier sprint. Run dependency-ready, disjoint work in parallel; serialize shared files and installed-binary ownership. Each child uses native v3 cards, a bound FastWork worktree, an issue-bound goal, focused PVF proof, and independent exact-head review. Child implementation stays in its own issue. Typed finish and cleanup are asynchronous and do not gate downstream implementation.\n\n## Completion and review\n\n- Account for every listed child exactly once with its reviewed implementation head, applicable passing checks, merge commit and ancestry to the sprint closing revision, or an explicit operator-authorized disposition that preserves unmet claims.\n- Verify actual source-specific acceptance, not merely closed issue state, generated schemas or a zero-test run.\n- Independently review the combined sprint result, integration boundaries, test adequacy and residual risks. Record lane-specific evidence and distinguish local proof, CI and real provider/cloud observations.\n- Record dependencies delivered to downstream issues and exact remaining limitations. Do not close the umbrella while a required undispositioned child or release-blocking finding remains.\n\n## Non-goals\n\nNo duplicate children, new product scope, broad all-to-all dependency, paid execution or publication authority, rewriting historical findings, or automatic merge/release. Podcast #671 remains its own sidecar.\n\n\n<!-- csdlc-v3-operation:a020888c7354b0874f634dcab677fbda32b5009a77d3db8e30d04d83e046ae4c -->",
  "number": 932,
  "state": "OPEN",
  "title": "[v0.92.2][Sprint 6] Hardware/provider qualification",
  "url": "https://github.com/agent-logic/agent-design-language/issues/932"
}


## Target Files / Surfaces

.csdlc/evidence/932 and .csdlc/issues/932 only; child evidence is read-only input.

## Validation Plan

Parse the ledger; verify 12/12 pass; authenticate child PR and issue states; verify merge ancestry, terminal receipts and worktree absence; run native six-card validation, independent exact-head review, diff checks and hosted CI.

## Demo / Proof Requirements

Child-specific actual hardware/provider evidence and independent integrated sprint review

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

No new child implementation, hardware rerun, production rollout, release approval, performance marketing, pooled-VRAM claim or rewriting child evidence.

## Notes / Risks

MLX proves only the tested adapter/Metal route; PAIR gains include heterogeneous hardware and do not pool VRAM; speculative decoding remains repair_inconclusive. These limits are results, not missing acceptance.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
