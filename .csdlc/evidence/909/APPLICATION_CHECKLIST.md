# Exact application preparation for #909

Status: **not executable yet**. This is a reviewable procedure, not an approval.
`PLATFORM_STATE_GAP.md` is the outstanding platform-plan gate. The operator
confirmed the intended `gcp-admins` group; directory API visibility remains
unproven and must be checked before any separately approved application.
No command in this document has been executed unless explicitly listed as a
completed read-only check in the evidence JSON.

## Responsibility and scope

- Company operator and approval contact: Daniel, `daniel@agent-logic.ai`, the
  verified company human identity for this issue. This identifies the requesting
  operator; it does not claim directory-group custody or future apply approval.
- Existing bootstrap execution identity:
  `tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com`.
  Bootstrap read-only plan exercised configured impersonation successfully.
- Intended long-term company admin group: `gcp-admins@agent-logic.ai`, required
  by reviewed #492 source and now explicitly confirmed by the operator ("gcp-admins is fine"). Directory lookup did not resolve it; the installed CLI maps both HTTP403 and404 to the same message, so absence is not proven. A verified
  existing replacement or group creation requires explicit owner direction;
  #909 does not create groups or silently substitute an individual binding.
- State/data cleanup custodian: must be identified with the retained #731 state.
  The company operator owns resolving this missing custody. IAM permission alone
  does not establish data-deletion authority.
- Billing approval and incident contact: company operator until a verified
  billing owner is explicitly delegated. No credit is included in cost basis.

The exact intended host/folder/org/billing/region is in `live-inventory.json`.
The POC project, its existing instance/disk, all non-selected resources, source
objects and all cloud credentials are outside application and cleanup scope.

## Completed plan decisions

| Package | Actual plan result | Required decision |
| --- | --- | --- |
| Bootstrap | Exit 0, two no-op addresses; private local copy of remote serial 4 | No apply is needed. Retain existing adopted backend and bucket IAM. Never recreate it from an empty state. |
| Organization | Exit 2, exactly five creates, no update/delete/replacement | Intended group is operator-confirmed; verify API visibility and obtain operator approval for the plan/control limitations before application. Refresh plan after any input or identity correction. |
| Platform | No plan; live resources exist and state is not located | Hold; do not create/import/reconstruct state under this issue. |

The organization plan's five addresses are:

1. `google_project_iam_member.corporate_owner_project_roles["roles/owner"]`
2. `google_project_iam_member.corporate_owner_project_roles["roles/viewer"]`
3. `google_project_iam_member.corporate_owner_project_roles["roles/iam.securityReviewer"]`
4. `google_billing_budget.host_project_guardrail`
5. `google_bigquery_dataset.billing_export`

The company group was not in host project IAM, the billing-account budget list
was empty, and the host dataset list was empty during this capture. Those
observations justified a fresh organization plan; they do not resolve the
unproven directory API visibility. Planned values are in `organization-plan-summary.json`.

## Ordered procedure once missing evidence is supplied

1. State custodian supplies the authoritative platform state/location and its
   provenance; directory owner resolves the exact target group. Run read-only
   verification against both. Stop on mismatch, inaccessible state, unknown
   ownership or any requirement for unauthorized import/backend mutation.
2. Refresh identity, inventory, source/lock hashes and all three plans in an
   isolated directory. Bootstrap stays no-op unless a separately reviewed drift
   requires action. Record private state lineage/serial/generation and exact
   raw-plan SHA-256. Compare remote state content/generation before and after
   planning; a changed state invalidates review. All local state and plans remain
   private, excluded from tracked evidence.
3. Review every planned address/action and cost consequence. Require zero POC,
   VM, GPU, NAT, load-balancer, DNS, public-listener, or data-copy changes. Any
   platform replacement/deletion is no-go until separately explained and approved.
4. Operator signs an issue-specific application approval containing: exact
   company identity, source revision, lock hash, each plan digest, resource
   action list, named executor/custodian/billing contact, expiry and time limit,
   spend envelope, expected readbacks, and exact rollback decision. Old #730/#731
   approvals and #909 execution authorization do not satisfy this gate.
5. The following command shape applies **only the newly approved saved plan**;
   it is deliberately not an auto-approve or fresh replanning path:

   ```sh
   # PLAN is an operator-selected private saved-plan path whose SHA-256 exactly
   # matches the fresh signed approval. ROOT is the exact bound issue checkout.
   test -n "$PLAN" && test -n "$ROOT"
   shasum -a 256 "$PLAN"
   terraform -chdir="$ROOT/.adl/runs/909/organization" apply -input=false "$PLAN"
   ```

   Supply process-scoped approved short-lived credentials, with shell tracing
   disabled. No credential value or raw state is logged in tracked evidence.
   Do not run this without the new exact application approval and directory verification.
6. Read back the three exact group memberships, created budget identity/filter/
   thresholds/notification recipients, and dataset ownership/location. Enable
   billing export only through an explicitly approved separate exact operation;
   creating a dataset does not turn export on. Stop dependent steps on mismatch.
7. Apply the independently approved platform saved plan only after its state is
   reconciled. The command has the same shape with the platform working directory
   and its own approved plan; no plan is available to authorize today. Read back
   each selected network, subnet, rule, metadata key, identity/grant, bucket and
   metric. Resource limits are the reviewed module's exact denominator, with zero
   compute launches and no POC changes.
8. Preserve post-apply state generation, audit identity/time and readback
   comparison; report unexpected residue or failures to the operator. Keep the
   issue incomplete until accepted final evidence is linked. Application itself
   is outside #909's current authorized work.

## Rollback decisions tied to actual plan

Bootstrap: current no-op requires no rollback. Do not delete an adopted state
bucket, its IAM or history as a cleanup shortcut.

Organization: before any future apply, preserve pre-change IAM, budget list and
dataset absence. If an approved five-create apply is partial, reconcile actual
state and cloud reads before preparing the rollback plan. Remove only those
three newly added group memberships (not a whole policy), only the newly created
budget identity, and the new dataset only if empty and data owner agrees.
A dataset receiving export tables/data crosses an irreversible data-retention
boundary: retain it until an explicit disposition, never force destroy it.
Never remove the last independent administrator. Produce and review a rollback
saved plan; apply that exact plan only with separate rollback authority. No
blind destroy, targeted delete-by-name or state push is allowed here.

Platform: exact rollback is blocked by missing state/plan. The dependency order
in `MOVE_IN_PACKET.md` constrains it but cannot authorize it. Once supplied,
classify every planned action as create/update/delete/no-op and record the exact
inverse or recovery method; existing retained resources must survive rollback.
Preserve bucket generations and workload dependencies before reviewing deletion.
Never claim that restoring Terraform state alone restores cloud resources.

## Cost, expiry and maintenance controls

- #909 current work authorizes zero infrastructure changes and no paid launch.
- The inherited proposed organization budget is a USD 20 monthly notification
  threshold scoped to this host **and `issue=492`**; it does not cover all host
  resources or enforce a hard spending cap. Thresholds are 50%, 90% and 100%.
- Current live budgets and billing export datasets are absent. Export activation,
  notification delivery and whole-estate cost attribution remain explicit
  residual controls, never presented as already deployed.
- The previous platform proof allowed USD 5 incremental foundation cost in its
  first 30 days. That historical approval is not current spend authority. Before
  application, the operator must accept a fresh bounded storage/logging/retention
  estimate and exact resource limits; unknown charges or reliance on unverified
  credits are no-go. A plan is not a bill or a pricing estimate.
- Retained backend data is long-lived recovery data. Platform storage is divided
  into state/artifacts/models/continuity-evidence/logs. Custodians must set
  retention and cleanup expectations for each; labels or an empty list alone do
  not authorize deletion. Account for soft-deleted and noncurrent generations.
- During application, use the approval's hard expiry/time limit; on timeout stop
  further changes and execute only the approved recovery path. Afterward the
  operator reviews actual spend and owned residue. No ongoing watcher, scheduled
  cleanup, budget mutation or cloud API enablement is created by this packet.
