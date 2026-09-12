# #909 isolated platform-state recovery proposal

Status: **proposal only; not authorized or executed**. This would produce a
reconciled candidate for a real platform plan, subject to no unexpected drift;
authoritative state custody/adoption remains separately unresolved. It would
not change cloud resources or a remote backend. It is a state mutation in an isolated local candidate and requires
explicit operator scope expansion before execution.

## Source and exact missing identities

Use only the immutable saved-plan snapshot identified in `PLATFORM_STATE_GAP.md`:
serial 33, lineage `85034bc7-32bd-e807-1798-81537b9eec28`, 17 managed resources.
Preserve those exact bytes as a private read-only source, then make a separate
candidate in an ignored issue-local directory. Never replace the historical
archive, adopted bootstrap state or any remote state.

The final historical apply planned these three additional identities:

| Terraform address | Exact existing resource identity to verify and import locally |
|---|---|
| `google_compute_firewall.deny_unapproved_egress` | `projects/cs-host-377d41e71a824f92802120/global/firewalls/axioma-dev-csm-private-deny-unapproved-egress` |
| `google_project_iam_member.operator_iap_tunnel` | Project `cs-host-377d41e71a824f92802120`, role `roles/iap.tunnelResourceAccessor`, member `user:daniel@agent-logic.ai` |
| `google_project_iam_member.operator_os_login` | Project `cs-host-377d41e71a824f92802120`, role `roles/compute.osLogin`, member `user:daniel@agent-logic.ai` |

These platform user memberships are the historical applied identities; the
operator-confirmed corporate `gcp-admins@agent-logic.ai` belongs to the separate
organization proposal. Do not silently substitute that group during recovery.

## Proposed authorized boundary and verification

1. Verify saved-plan hash, embedded state lineage/serial and 17 exact addresses
   against immutable history. Verify current explicit company account/project.
   Read all 20 exact live resource identities and require matching ownership;
   stop on missing/ambiguous resources, duplicates, drift needing cloud edits,
   changed provider schema or any additional import requirement.
2. Copy unchanged reviewed platform source and provider lock into a new private
   isolated local directory with backend disabled. Preserve the original partial
   snapshot and record hashes. Confirm there is no active remote backend before
   any candidate state write; use no shared/adopted Terraform working directory.
3. After approval only, extract the partial state to the candidate and perform
   precisely three `terraform import` operations using the exact identities
   above. The installed locked provider's import grammar must be checked before
   execution. Each import reads the provider and writes only candidate local
   state. It must not create/delete/update a cloud resource. Stop on any attempt
   to access or lock a remote backend. No bulk import, state push, migration,
   refresh-only apply, ordinary apply or resource recreation is included.
4. Preserve pre/post candidate hashes and serials; inspect 20 exact unique
   managed addresses and all live identity fields. Run a saved read-only plan
   from that candidate using the current reviewed source and historical applied
   inputs. Independently review full actions, values, drift and source/lock hashes.
   Non-no-op results require explicit disposition; never auto-apply them.
5. Retain the reconstructed state privately with provenance labelled
   **reconstructed candidate**, not recovered authoritative post-apply state.
   Produce a sanitized plan and recovery receipt. Adoption, durable custody,
   backend upload/migration, later apply and spending remain separate decisions.

Recovery rollback is deletion of only the disposable candidate after retaining
sanitized diagnostic evidence; the immutable source snapshot remains preserved.
No cloud rollback is expected because cloud mutations are excluded. Any observed
cloud mutation invalidates this boundary and must stop/report immediately.

## Scope interpretation

The source issue stops on `mutation_without_operator_authority` and requires
separate authority for cloud/account changes and infrastructure application.
It does not explicitly exempt isolated local imports. The bound STP/SIP set the
execution profile to "Read-only GCP and Terraform plan; local
consistency/redaction checks; no apply or state changes". The parent assignment
also expressly excludes reconstruction/import. Thus read-only history research
is authorized, but this local candidate import is **not**. Explicit approval
must name this bounded three-import local recovery before the SPP/SIP/STP are
updated through native editors and any recovery runs. The proposal does not
request or supply cloud-apply or remote-state authority.

## Independent proposal review

The independent #908 reviewer verified the immutable plan hash and embedded
serial/lineage/17-resource set in memory; all three missing addresses match
current live readbacks already retained. The reviewer found the isolated
three-import boundary feasible under explicit approval and requested that the
proposal avoid claiming state custody/adoption resolved. That wording is fixed.
No recovery or cloud command ran during review.
