# GCP hierarchy and cost decision

Issue: #490

Status: accepted decision register for the GCP-A hierarchy and cost denominator.

## Scope

This register records the GCP-A hierarchy and cost decision for the v0.92.1 GCP move-in lane. It is read-only decision evidence only. It does not authorize API enablement, project creation, Terraform apply, IAM mutation, billing mutation, quota purchase, GPU launch, production traffic, or service-account key creation.

Evidence root:

- `docs/milestones/v0.92.1/evidence/cloud/gcp-a/readbacks/`

## Accepted hierarchy and billing decision

| Surface | Accepted decision | Evidence |
| --- | --- | --- |
| Organization | Use the company Organization `organizations/321515087273` (`agent-logic.ai`). | `readbacks/organizations.json` |
| Folder | Keep POC resources under `folders/726824330959` (`Proof of Concept`). Use `folders/929563862525` (`gcp-internal-cloud-setup`) as the long-term GCP foundation folder for #491+. | `readbacks/folders-321515087273.json` |
| Project | Treat `cs-poc-cha8mmii0xk0iaw5vpf8mxf` as the existing POC/shared-services project only. Use `cs-host-377d41e71a824f92802120` (`Cloud Setup Host Project`) as the long-term Terraform/bootstrap host project for the next GCP issues unless a later issue explicitly replaces it. | `readbacks/config.json`, `readbacks/projects.json`, `readbacks/project-cs-host-377d41e71a824f92802120_.json` |
| Billing | Use `billingAccounts/01FA88-CC4968-ADF817` (`My Billing Account`), open, USD, parented to `organizations/321515087273`. The selected host project has billing enabled against this account. | `readbacks/billing-accounts.json`, `readbacks/billing-account-01FA88-CC4968-ADF817_.json`, `readbacks/project-billing-cs-host-377d41e71a824f92802120_.json` |
| Region | Use `us-west2` as the first-workload primary Region. Keep `us-central1` as an explicit later fallback only if a future issue records a reason. | `readbacks/compute-regions.json` |
| Data residency | US-only for the first workload envelope. No non-US Region is accepted by this decision. | `readbacks/compute-regions.json`, this register |
| Credit expiry | No credit is accepted as part of the cost basis. The billing readback proves the account is open and USD, but it does not expose credit grants or expiry. Therefore the accepted denominator is: do not rely on credits; any later credit-dependent plan must stop until a separate operator-approved billing-console receipt is retained. | `readbacks/billing-account-01FA88-CC4968-ADF817_.json` |

## POC and Long-term ownership

POC ownership remains separate from long-term foundation ownership.

- POC: `folders/726824330959` (`Proof of Concept`) contains existing POC/shared projects including `cs-poc-cha8mmii0xk0iaw5vpf8mxf`.
- Long-term: `folders/929563862525` (`gcp-internal-cloud-setup`) and `cs-host-377d41e71a824f92802120` are the accepted foundation denominator for #491+ bootstrap work.
- The current human account `daniel@agent-logic.ai` can read the denominator and currently has broad organization/folder/project roles in retained IAM readbacks. That is not the recommended long-term Terraform execution identity.

## Hard cost ceiling

Hard cost ceiling:

- #490: USD 0. This issue performs no GCP mutation and launches no workload.
- #491 Terraform/bootstrap preparation: no paid runtime workload is authorized by this decision. Any storage, state, logging, or API cost introduced by #491 must be separately bounded in #491 before apply.
- First paid runtime workload: USD 20 maximum, only for the later separately authorized GCP-E GPU readiness smoke test. This #490 decision does not launch it and does not treat existing quota as permission to run it.

Credit-sensitive spending is not authorized because no credit grant or credit expiry evidence is retained in the readbacks.

## Quota is not capacity

Quota is not capacity. The accepted host and POC compute readbacks show `CPUS_ALL_REGIONS=32`, `GPUS_ALL_REGIONS=0`, and no current usage for those aggregate quota rows, but this does not prove launch capacity or permission to run a workload. Capacity remains unaccepted until billing, service enablement, region availability, policy, cleanup, and issue-specific cost gates are proven in later issues.

## Current readback limitations

The refreshed user login is sufficient for #490 read-only hierarchy, billing, project, IAM, service, quota, and network denominator readbacks over the current candidate projects. Some optional policy and compute/network readbacks fail closed where APIs are disabled on projects. This is accepted as a no-mutation fact for #490, not as authorization to enable those APIs.

Observed examples:

- `orgpolicy.googleapis.com` is disabled for the active POC project, so org, folder, and several project policy readbacks return `SERVICE_DISABLED`.
- `compute.googleapis.com` is disabled for some non-target projects, so compute and network readbacks on those projects return `SERVICE_DISABLED`.
- The accepted long-term host project has compute quota and a default network readback, but #490 does not approve using either as workload capacity.

## Execution identity decision for #491+

The recommended #491+ execution identity is a company-controlled Terraform service account in the accepted long-term host project, used through impersonation or Workload Identity Federation. Human user ADC is acceptable for #490 read-only discovery; it is not the recommended apply identity.

Key decision:

- No service-account key is needed for #490.
- Do not create a key by default for #491+. Prefer service-account impersonation from the operator account for local work and Workload Identity Federation for CI/non-human execution.
- A JSON key should be a documented break-glass fallback only: time-bounded, stored outside the repo, mode 0600, never printed, never committed, and revoked after the bounded operation.

See `docs/milestones/v0.92.1/evidence/cloud/gcp-a/gcp-execution-identity-plan.md` for the phased role plan and first proof command.

## No mutation boundary

No mutation was performed or authorized for #490. The retained command manifest is limited to read-only `gcloud` list, describe, get, config, and auth readback shapes. Failed readbacks remain evidence gaps or no-mutation facts; they are not treated as success.

## Acceptance mapping

| Acceptance | Current result |
| --- | --- |
| AC-1 Organization, Folder, Project, Billing, and Region are exact or explicitly blocked. | Accepted: Organization `321515087273`, long-term Folder `929563862525`, host Project `cs-host-377d41e71a824f92802120`, Billing `01FA88-CC4968-ADF817`, Region `us-west2`. |
| AC-2 POC and Long-term ownership are explicit. | Accepted: POC folder/project are separate from the long-term setup folder/host project. |
| AC-3 The first workload has a hard cost ceiling. | Accepted: #490 USD 0; first later paid runtime workload is capped at USD 20 and requires separate authorization. #491 must separately bound any bootstrap apply costs. |
| AC-4 Quota is not treated as capacity. | Accepted by policy statement and quota evidence above. |
| AC-5 Evidence contains no credentials, secrets, raw token contents, or mutation commands. | Local validator passed over this register and retained evidence. |
| AC-6 Fresh exact-head review has zero actionable findings and publication truth closes only issue #490. | Not started. |

## Next required lifecycle steps

1. Run `.csdlc/prepared/issues/490/validate-gcp-a-decision.sh`.
2. Run typed C-SDLC validation and doctor.
3. Obtain exact-head review.
4. Publish only if review and C-SDLC gates pass.
