# Company GCP move-in packet — #909 / Sprint 8 #934

Status: **complete planning candidate, pending final independent review**.
Actual plans cover all three selected packages: bootstrap two no-ops, platform
20 no-ops, organization five creates. The platform plan uses an explicitly
approved reconstructed private candidate; custody/adoption remains a separately
approved prerequisite before future application. No cloud resource change,
remote state write or apply was performed. Local state changed only through the
three approved imports recorded in `local-recovery-receipt.json`.

## Source and prior-work disposition

Inspected repository revision: `f1c4e2a915c215797f0d2708cb8b0568f2b80b32`.
`source-inventory.json` records every selected Terraform source and provider-lock
digest. Reuse the following packages, without rebuilding predecessor work:

| Source | Existing responsibility | Disposition in #909 |
| --- | --- | --- |
| `infra/gcp/organization/` (#492) | Host project corporate IAM, filtered budget, billing-export dataset | Reuse; reconcile live IAM, budget and dataset before any plan acceptance |
| `infra/gcp/bootstrap/` (#491, #730, #740) | Private versioned Terraform state bucket and bootstrap SA bucket IAM | Reuse short-lived identity/recovery design; do not recreate or adopt any bucket without live state reconciliation |
| `infra/gcp/platform/` (#493, #731) | Private network, operator/workload identity, storage classes and logging metric | Reuse; reconcile actual surviving foundation and state; historical disposable proof is not current presence/absence |
| `docs/operations/cloud/gcp/decisions/GCP_HIERARCHY_COST_DECISION.md` (#490) | Accepted company hierarchy and no-credit cost basis | Governs selection; supersedes speculative names in the older local move-in rationale |
| `docs/operations/cloud/gcp/terraform-bootstrap/README.md` | Impersonation, saved-plan and backend recovery contract | Reuse design, never reuse expired #730 apply authorization |
| `docs/operations/cloud/gcp/platform-foundation/README.md` | Private foundation and disposal procedure | Reuse only after exact current resource/state selection |

The original local GCP move-in rationale is planning history, not current
inventory. Its POC selection, speculative new project hierarchy and possible
credit balance do not override #490. Required principles adopted here are
separate data/state owners, Terraform authority, short-lived credentials,
reviewed plans, audit retention, budgets plus actual workload limits, and
cleanup with independent readback. GPU and Runtime phases remain excluded.

## Selected identities and current observation

The accepted company hierarchy is now verified by live read-only observations in `live-inventory.json`; resource ownership beyond those observations remains explicit:

| Boundary | Exact selected target | Current proof |
| --- | --- | --- |
| Company organization | `organizations/321515087273`, `agent-logic.ai` | Verified live describe |
| Source POC folder | `folders/726824330959` | Verified live |
| Source POC project | `cs-poc-cha8mmii0xk0iaw5vpf8mxf` | Verified live; preserve all resources/data |
| Destination foundation folder | `folders/929563862525` | Verified live |
| Destination host project | `cs-host-377d41e71a824f92802120` | Verified live describe |
| Billing | `billingAccounts/01FA88-CC4968-ADF817` | Verified live enabled company billing |
| Region | `us-west2`, US residency | Region readback passed; policy readback tracked separately |
| Human read-only identity | `daniel@agent-logic.ai` in normal authenticated company gcloud configuration | Verified normal authenticated company context |
| Bootstrap identity | `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com` | Bootstrap provider impersonation succeeded in read-only state-backed plan |
| Corporate owner | `group:gcp-admins@agent-logic.ai` | Corporate group not observed in current project IAM; proposed reconciliation required |

`identity-attempt.json` preserves the superseded stale Git-common configuration failure. The operator confirmed authentication; normal gcloud metadata selected the exact company account and host project, and explicit account/target live reads succeeded. `live-inventory.json` supersedes that blocker. No personal identity, static key or configuration mutation was used.

This is foundation move-in, not a POC project transfer or data migration. POC
resources, model data, logs and existing workloads remain unchanged. Any future
copy, project reparenting, billing reassignment, DNS cutover or deletion requires
its own exact source/destination inventory and explicit approval.

The completed first census records timestamped project/folder/org parentage, billing
link/open currency status, IAM owners and bootstrap impersonation; project APIs,
organization policies, VPC/subnets/firewalls, quotas, service accounts, buckets
and BigQuery datasets; and budgets. Supplementary policy/export and state reconciliation still apply. Capture resource names,
state ownership, data class, dependent workloads and retention/deletion controls.
Read only metadata needed for this reconciliation; do not download stored user
content. Failed/denied queries are gaps, never an empty inventory.

## Terraform resource boundary

Source declarations are enumerated in `source-inventory.json`. They are not a
Terraform plan and do not assert creation, update, deletion or no-op actions.

| Package | Configured resource boundary |
| --- | --- |
| Organization | Three project IAM memberships for the corporate group (`roles/owner`, `roles/viewer`, `roles/iam.securityReviewer`); one host-project budget; one BigQuery dataset `adl_gcp_c_billing_export` |
| Bootstrap | Bucket `adl-tf-state-cs-host-377d41e71a824f92802120`; one `roles/storage.admin` grant to bootstrap SA; versioning, public-access prevention, uniform access, seven-day soft deletion, no force destroy |
| Platform, example selection | One VPC `axioma-dev-csm-private`; subnet `axioma-dev-csm-private-us-west2`, `10.42.0.0/24`; three firewall rules; project OS Login metadata; three project IAM memberships; SA `axioma-dev-workload`; five data-owner buckets; four bucket IAM memberships; one missing-deadline logging metric |

The live census confirms the selected network and five platform buckets. It does not yet reconcile them against an adopted Terraform state. The five bucket suffixes are `state`, `artifacts`,
`models`, `continuity-evidence`, `logs`, each under the host project plus
`-dev-axioma-`. The platform `state` bucket is a separate resource from the
bootstrap Terraform backend bucket. Never delete either based on a similar name.
The platform contains no VM, GPU, NAT, load balancer or public listener.

## Validation and actual plans

Terraform 1.15.3 on darwin_arm64 ran `fmt -check`, `init -backend=false
-input=false`, and `validate -no-color` for all three packages successfully.
Unmodified `.tf` files plus each tracked provider lock were copied into ignored
issue-local directories. `static-checks.json` records exact commands and exits;
local logs are `.adl/runs/909/{organization,bootstrap,platform}-*.log`.
These checks prove formatting/schema consistency only. During these static checks, no backend configuration or existing state was copied, initialized, migrated, locked or changed. Subsequent bootstrap planning separately downloaded a private read-only copy of the existing state.

**All three packages have real plans.** Bootstrap uses a private read-only copy of remote serial 4 and has two no-ops. Organization has exactly five creates and no update/delete/replacement, justified by live absence of its managed memberships/budget/dataset. Platform uses the approved reconstructed local serial 36 and has 20 no-ops; all 20 identities were verified before and after recovery and all readback values were unchanged. Eight historical-state refresh entries are empty-collection normalization, IAM etag and bucket update timestamps; none proposes a configuration change. Exact actions/values and provenance are retained in `platform-plan-summary.json` and `local-recovery-receipt.json`. No remote backend was initialized, locked or changed. The original post-apply platform state remains unavailable; the candidate is explicitly reconstructed, not asserted to be that original state.

To finish the remaining platform plan gate:

1. Name the current state custodian and exact existing backend/state for each
   package. Read existing metadata and address inventory without writing or
   migrating state. If no authoritative state can be located, stop and obtain
   ownership resolution; do not import or create a backend under #909.
2. Pin the source revision, provider lock, target variables and selected existing
   state. Use short-lived approved company credentials and an isolated local
   working directory; preserve secret/raw state and plan material outside tracked
   evidence. Do not initialize an ambiguous backend or invoke recovery scripts
   that include apply/destroy as part of their proof.
3. Prepare the read-only plan against the verified state, with no backend/state
   mutation. Review whether the chosen backend's normal plan lock writes require
   a separately approved mechanism; `-lock=false` is only appropriate with a
   custodian-confirmed quiescent state and serial/generation comparison before
   and after. Do not weaken concurrency protection silently.
4. Record every actual address/action, before/after identity, cost/control
   effect and dependency. Retain raw plan privately, with digest and redacted
   review evidence. No-op, proposed change, incomplete and failed are distinct.
5. Reject unexpected source-project changes, replacement/deletion, broad IAM,
   public access, data movement, missing owners or state drift. Reconcile and
   independently review the same inventory, plan, application and rollback set.

## Application and rollback gates

The order below is grounded in the three packages but **cannot be the final
exact application procedure until the real plan and live ownership are known**.
Responsible roles require named acceptance before application: company operator
(identity and authorization), Terraform/state custodian (plan and recovery),
billing owner (notification/charge controls), and data owners (retention/cleanup).
No role acceptance is inferred from the existence of an IAM binding.

1. Operator verifies company hierarchy, host project and billing identity again;
   custodian verifies clean exact source, plan digest/expiry, state serial and
   unchanged variables; billing owner accepts the exact spend envelope. A changed
   plan, missing state, expired credentials or unanswered inventory gap is no-go.
2. Reuse the accepted bootstrap backend. If a bootstrap change is proposed,
   establish recoverable state generation and independent custodian access before
   approving it. Never delete or replace an adopted backend automatically.
3. Apply only separately approved organization/billing plan actions. Verify each
   corporate role, exact budget filter/notification destination and dataset
   ownership. Verify export activation separately; dataset creation alone does
   not activate billing export.
4. Apply only separately approved platform plan actions. Verify VPC/subnet
   identities, private API access, three firewall rules, OS Login and operator
   grants, dedicated workload SA, five bucket owners and logging metric.
   No workload launch is included or needed to claim this planning deliverable.
5. Independent reviewer checks readbacks against the approved plan and records
   residual ownership/cost. Preserve state, audit and data-retention evidence.

Rollback follows affected dependencies in reverse and requires its own reviewed
plan, never a blind `terraform destroy`: stop dependent work; inventory and
preserve retained bucket/object generations; remove only newly introduced
workload bucket grants and project log-writer grant; remove a new workload SA
only when no consumers remain; remove newly introduced metric and eligible
empty buckets; then remove newly introduced operator grants/metadata only after
comparison with pre-change values; remove owned firewall rules, subnet and VPC
only after zero dependent interfaces/resources are proven. Organization IAM
changes restore exact pre-change membership, never revoke the last independent
company administrator. Billing datasets and adopted state remain retained until
explicit data-owner disposition. Review actual Terraform dependency ordering
against the saved rollback plan; this narrative is not a substitute.

Irreversible boundaries include object-version deletion, dataset/table deletion,
state-history loss, log expiry and loss of the last administrative identity.
None is authorized here. Restore a previous state object only under the backend
recovery procedure with generation checks and a reviewed reconciliation plan;
state rollback alone does not undo real cloud actions.

## Billing, audit, limits and cleanup

The inherited organization budget is USD 20 with 50%, 90% and 100% thresholds,
**filtered to host-project resources labeled `issue=492`**. It is not a
whole-project or #909 spend cap. The live billing-account budget list and host-project dataset list are empty. The organization plan proposes the budget and landing dataset; it does not activate detailed billing export or prove notification delivery. Current project log sinks are only `_Required` and `_Default`; central audit export is not established by this packet.
Do not rely on historical credits, quota or budget alerts as a hard stop.

#909 authorizes no apply, workload or paid deployment. Before any future apply,
record storage/logging/retention costs and a separate approved cost envelope;
resource-count limits are the exact accepted plan, with zero VM/GPU additions.
The old #730 USD 5/90-minute authorization and later workload caps are historical,
not reusable authority. Billing owner must resolve coverage beyond `issue=492`
and verify notification delivery/recipients before application; Daniel is the named billing approval contact. These are explicit future application gates, not claims of deployed controls.

Preserve source/plan digests, resource readbacks, identity checks, approvals,
state generation comparisons and redacted audit evidence. Terraform raw state,
binary plans, backend configuration, tokens and credentials remain private and
untracked. Machine-readable evidence is separate from human diagnostics.

Cleanup owner is the named state/data custodian for each exact selected
resource. Read noncurrent object versions, retention and soft-delete controls
before any removal. Verify independent post-cleanup metadata and state-address
readback; failed queries cannot prove zero residue. The POC project and all
non-selected workloads are outside cleanup scope. Do not invoke the historical
#731 apply/destroy proof scripts during #909.

## Acceptance and residual routing

| Obligation | Evidence | Status / owner |
| --- | --- | --- |
| Current company identity and complete inventory | `live-inventory.json`, `local-recovery-receipt.json` | Verified company hierarchy/billing and all20 selected platform identities; Daniel owns future custody decisions |
| Prior reviewed work reused | `source-inventory.json`, source matrix above | Unchanged source/locks reused; existing bootstrap/platform retained |
| Terraform fmt/validate | `static-checks.json` | All nine commands passed |
| Real plan and exact application/rollback consistency | Three package plan summaries and `APPLICATION_CHECKLIST.md` | Bootstrap2noops, platform20noops, organization5creates; exact rollback and custody prerequisite explicit |
| Billing and cleanup controls | Source gaps recorded above | Daniel owns approval/cleanup; exact filter limitations, no-launch limits and future charge/readback gates explicit |
| Independent review and source/link/redaction checks | Local checks pass; first review found null policy projection | Repaired with constraint identities and20 effective readbacks; final delta review pending |

Umbrella #934 owns sprint routing; #864 remains the accepted planning dependency.
Preserve all seven milestone planning tasks. This gap report does not defer or
waive any #909 obligation. Other cloud billing, Runtime deployment, GPU launch,
six-resident qualification and Observatory work remain separate owners/issues.

## Platform recovery provenance and future custody

See `PLATFORM_STATE_GAP.md` for inspected locations, the native historical cleanup lead, and non-mutating recovery options. Cleanup loss is only a hypothesis. The current live POC has one instance and one disk; the host has zero instances/disks but its foundation remains. Preserve both projects and all data. `foundation-controls.json` records supplementary bucket/IAM/OS Login/logging/policy/dataset metadata.

`APPLICATION_CHECKLIST.md` provides exact completed-plan decisions, responsible approval contacts, ordered saved-plan commands and rollback/cost boundaries. The operator confirmed the intended `gcp-admins` identity; `corporate-group-check.json` still records unproven directory API visibility for verification before a future application. No group is created or replaced here. `RECOVERY_PROPOSAL.md` preserves the reviewed proposal; its exact three local imports were later approved and executed as `local-recovery-receipt.json` records. Adoption or upload is not included.

Policy projection now retains actual constraint names; `effective-policies.json` contains all20 effective host-policy readbacks for the observed organization constraints. Region quota values are retained in `live-inventory.json`. Both are current metadata evidence, not workload launch capacity or authorization.
