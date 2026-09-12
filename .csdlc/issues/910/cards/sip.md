# ADL Input Card

Semantic role: Structured Issue Prompt (`SIP`).
Canonical Template Source: `docs/templates/prompts/1.0.5/sip.md`

Task ID: issue-0910
Run ID: issue-0910
Version: v0.92.2
Title: [v0.92.2][OBS-S3] Deploy the existing Observatory S3 and CloudFront sidecar
Branch: codex/910-observatory-deploy
Card Status: ready
Generated: 2026-09-12T01:34:22.125782+00:00

Context:
- Issue: https://github.com/agent-logic/agent-design-language/issues/910
- PR:
- Source Issue Prompt: https://github.com/agent-logic/agent-design-language/issues/910
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
  output_card: .csdlc/issues/910/cards/sor.md
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
- Source issue-prompt slug: 910-observatory-deploy
- Required outcome type: deployment
- Demo required: false

## Goal

Prepare exact reviewed Observatory assets/infrastructure deployment and approval packet; after explicit precise approval deploy and prove live HTTPS/WSS. This preparation phase performs no cloud writes.

## Required Outcome

Prepare exact reviewed Observatory assets/infrastructure deployment and approval packet; after explicit precise approval deploy and prove live HTTPS/WSS. This preparation phase performs no cloud writes.

## Acceptance Criteria

Business identity/DNS/certificate/current Runtime origins verified; exact merged #720 asset hashes; bounded Terraform plan from verified state; rollback/cache/cost/owners specified; independent review before precise apply/upload approval; later actual deployment/readback/browser evidence required for full #910 acceptance.

## Inputs

{
  "body": "# [v0.92.2][OBS-S3] Deploy and verify the existing static Observatory sidecar\n\n## One complete result\n\nDeploy the exact reviewed Observatory static revision at `observatory.csm.agent-logic.ai` using the existing #679/merged #685 Terraform package, then verify live browser HTTPS and Runtime WSS behavior plus infrastructure posture. A reviewed plan or Terraform validation alone is not the deployed result.\n\n## Owned surfaces\n\nOwn bounded application/deployment evidence for `infra/aws/observatory/` and the existing static Observatory bundle identified by that root's README. Preserve the architecture: private versioned S3, CloudFront OAC, ACM/Route53, access logs and explicit security headers. Use the repository's existing bounded readback helper after inspecting its authorization contract. Retain exact bundle/revision/object versions, plan digest, apply outcome, invalidation and browser evidence in the issue's deployment packet. No Runtime compute infrastructure is created here.\n\n## Acceptance\n\n1. After the global creation gate and dependencies, obtain/reconfirm explicit authorization for the precise business-account apply and asset upload. Use only `agent-logic-admin`; verify the business identity, DNS/certificate authority and exact Runtime HTTPS/WSS origins. No live apply is authorized merely by creating this issue.\n2. Validate and review the exact Terraform plan before applying. Reject unexpected destructive changes or architecture redesign. Apply the reviewed package, upload the exact versioned static assets with correct cache headers, and verify CloudFront invalidation completed.\n3. Authenticated AWS readbacks prove private S3 origin/OAC, CloudFront/ACM/Route53 convergence, access logging/retention and security headers. Retain sanitized configuration posture rather than raw state, account identifiers, tokens or private keys.\n4. Open the deployed site in a browser over HTTPS and connect to the declared live Runtime over WSS using the existing browser-to-Runtime authentication boundary. Verify observed data is live following OBS-LIVE/#720; no retained telemetry presented as live. Record exact deployed revision and actual successful/failed checks.\n5. Rehearse or dry-run rollback to prior versioned content with invalidation, document infrastructure-change rollback separately, and finish cost/monitoring/ownership records. No bucket/distribution deletion is a content rollback mechanism.\n6. Independent review must reconcile plan/apply, deployed assets, authenticated readbacks, cache invalidation and browser/WSS evidence. OBS-S3 is outside early CF-INTEGRATE/TAIL-01 gates but its completed deployment acceptance explicitly gates TAIL-10.\n\n## Dependencies and global start gate\n\nExecution prerequisites: WP-01, OBS-LIVE, with accepted required outputs before dependent execution. OBS-LIVE is existing #720. Preserve any external foundation dependencies stated above and in the issue wave; no replacement of completed predecessor work.\n\nAll 69 milestone issue identities must be created and every creation-batch review must pass before any implementation starts. Batch grouping itself adds no execution dependency. Creation/review is distinct from implementation, deployment, manuscript publication and milestone closure.\n\n## Complete specification coverage\n\n- Acceptance obligations: agent_logic_admin_profile_resolves_business_account, terraform_plan_reviewed_before_apply, private_s3_origin_uses_oac, cloudfront_acm_route53_converged, access_logging_configured, security_headers_configured, exact_observatory_revision_deployed, cache_invalidation_completed, browser_https_and_runtime_wss_pass, rollback_and_cost_recorded.\n- PVF obligations: agent_logic_admin_identity_readback, terraform_validate_and_plan, authenticated_aws_resource_readback, logging_and_security_header_readback, cache_invalidation_readback, browser_https_smoke, runtime_wss_smoke, rollback_rehearsal_or_dry_run.\n\n## Verification and truthful evidence\n\nUse the smallest proving checks for the actual result: source/structure/link/redaction and completeness validation for documents/inventories; real read-only cloud observations for inventories; actual authorized deployment/readback/browser evidence for OBS-S3; full revised prose and source/citation/editorial review for publication preparation. No schema/template/outline or unrelated green CI substitutes for the result. Preserve exact source revisions, observation times and explicit not-run/not-proven outcomes.\n\nPVF classification: deterministic local document/contract checks are local CPU and required issue gates, with source/identity/redaction negatives as applicable. Cloud readback, Terraform plan and browser/live-service checks are separate external-resource lanes with exact approved environment and evidence scope; do not label them deterministic offline proof. OBS-S3 live application has its explicit approval boundary. New tests record lane, proof role, determinism, resource profile and release-gate status in the tightly coupled manifest. Obtain independent review before publication; local structural checks alone do not establish content truth.\n\n## Non-goals, authority and stops\n\nRetain exclusions: customer_scale_hosting, runtime_compute_deployment, terraform_architecture_redesign, multi_tenant_observatory. Stop conditions: wrong_aws_account, unexpected_destructive_plan, credential_or_secret_exposure, runtime_origin_unavailable, architecture_redesign_required. Treat missing execution authority, ownership collision, incomplete required evidence and privacy exposure as explicit blockers.\n\nUse native C-SDLC v3 with current authenticated authority and an issue-bound FastWork worktree/session goal for ADL work; root main stays inspection-only. Preserve inherited edits and historical evidence. Cloud/provider/account changes, infrastructure application and external publishing require their own explicit execution authority; creation alone supplies none. Do not print credentials, state secrets or private manuscript content. All seven planning tasks remain required and their completion is distinct from implemented Beta functionality.\n\n## Source basis\n\n`docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `ADR_PLAN_v0.92.2.md`, and the concrete source surfaces above. Resolve current source/ownership before execution; these issue bodies do not claim the described work has already happened.\n\n\n## Canonical execution links\n\nPlanning owner: #864. Creation/review batch: 8; this grouping adds no execution gate.\nExecution prerequisite: #864 (WP-01); accepted output is required before dependent execution.\nExecution prerequisite: #720 (OBS-LIVE); accepted output is required before dependent execution.\n\nReviewed creation source: `0f877ab05d554e55b5de0e9cfa7aa40f69584346`. This issue records a complete task; creation does not claim execution or acceptance.\n\n\n<!-- csdlc-v3-operation:8710c9caf76b2c19d50cf123999b6241b769f0a6310ca45989fd82f399e40f18 -->",
  "number": 910,
  "state": "OPEN",
  "title": "[v0.92.2][OBS-S3] Deploy the existing Observatory S3 and CloudFront sidecar"
}


## Target Files / Surfaces

infra/aws/observatory/ existing package; merged #720 static bundle; .csdlc/evidence/910/; .csdlc/issues/910/cards/

## Validation Plan

Existing Observatory Terraform/static validator; exact static asset hash/secret checks; explicit read-only business AWS/DNS/Runtime preflight; isolated backend-disabled Terraform fmt/validate/plan; independent plan/evidence review. No apply/upload/invalidation.

## Demo / Proof Requirements

Plan/readback/browser live lanes; actual apply/upload blocked until explicit approval; no Runtime compute

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

Runtime compute, architecture redesign, personal AWS, cloud writes before explicit approval, bulk resource changes.

## Notes / Risks

Stop on unavailable live Runtime origins, wrong identity, unknown state/custody, destructive plan or secret exposure. Preparation is not deployed completion.

## Instructions to the Agent
- Read this file.
- Read the linked source issue prompt before starting work.
- Do not create a branch or worktree from this card alone.
- When execution is approved, run native v3 `csdlc bind` and then perform the work described above.
- Write execution outcome truth to the paired `sor.md` file during execution.
