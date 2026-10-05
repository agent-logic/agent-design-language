# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1150
Run ID: issue-1150
Version: 1.0.5
Title: [v0.93][CodeFriend] Complete independent Beta 1 installed qualification
Branch: codex/1150-codefriend-independent-qualification
Card Status: ready
Generated: 2026-10-04T06:03:30Z

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1150
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/1150
- Docs: docs/milestones/v0.93.1/WP_EXECUTION_SPECIFICATIONS_v0.93.1.yaml; docs/milestones/v0.93.1/features/CODEFRIEND_LAUNCH_v0.93.1.md
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
  output_card: .git/csdlc-v3/local/projections/1150/cards/sor.md
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
- Source issue-prompt slug: cf05-independent-beta1-qualification
- Required outcome type: installed_independent_qualification_decision
- Demo required: true

## Goal

Produce one independent Beta 1 qualification decision for the exact installed candidate against the retained 12-cell and 24-obligation denominator after every prerequisite is accepted.

## Required Outcome

A complete matrix and exact-candidate packet records actual outcomes for all 12 contract-derived coverage tuples and all 24 original obligations, with missing or failed items blocking launch.

## Acceptance Criteria

Cover macOS and Linux x ADL and external repository x hosted-mode website build, local-agent website and CLI as 12 contract-derived tuples; reconcile all 24 original obligations without inventing unavailable historical cell IDs; repeat ADL, external OSS and PR-review journeys; assess source-grounded evidence, diagrams, meaningful generated tests, documentation, redaction, report quality and human HTML/PDF inspection; bind actual counts and results to exact installed versions; prove the waiting-list mechanism and hosted-mode path are built, tested and deployable without requiring public deployment; preserve refusal, cancellation, interruption, recovery, cost-control, deletion and rollback outcomes; any missing or failed obligation blocks Beta 1 readiness.

## Inputs

Issue #1150; CF-05 in WP_EXECUTION_SPECIFICATIONS_v0.93.1.yaml; CODEFRIEND_LAUNCH_v0.93.1.md; retained #915/#916 evidence, including the SHA-256-identified Q01-Q24 requirement map; accepted CF-04 #34, CT-05 #44, INTEGRATE #45, ADL #1148 and #1149 results; exact installed candidate identity. Historical aggregate is 0/12 accepted; all 24 mapped obligations remain unproven and six exports are partial evidence only.

## Target Files / Surfaces

Issue-local qualification matrix, exact-candidate evidence packet, external-tester rehearsal results, waiting-list and hosted-mode deployability proof, independent review, and readiness-blocking disposition. No product repair or public deployment surface is owned here.

## Validation Plan

Plan an installed-integration qualification only after all five prerequisites and the exact candidate are accepted. Reuse unchanged retained evidence only when identity and scope match; execute the unresolved denominator and record actual counts. Independent review must resolve actionable findings.

## Demo / Proof Requirements

The declared 12-tuple/24-obligation denominator and external-tester journeys are the proof surface. Preparation, exports, schema checks or zero executed scenarios do not count.

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

No product repairs, provider replay, paid spend, public deployment, audience activation, public launch, inferred acceptance, invented private cell labels, or duplicate CF-05 issue.

## Notes / Risks

Preparation only. v0.93.1 ends with a deployable, tested candidate and waiting-list mechanism; public deployment and live launch occur later. The original per-cell private packet remains absent. The retained #915 handoff supplies exact Q01-Q24 requirement text but no executed scenario IDs or qualification credit. Do not infer prerequisite acceptance from issue closure; require exact accepted producer evidence, exact installed candidate custody, applicable provider authority for genuinely provider-backed local scenarios, and complete prompt-only SRP readiness before qualification execution.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
