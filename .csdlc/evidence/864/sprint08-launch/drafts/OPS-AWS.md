# [v0.92.2][OPS-AWS] Refresh the business AWS ownership inventory from #484

## One complete result

Deliver one current, sanitized incremental AWS inventory and maintenance record against completed #484. Reconcile SCR, S3, model artifacts and stale/unknown resources using actual read-only business-account observations. The work is the completed current inventory, not repeating the original move-in program or merely recommending an inventory.

## Owned surfaces

Use `docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md` and retained #484 command/evidence manifests under `docs/milestones/v0.92.1/evidence/cloud/aws-a/` as historical baseline. Preserve their original bytes; write the new dated delta and maintenance record under this issue's evidence directory, and update the current inventory index only where needed. Inspect `run-readonly-inventory.sh` and `build-inventory-summary.sh` there before reuse; adapt only bounded sanitization/completeness defects, without executing mutation commands.

## Acceptance

1. Resolve `agent-logic-admin` to the approved Agent Logic business account immediately before relying on cloud state. Do not use the personal/default account or copy account identifiers/credentials into new public artifacts. Keep sanitized identity-verification evidence.
2. Observe current scoped global/regional inventory through read-only APIs, covering SCR/S3/model ownership and staleness. Compare with #484's denominator while explicitly recording current region/resource discovery, new/deleted/stale entries, read failures and excluded surfaces. Missing readback is not absence of a resource.
3. Preserve dispositions owned, externally-owned, frozen-unknown, not-observed and read-failed; every change has current evidence and capture time. Do not convert unknown ownership into deletion permission.
4. Finish the maintenance runbook with repeatable safe commands, refresh cadence/owner, evidence retention and staleness handling. Validate current delta completeness and redaction with wrong-account, missing-surface and stale-evidence cases.
5. Independently review the actual readbacks/delta and remaining uncertainty. No migration, cleanup or deployment occurs; any required mutation is separately routed.

## Dependencies and global start gate

Execution prerequisites: WP-01, with accepted required outputs before dependent execution. OBS-LIVE is existing #720. Preserve any external foundation dependencies stated above and in the issue wave; no replacement of completed predecessor work.

All 69 milestone issue identities must be created and every creation-batch review must pass before any implementation starts. Batch grouping itself adds no execution dependency. Creation/review is distinct from implementation, deployment, manuscript publication and milestone closure.

## Complete specification coverage

- Acceptance obligations: issue_484_baseline_preserved, business_account_verified, readonly_inventory_current, staleness_explicit, secrets_absent.
- PVF obligations: baseline_comparison, account_readback, inventory_validator, redaction_scan.

## Verification and truthful evidence

Use the smallest proving checks for the actual result: source/structure/link/redaction and completeness validation for documents/inventories; real read-only cloud observations for inventories; actual authorized deployment/readback/browser evidence for OBS-S3; full revised prose and source/citation/editorial review for publication preparation. No schema/template/outline or unrelated green CI substitutes for the result. Preserve exact source revisions, observation times and explicit not-run/not-proven outcomes.

PVF classification: deterministic local document/contract checks are local CPU and required issue gates, with source/identity/redaction negatives as applicable. Cloud readback, Terraform plan and browser/live-service checks are separate external-resource lanes with exact approved environment and evidence scope; do not label them deterministic offline proof. OBS-S3 live application has its explicit approval boundary. New tests record lane, proof role, determinism, resource profile and release-gate status in the tightly coupled manifest. Obtain independent review before publication; local structural checks alone do not establish content truth.

## Non-goals, authority and stops

Retain exclusions: resource_migration, deployment. Stop conditions: personal_account, mutation_required, credential_exposure. Treat missing execution authority, ownership collision, incomplete required evidence and privacy exposure as explicit blockers.

Use native C-SDLC v3 with current authenticated authority and an issue-bound FastWork worktree/session goal for ADL work; root main stays inspection-only. Preserve inherited edits and historical evidence. Cloud/provider/account changes, infrastructure application and external publishing require their own explicit execution authority; creation alone supplies none. Do not print credentials, state secrets or private manuscript content. All seven planning tasks remain required and their completion is distinct from implemented Beta functionality.

## Source basis

`docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `ADR_PLAN_v0.92.2.md`, and the concrete source surfaces above. Resolve current source/ownership before execution; these issue bodies do not claim the described work has already happened.
