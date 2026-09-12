# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0934
Run ID: issue-0934
Version: v0.92.2
Title: [v0.92.2][Sprint 8] Cloud operations and Observatory
Branch: codex/934-sprint8-coordination
Card Status: ready
Generated: 2026-09-12T00:09:30.099416+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/934
- PR:
- Source Issue Prompt: .csdlc/evidence/934/source-issue.md
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
  output_card: .csdlc/issues/934/cards/sor.md
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
- Source issue-prompt slug: 934-sprint8-coordination
- Required outcome type: sprint coordination and acceptance
- Demo required: false

## Goal

Complete the four-child Sprint 8 result through governed independent execution and integrated review

## Required Outcome

One truthful reviewed sprint result covering #720/#908/#909/#910

## Acceptance Criteria

Every child accepted on actual source-specific proof or explicitly authorized disposition; independent combined review; no required unresolved child or finding

## Inputs

{
  "body": "## Outcome\n\nCoordinate Sprint 8 \u2014 Cloud operations and Observatory through one complete reviewed execution result.\n\n## Existing child roster\n\n- #720 \u2014 OBS-LIVE\n- #908 \u2014 OPS-AWS\n- #909 \u2014 OPS-GCP\n- #910 \u2014 OBS-S3\n\n## Management authority\n\nPart of #926. Created by explicit operator instruction that every execution sprint needs an umbrella. This management issue is outside the unchanged 69-core-task denominator. Source: execution_sprints in `.csdlc/evidence/864/all-issue-launch.json` and `docs/milestones/v0.92.2/SPRINT_v0.92.2.md`. Membership changes require explicit recorded scope and dependency reconciliation; never silently drop unfinished work.\n\n## Execution\n\nUse each child's existing numeric prerequisites and accepted output gates. Sprint numbering does not require completion of every earlier sprint. Run dependency-ready, disjoint work in parallel; serialize shared files and installed-binary ownership. Each child uses native v3 cards, a bound FastWork worktree, an issue-bound goal, focused PVF proof, and independent exact-head review. Child implementation stays in its own issue. Typed finish and cleanup are asynchronous and do not gate downstream implementation.\n\n## Completion and review\n\n- Account for every listed child exactly once with its reviewed implementation head, applicable passing checks, merge commit and ancestry to the sprint closing revision, or an explicit operator-authorized disposition that preserves unmet claims.\n- Verify actual source-specific acceptance, not merely closed issue state, generated schemas or a zero-test run.\n- Independently review the combined sprint result, integration boundaries, test adequacy and residual risks. Record lane-specific evidence and distinguish local proof, CI and real provider/cloud observations.\n- Record dependencies delivered to downstream issues and exact remaining limitations. Do not close the umbrella while a required undispositioned child or release-blocking finding remains.\n\n## Non-goals\n\nNo duplicate children, new product scope, broad all-to-all dependency, paid execution or publication authority, rewriting historical findings, or automatic merge/release. Podcast #671 remains its own sidecar.\n\n\n<!-- csdlc-v3-operation:a947df09c776992cd1bd4754b6943762c926ef8defe3a61bfd1c0e9b2fcb98b7 -->",
  "number": 934,
  "state": "OPEN",
  "title": "[v0.92.2][Sprint 8] Cloud operations and Observatory"
}


## Target Files / Surfaces

.csdlc/evidence/934 and .csdlc/issues/934 only

## Validation Plan

Verify four-member roster, exact child heads and CI, actual cloud/browser proof, merge ancestry and independent review; do not duplicate broad Rust runs

## Demo / Proof Requirements

Child-specific actual evidence and independent integrated sprint review

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

Child implementation, automatic merge, cloud mutation without explicit authorization, release publication

## Notes / Risks

No implementation or external effect is authorized by a structural readiness pass. Child session goals remain separate.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
