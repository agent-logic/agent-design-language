# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0909
Run ID: issue-0909
Version: v0.92.2
Title: [v0.92.2][OPS-GCP] Produce one apply-ready company GCP move-in execution packet
Branch: codex/909-gcp-move-in
Card Status: ready
Generated: <timestamp>

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/909
- PR:
- Source Issue Prompt: <source_issue_prompt>
- Docs: Operator explicitly approved RECOVERY_PROPOSAL.md via yes carry on: isolated private candidate from immutable17resource state, exactly3existing-resource local imports, then read-only plan. No cloud resource edits, remote state writes, backend migration, authoritative adoption or apply.
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
  output_card: <output_card>
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
- Source issue-prompt slug: 909-gcp-move-in
- Required outcome type: <required_outcome_type>
- Demo required: false

## Goal

<goal>

## Required Outcome

<required_outcome>

## Acceptance Criteria

<acceptance_criteria>

## Inputs

{
  "body": "# [v0.92.2][OPS-GCP] Finish the company GCP move-in execution packet\n\n## One complete result\n\nDeliver one complete apply-ready company GCP move-in packet containing current source/destination inventory, reused reviewed Terraform, ordered apply/rollback steps, ownership/billing/cleanup controls and residual routing. This is a required complete operations-planning deliverable; it does not claim an applied migration.\n\n## Owned surfaces\n\nUse `infra/gcp/organization/`, `infra/gcp/bootstrap/`, `infra/gcp/platform/` and their associated runbooks in `docs/operations/cloud/gcp/organization-billing/`, `terraform-bootstrap/` and `platform-foundation/`. Reuse merged v0.92.1 foundations instead of rebuilding them. Write the cohesive reviewed move-in packet under the issue's evidence directory and cross-link current operational docs as needed. Keep backend configuration, state, tfplans and credentials out of tracked/public artifacts. Local ignored move-in notes are rationale only; adopt necessary source-grounded requirements into the completed packet.\n\n## Acceptance\n\n1. Verify company organization/project/billing identity through approved read-only access; reject personal-project selection. Capture current source/destination ownership, dependencies, resource/data boundaries and prior-work disposition without exposing credentials.\n2. Reconcile the existing Terraform package to current inventory. Run formatting, validate and a bounded read-only plan using the approved environment; record actual commands/results and all proposed changes. No apply, state migration, import or backend creation is authorized here. A plan that cannot be produced is an explicit unresolved gap, not apply-ready completion.\n3. Finish the exact ordered application procedure with prerequisites, responsible owners, account checks, approvals required at application time, expected readbacks and go/no-go criteria. Complete rollback order and irreversible-boundary handling, with local/dry-run verification where possible.\n4. Include billing controls, resource limits, audit evidence, cleanup ownership and residual routing. Inventory/plan/rollback sections must describe the same exact selected infrastructure; an outline or copied template cannot close the issue.\n5. Independently review the complete packet and verify source links, structure, redaction and plan consistency. Preserve all seven required milestone planning tasks. No Runtime deployment, paid GPU launch or repeated six-resident qualification is included.\n\n## Dependencies and global start gate\n\nExecution prerequisites: WP-01, with accepted required outputs before dependent execution. OBS-LIVE is existing #720. Preserve any external foundation dependencies stated above and in the issue wave; no replacement of completed predecessor work.\n\nAll 69 milestone issue identities must be created and every creation-batch review must pass before any implementation starts. Batch grouping itself adds no execution dependency. Creation/review is distinct from implementation, deployment, manuscript publication and milestone closure.\n\n## Complete specification coverage\n\n- Acceptance obligations: company_identity_verified, prior_work_reused, terraform_package_reviewed, apply_and_rollback_order_explicit, billing_and_cleanup_controls_explicit, residuals_routed.\n- PVF obligations: readonly_inventory, terraform_fmt_validate_plan, packet_structure, redaction_scan.\n\n## Verification and truthful evidence\n\nUse the smallest proving checks for the actual result: source/structure/link/redaction and completeness validation for documents/inventories; real read-only cloud observations for inventories; actual authorized deployment/readback/browser evidence for OBS-S3; full revised prose and source/citation/editorial review for publication preparation. No schema/template/outline or unrelated green CI substitutes for the result. Preserve exact source revisions, observation times and explicit not-run/not-proven outcomes.\n\nPVF classification: deterministic local document/contract checks are local CPU and required issue gates, with source/identity/redaction negatives as applicable. Cloud readback, Terraform plan and browser/live-service checks are separate external-resource lanes with exact approved environment and evidence scope; do not label them deterministic offline proof. OBS-S3 live application has its explicit approval boundary. New tests record lane, proof role, determinism, resource profile and release-gate status in the tightly coupled manifest. Obtain independent review before publication; local structural checks alone do not establish content truth.\n\n## Non-goals, authority and stops\n\nRetain exclusions: runtime_deployment, paid_gpu_launch, six_resident_requalification. Stop conditions: personal_project_selected, mutation_without_operator_authority, credential_exposure, duplicate_v0921_scope. Treat missing execution authority, ownership collision, incomplete required evidence and privacy exposure as explicit blockers.\n\nUse native C-SDLC v3 with current authenticated authority and an issue-bound FastWork worktree/session goal for ADL work; root main stays inspection-only. Preserve inherited edits and historical evidence. Cloud/provider/account changes, infrastructure application and external publishing require their own explicit execution authority; creation alone supplies none. Do not print credentials, state secrets or private manuscript content. All seven planning tasks remain required and their completion is distinct from implemented Beta functionality.\n\n## Source basis\n\n`docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `ADR_PLAN_v0.92.2.md`, and the concrete source surfaces above. Resolve current source/ownership before execution; these issue bodies do not claim the described work has already happened.\n\n\n## Canonical execution links\n\nPlanning owner: #864. Creation/review batch: 8; this grouping adds no execution gate.\nExecution prerequisite: #864 (WP-01); accepted output is required before dependent execution.\n\nReviewed creation source: `0f877ab05d554e55b5de0e9cfa7aa40f69584346`. This issue records a complete task; creation does not claim execution or acceptance.\n\n\n<!-- csdlc-v3-operation:90f47b55a2c249d2fa7c9ad0ee3ad93ac41dad49ea65c1b5a0cd7d67a76815ba -->",
  "number": 909,
  "state": "OPEN",
  "title": "[v0.92.2][OPS-GCP] Produce one apply-ready company GCP move-in execution packet"
}


## Target Files / Surfaces

<target_files_surfaces>

## Validation Plan

<validation_plan>

## Demo / Proof Requirements

Read-only GCP and Terraform plan; local consistency/redaction checks; no apply or state changes

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

<non_goals>

## Notes / Risks

<notes_risks>

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
