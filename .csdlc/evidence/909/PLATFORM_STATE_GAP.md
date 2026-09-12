# #909 platform state location gap

Status: required acceptance blocker; **cause unproven**. Company authentication
is working. Do not confuse this gap with the superseded stale login cache.

Current live readbacks find the #731 private VPC, subnet, workload identity and
five storage-owner buckets in the accepted company host project. The bootstrap
backend contains `bootstrap/default.tfstate` and historical recovery canaries.
The platform `state` bucket contains no objects in the current recursive listing.
Neither observation identifies authoritative state for `infra/gcp/platform`.

A bounded search inspected:

- Current primary and issue worktree Terraform roots: no adopted platform state.
- Registered worktree inventory: old #731 worktree is absent.
- Git-common retained-v2 GCP bootstrap/recovery and cleanup/archive metadata:
  bootstrap recovery state exists; no #731 platform state located.
- Native `closeout-sweep-20260909` preview/remove requests for #731: exact old
  registered worktree was selected for cleanup after PR #742 closed #731.
- Native preserved #731 root artifacts and closeout archive directory inventory:
  no platform `.tfstate` found. Other issue archives were not treated as #731
  custody or searched for credentials.
- #731 source issue, PR #742 and retained output/design/scripts: foundation stays
  after successful readback; no current authoritative state location is stated.

These are observations of the inspected locations, not proof that state was
lost, that native cleanup deleted it, or that no backup exists. The old apply
script used local package state unless the caller supplied a backend; an ignored
state file is therefore a plausible recovery lead. Never convert that hypothesis
into an accusation or automatic reconstruction.

## Required next decision

1. Preferred: company/state custodian provides the retained #731 platform state
   or exact private backup/backend location. Read metadata, establish provenance
   (lineage, serial, source revision, resource addresses), compare current cloud
   identities, then run a bounded read-only plan from an isolated private copy.
   Preserve the original and compare live backend generation before/after.
2. If state exists elsewhere: obtain explicit owner/location confirmation and
   read-only access. Do not initialize, migrate or change the backend under #909.
3. If the custodian confirms state unavailable: record confirmed loss and create
   a separately authorized recovery task. Recovery must inventory every exact
   live resource, establish ownership and import mapping, independently review
   the recovery plan, preserve backups, and obtain explicit state/import
   authority before any mutation. #909 remains incomplete until accepted output
   provides a real reconciled platform plan.

No import, refresh-only state write, backend creation, state push, resource
recreation, apply, deletion or forced cleanup is authorized by this packet.
A fresh empty-state creation plan for existing platform resources is rejected.
The completed bootstrap no-op and organization five-create plans remain valid
bounded observations, not substitutes for this missing platform gate.
