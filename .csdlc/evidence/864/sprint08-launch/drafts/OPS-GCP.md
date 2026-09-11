# [v0.92.2][OPS-GCP] Finish the company GCP move-in execution packet

## One complete result

Deliver one complete apply-ready company GCP move-in packet containing current source/destination inventory, reused reviewed Terraform, ordered apply/rollback steps, ownership/billing/cleanup controls and residual routing. This is a required complete operations-planning deliverable; it does not claim an applied migration.

## Owned surfaces

Use `infra/gcp/organization/`, `infra/gcp/bootstrap/`, `infra/gcp/platform/` and their associated runbooks in `docs/operations/cloud/gcp/organization-billing/`, `terraform-bootstrap/` and `platform-foundation/`. Reuse merged v0.92.1 foundations instead of rebuilding them. Write the cohesive reviewed move-in packet under the issue's evidence directory and cross-link current operational docs as needed. Keep backend configuration, state, tfplans and credentials out of tracked/public artifacts. Local ignored move-in notes are rationale only; adopt necessary source-grounded requirements into the completed packet.

## Acceptance

1. Verify company organization/project/billing identity through approved read-only access; reject personal-project selection. Capture current source/destination ownership, dependencies, resource/data boundaries and prior-work disposition without exposing credentials.
2. Reconcile the existing Terraform package to current inventory. Run formatting, validate and a bounded read-only plan using the approved environment; record actual commands/results and all proposed changes. No apply, state migration, import or backend creation is authorized here. A plan that cannot be produced is an explicit unresolved gap, not apply-ready completion.
3. Finish the exact ordered application procedure with prerequisites, responsible owners, account checks, approvals required at application time, expected readbacks and go/no-go criteria. Complete rollback order and irreversible-boundary handling, with local/dry-run verification where possible.
4. Include billing controls, resource limits, audit evidence, cleanup ownership and residual routing. Inventory/plan/rollback sections must describe the same exact selected infrastructure; an outline or copied template cannot close the issue.
5. Independently review the complete packet and verify source links, structure, redaction and plan consistency. Preserve all seven required milestone planning tasks. No Runtime deployment, paid GPU launch or repeated six-resident qualification is included.

## Dependencies and global start gate

Execution prerequisites: WP-01, with accepted required outputs before dependent execution. OBS-LIVE is existing #720. Preserve any external foundation dependencies stated above and in the issue wave; no replacement of completed predecessor work.

All 69 milestone issue identities must be created and every creation-batch review must pass before any implementation starts. Batch grouping itself adds no execution dependency. Creation/review is distinct from implementation, deployment, manuscript publication and milestone closure.

## Complete specification coverage

- Acceptance obligations: company_identity_verified, prior_work_reused, terraform_package_reviewed, apply_and_rollback_order_explicit, billing_and_cleanup_controls_explicit, residuals_routed.
- PVF obligations: readonly_inventory, terraform_fmt_validate_plan, packet_structure, redaction_scan.

## Verification and truthful evidence

Use the smallest proving checks for the actual result: source/structure/link/redaction and completeness validation for documents/inventories; real read-only cloud observations for inventories; actual authorized deployment/readback/browser evidence for OBS-S3; full revised prose and source/citation/editorial review for publication preparation. No schema/template/outline or unrelated green CI substitutes for the result. Preserve exact source revisions, observation times and explicit not-run/not-proven outcomes.

PVF classification: deterministic local document/contract checks are local CPU and required issue gates, with source/identity/redaction negatives as applicable. Cloud readback, Terraform plan and browser/live-service checks are separate external-resource lanes with exact approved environment and evidence scope; do not label them deterministic offline proof. OBS-S3 live application has its explicit approval boundary. New tests record lane, proof role, determinism, resource profile and release-gate status in the tightly coupled manifest. Obtain independent review before publication; local structural checks alone do not establish content truth.

## Non-goals, authority and stops

Retain exclusions: runtime_deployment, paid_gpu_launch, six_resident_requalification. Stop conditions: personal_project_selected, mutation_without_operator_authority, credential_exposure, duplicate_v0921_scope. Treat missing execution authority, ownership collision, incomplete required evidence and privacy exposure as explicit blockers.

Use native C-SDLC v3 with current authenticated authority and an issue-bound FastWork worktree/session goal for ADL work; root main stays inspection-only. Preserve inherited edits and historical evidence. Cloud/provider/account changes, infrastructure application and external publishing require their own explicit execution authority; creation alone supplies none. Do not print credentials, state secrets or private manuscript content. All seven planning tasks remain required and their completion is distinct from implemented Beta functionality.

## Source basis

`docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `ADR_PLAN_v0.92.2.md`, and the concrete source surfaces above. Resolve current source/ownership before execution; these issue bodies do not claim the described work has already happened.
