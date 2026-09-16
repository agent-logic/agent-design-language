# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-1003
Run ID: issue-1003
Version: v0.92.2
Title: [v0.92.2][C-SDLC] Restore rebind and validator replacement after scope amendments
Branch: codex/1003-scope-rebind-validator-recovery
Card Status: ready
Generated: 2026-09-16T02:43:15.759077+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/1003
- PR:
- Source Issue Prompt: .git/csdlc-v3/local/invocations/worker10-1003/source-issue.json
- Docs: docs/csdlc-v3/INTENT_COMMANDS.md
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
  output_card: .csdlc/issues/1003/cards/sor.md
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
- Source issue-prompt slug: scope-rebind-validator-recovery
- Required outcome type: behavioral_fix
- Demo required: yes

## Goal

Restore guarded rebind after scope amendments and permit replacing an inadmissible validator before proof.

## Required Outcome

A scope-rewound issue rebinds in its registered checkout at unchanged HEAD; changed HEAD refreshes binding without running old validators; typed validator replacement succeeds before proof; stale authority, branch/worktree mismatch, stale requests and pending recovery still fail closed; evidence invalidation remains explicit.

## Acceptance Criteria

A scope-rewound issue rebinds in its registered checkout at unchanged HEAD; changed HEAD refreshes binding without running old validators; typed validator replacement succeeds before proof; stale authority, branch/worktree mismatch, stale requests and pending recovery still fail closed; evidence invalidation remains explicit.

## Inputs

Retained #1003 reproduction and current source.

## Target Files / Surfaces

csdlc-v3/src/application/intent/{context,local}.rs; semantic binding owner; focused intent/semantic regression tests; docs/csdlc-v3/INTENT_COMMANDS.md

## Validation Plan

Focused Rust intent and semantic-owner tests, installed candidate rebind and validator replacement fixtures, negative identity/admission cases, cargo fmt and strict Clippy, then required GitHub CI tests and coverage. PVF deterministic local contract/regression proof; small CPU/local Git and isolated synthetic transport; required gate; no provider/cloud.

## Demo / Proof Requirements

Installed isolated candidate demonstrates both rebind paths and validator replacement without running inadmissible validator.

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

No raw state edits, automatic phase advancement, weaker validator admission, fallback authority or changes to #970.

## Notes / Risks

Preparation only; no implementation or proof claimed. Preserve all authority/version/topology guards. No arbitrary execution time or token limits.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
