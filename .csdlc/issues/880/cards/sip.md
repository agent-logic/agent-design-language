# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0880
Run ID: issue-0880
Version: v0.92.2
Title: [v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet
Branch: codex/880-ci-ingestion
Card Status: ready
Generated: 2026-09-11T23:55:27.199943+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/880
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/880
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
  output_card: .csdlc/issues/880/cards/sor.md
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
- Source issue-prompt slug: 880-ci-ingestion
- Required outcome type: production-behavior
- Demo required: yes

## Goal

The CI ingestion entrypoint consumes declared checkout/revision/scope inputs and emits a usable common packet with truthful partial-input state. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); shared command/path decisions belong to WP-01/#864. This task provides CI input acquisition, not architecture fitness gating (CF-GOV-CI).

## Required Outcome

The CI ingestion entrypoint consumes declared checkout/revision/scope inputs and emits a usable common packet with truthful partial-input state. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); shared command/path decisions belong to WP-01/#864. This task provides CI input acquisition, not architecture fitness gating (CF-GOV-CI).

## Acceptance Criteria

1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope. 2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets. 3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials. 4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data. 5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success.

## Inputs

Source issue, canonical v0.92.2 contracts and accepted dependency outputs.

## Target Files / Surfaces

Reuse the predecessor's selected installed command and `adl/src/codefriend/ingestion/` contract. Own proposed `adl/src/codefriend/ingestion/ci.rs`, narrow CLI registration, focused `adl/tests/codefriend_ci_ingestion.rs` and one named `.github/workflows/` smoke job chosen in the issue plan. The workflow must call the installed product path; it is not a competing ingestion implementation. Read the portable-adapter feature and adopted contracts before execution.

## Validation Plan

1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope. 2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets. 3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials. 4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data. 5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success. PVF: deterministic local contract/installed integration followed by required CI smoke; role: declared revision, packet parity and partial/credential negatives; resources: bounded runner CPU/disk, isolated fixture repository, no metered inference; gate: required Beta CI input route. Retain exact command and nonzero outcome in CI independently from local proof. Stop on missing input, failed required proof, contract conflict, credential capture, host-bound packet or unexecuted CI gate. Exclude CF-GOV-CI, GitHub API adapter, review/features, broad connectors and schema/scaffold-only closure.

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

PVF: deterministic local contract/installed integration followed by required CI smoke; role: declared revision, packet parity and partial/credential negatives; resources: bounded runner CPU/disk, isolated fixture repository, no metered inference; gate: required Beta CI input route. Retain exact command and nonzero outcome in CI independently from local proof. Stop on missing input, failed required proof, contract conflict, credential capture, host-bound packet or unexecuted CI gate. Exclude CF-GOV-CI, GitHub API adapter, review/features, broad connectors and schema/scaffold-only closure.

## Notes / Risks

Prepared only. Child implementation and its review have not run.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
