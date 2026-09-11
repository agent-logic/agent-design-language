# [v0.92.2][OBS-S3] Deploy and verify the existing static Observatory sidecar

## One complete result

Deploy the exact reviewed Observatory static revision at `observatory.csm.agent-logic.ai` using the existing #679/merged #685 Terraform package, then verify live browser HTTPS and Runtime WSS behavior plus infrastructure posture. A reviewed plan or Terraform validation alone is not the deployed result.

## Owned surfaces

Own bounded application/deployment evidence for `infra/aws/observatory/` and the existing static Observatory bundle identified by that root's README. Preserve the architecture: private versioned S3, CloudFront OAC, ACM/Route53, access logs and explicit security headers. Use the repository's existing bounded readback helper after inspecting its authorization contract. Retain exact bundle/revision/object versions, plan digest, apply outcome, invalidation and browser evidence in the issue's deployment packet. No Runtime compute infrastructure is created here.

## Acceptance

1. After the global creation gate and dependencies, obtain/reconfirm explicit authorization for the precise business-account apply and asset upload. Use only `agent-logic-admin`; verify the business identity, DNS/certificate authority and exact Runtime HTTPS/WSS origins. No live apply is authorized merely by creating this issue.
2. Validate and review the exact Terraform plan before applying. Reject unexpected destructive changes or architecture redesign. Apply the reviewed package, upload the exact versioned static assets with correct cache headers, and verify CloudFront invalidation completed.
3. Authenticated AWS readbacks prove private S3 origin/OAC, CloudFront/ACM/Route53 convergence, access logging/retention and security headers. Retain sanitized configuration posture rather than raw state, account identifiers, tokens or private keys.
4. Open the deployed site in a browser over HTTPS and connect to the declared live Runtime over WSS using the existing browser-to-Runtime authentication boundary. Verify observed data is live following OBS-LIVE/#720; no retained telemetry presented as live. Record exact deployed revision and actual successful/failed checks.
5. Rehearse or dry-run rollback to prior versioned content with invalidation, document infrastructure-change rollback separately, and finish cost/monitoring/ownership records. No bucket/distribution deletion is a content rollback mechanism.
6. Independent review must reconcile plan/apply, deployed assets, authenticated readbacks, cache invalidation and browser/WSS evidence. OBS-S3 is outside early CF-INTEGRATE/TAIL-01 gates but its completed deployment acceptance explicitly gates TAIL-10.

## Dependencies and global start gate

Execution prerequisites: WP-01, OBS-LIVE, with accepted required outputs before dependent execution. OBS-LIVE is existing #720. Preserve any external foundation dependencies stated above and in the issue wave; no replacement of completed predecessor work.

All 69 milestone issue identities must be created and every creation-batch review must pass before any implementation starts. Batch grouping itself adds no execution dependency. Creation/review is distinct from implementation, deployment, manuscript publication and milestone closure.

## Complete specification coverage

- Acceptance obligations: agent_logic_admin_profile_resolves_business_account, terraform_plan_reviewed_before_apply, private_s3_origin_uses_oac, cloudfront_acm_route53_converged, access_logging_configured, security_headers_configured, exact_observatory_revision_deployed, cache_invalidation_completed, browser_https_and_runtime_wss_pass, rollback_and_cost_recorded.
- PVF obligations: agent_logic_admin_identity_readback, terraform_validate_and_plan, authenticated_aws_resource_readback, logging_and_security_header_readback, cache_invalidation_readback, browser_https_smoke, runtime_wss_smoke, rollback_rehearsal_or_dry_run.

## Verification and truthful evidence

Use the smallest proving checks for the actual result: source/structure/link/redaction and completeness validation for documents/inventories; real read-only cloud observations for inventories; actual authorized deployment/readback/browser evidence for OBS-S3; full revised prose and source/citation/editorial review for publication preparation. No schema/template/outline or unrelated green CI substitutes for the result. Preserve exact source revisions, observation times and explicit not-run/not-proven outcomes.

PVF classification: deterministic local document/contract checks are local CPU and required issue gates, with source/identity/redaction negatives as applicable. Cloud readback, Terraform plan and browser/live-service checks are separate external-resource lanes with exact approved environment and evidence scope; do not label them deterministic offline proof. OBS-S3 live application has its explicit approval boundary. New tests record lane, proof role, determinism, resource profile and release-gate status in the tightly coupled manifest. Obtain independent review before publication; local structural checks alone do not establish content truth.

## Non-goals, authority and stops

Retain exclusions: customer_scale_hosting, runtime_compute_deployment, terraform_architecture_redesign, multi_tenant_observatory. Stop conditions: wrong_aws_account, unexpected_destructive_plan, credential_or_secret_exposure, runtime_origin_unavailable, architecture_redesign_required. Treat missing execution authority, ownership collision, incomplete required evidence and privacy exposure as explicit blockers.

Use native C-SDLC v3 with current authenticated authority and an issue-bound FastWork worktree/session goal for ADL work; root main stays inspection-only. Preserve inherited edits and historical evidence. Cloud/provider/account changes, infrastructure application and external publishing require their own explicit execution authority; creation alone supplies none. Do not print credentials, state secrets or private manuscript content. All seven planning tasks remain required and their completion is distinct from implemented Beta functionality.

## Source basis

`docs/milestones/v0.92.2/WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml`, `ATOMIC_TASK_CONTRACTS_v0.92.2.json`, `CREATION_SELECTIONS_v0.92.2.md`, `ADR_PLAN_v0.92.2.md`, and the concrete source surfaces above. Resolve current source/ownership before execution; these issue bodies do not claim the described work has already happened.
