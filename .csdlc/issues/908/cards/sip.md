# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0908
Run ID: issue-0908
Version: v0.92.2
Title: [v0.92.2][OPS-AWS] Produce one current AWS inventory packet from the #484 baseline
Branch: codex/908-aws-inventory
Card Status: ready
Generated: 2026-09-12T00:09:29.974020+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/908
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/908
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
  output_card: .csdlc/issues/908/cards/sor.md
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
- Source issue-prompt slug: 908-aws-inventory
- Required outcome type: evidence
- Demo required: false

## Goal

Current sanitized read-only business AWS inventory delta against immutable #484, including SCR, S3, model artifacts and maintenance record.

## Required Outcome

Current sanitized read-only business AWS inventory delta against immutable #484, including SCR, S3, model artifacts and maintenance record.

## Acceptance Criteria

Business account verified before reads; all historical scoped surfaces and current enabled regions represented; per-resource delta and explicit failures/unknown ownership; maintenance and negative completeness/redaction/staleness proof; independent evidence review.

## Inputs

{
  "body": "# [v0.92.2][OPS-AWS] Refresh the business AWS ownership inventory from #484\n\n## One complete result\n\nDeliver one current, sanitized incremental AWS inventory and maintenance record against completed #484. Reconcile SCR, S3, model artifacts and stale/unknown resources using actual read-only business-account observations. The work is the completed current inventory, not repeating the original move-in program or merely recommending an inventory.\n\n## Owned surfaces\n\nUse `docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md` and retained #484 command/evidence manifests under `docs/milestones/v0.92.1/evidence/cloud/aws-a/` as historical baseline. Preserve their original bytes; write the new dated delta and maintenance record under this issue's evidence directory, and update the current inventory index only where needed. Inspect `run-readonly-inventory.sh` and `build-inventory-summary.sh` there before reuse; adapt only bounded sanitization/completeness defects, without executing mutation commands.\n\n## Acceptance\n\n1. Resolve `agent-logic-admin` to the approved Agent Logic business account immediately before relying on cloud state. Do not use the personal/default account or copy account identifiers/credentials into new public artifacts. Keep sanitized identity-verification evidence.\n2. Observe current scoped global/regional inventory through read-only APIs, covering SCR/S3/model ownership and staleness. Compare with #484's denominator while explicitly recording current region/resource discovery, new/deleted/stale entries, read failures and excluded surfaces. Missing readback is not absence of a resource.\n3. Preserve dispositions owned, externally-owned, frozen-unknown, not-observed and read-failed; every change has current evidence and capture time. Do not convert unknown ownership into deletion permission.\n4. Finish the maintenance runbook with repeatable safe commands, refresh cadence/owner, evidence retention and staleness handling. Validate current delta completeness and redaction with wrong-account, missing-surface and stale-evidence cases.\n5. Independently review the actual readbacks/delta and remaining uncertainty. No migration, cleanup or deployment occurs; any required mutation is separately routed.\n\n## Dependencies and global start gate\n\nExecution prerequisites: WP-01, with accepted required outputs before dependent execution. OBS-LIVE is existing #720. Preserve any external foundation dependencies stated above and in the issue wave; no replacement of completed predecessor work.\n\nAll 69 milestone issue identities must be created and every creation-batch review must pass before any implementation starts. Batch grouping itself adds no execution dependency. Creation/review is distinct from implementation, deployment, manuscript publication and milestone closure.\n\n## Complete specification coverage\n\n- Acceptance obligations: issue_484_baseline_preserved, business_account_verified, readonly_inventory_current, staleness_explicit, secrets_absent.\n- PVF obligations: baseline_comparison, account_readback, inventory_validator, redaction_scan.\n\n## Verification and truthful evidence\n\nUse the smallest proving checks for the actual result: source/structure/link/redaction and completeness validation for documents/inventories; real read-only cloud observations for inventories; actual authorized deployment/readback/browser evidence for OBS-S3; full revised prose and source/citation/editorial review for publication preparation. No schema/template/outline or unrelated green CI substitutes for the result. Preserve exact source revisions, observation times and explicit not-run/not-proven outcomes.\n\nPVF classification: deterministic local document/contract checks are local CPU and required issue gates, with source/identity/redaction negatives as applicable. Cloud readback, Terraform plan and browser/live-service checks are separate external-resource lanes with exact approved environment and evidence scope; do not label them deterministic offline proof. OBS-S3 live application has its explicit approval boundary. New tests record lane, proof role, determinism, resource profile and release-gate status in the tightly coupled manifest. Obtain independent review before publication; local structural checks alone do not establish content truth.\n\n## Non-goals, authority and stops\n\nRetain exclusions: resource_migration, deployment. Stop conditions: personal_account, mutation_required, credential_exposure. Treat missing execution authority, ownership collision, incomplete required evidence and privacy exposure as explicit blockers.\n\nUse native C-SDLC v3 with current authenticated authority and an issue-bound FastWork worktree/session goal for ADL work; root main stays inspection-only. Preserve inherited edits and historical evidence. Cloud/provider/account changes, infrastructure application and external publishing require their own explicit execution authority; creation alone supplies none. Do not print credentials, state secrets or private manuscript content. All seven planning tasks remain required and their completion is distinct from implemented Beta functionality.\n\n## Source basis\n\n`docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `ADR_PLAN_v0.92.2.md`, and the concrete source surfaces above. Resolve current source/ownership before execution; these issue bodies do not claim the described work has already happened.\n\n\n## Canonical execution links\n\nPlanning owner: #864. Creation/review batch: 8; this grouping adds no execution gate.\nExecution prerequisite: #864 (WP-01); accepted output is required before dependent execution.\n\nReviewed creation source: `0f877ab05d554e55b5de0e9cfa7aa40f69584346`. This issue records a complete task; creation does not claim execution or acceptance.\n\n\n<!-- csdlc-v3-operation:6a5781aad8b06ae2e368313b220d1ce451daa0f3d7ba4ac303780a2868c75a28 -->",
  "number": 908,
  "state": "OPEN",
  "title": "[v0.92.2][OPS-AWS] Produce one current AWS inventory packet from the #484 baseline"
}


## Target Files / Surfaces

.csdlc/evidence/908/; docs/operations/cloud/aws/inventory/current index only; .csdlc/issues/908/cards/

## Validation Plan

python3 .csdlc/evidence/908/inventory.py capture; python3 .csdlc/evidence/908/inventory.py validate; python3 .csdlc/evidence/908/test_inventory.py; independent review of sanitized actual readbacks and delta

## Demo / Proof Requirements

Read-only AWS inventory plus deterministic completeness/redaction negatives

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

Cloud mutation, migration, deletion, deployment, GCP, Observatory infrastructure and modifications to #484 baseline.

## Notes / Risks

Read failures are not absence. Unknown ownership remains frozen. Raw identities, credentials and object contents never enter new evidence.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
