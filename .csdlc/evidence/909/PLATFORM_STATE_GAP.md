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

## Historical custody investigation after operator follow-up

The operator confirmed the intended `gcp-admins` group and requested historical
issue investigation rather than another request to locate files. #731/PR #739,
its remediation PR #742, foundation #493/PR #587, hierarchy #490 and separate
bootstrap #730/#740 were inspected. #731 and PRs #739/#742 had no comments
providing an alternate state location. PR #739 explicitly kept local Terraform
state out of its PR. The applied foundation remained deployed after disposal
of the temporary workload.

The historical command used `terraform -chdir=infra/gcp/platform init
-backend=false`; the original local state path was the removed #731 worktree's
`infra/gcp/platform/terraform.tfstate`. Parent investigation of the executing
session independently found post-apply reads of that same local path. No
migration or backup command was found in that bounded execution history.

A partial recovery source **does exist** in Git: at commit
`78490bd7bc8f0815914469fb4c0323d23437dd13`,
`.csdlc/evidence/731/terraform-plan-denominator/foundation.tfplan` has SHA-256
`e20e83dcafc5d8f7cd963660bbcc249be6fea81ca76d841e8e2e298a4426f6df`,
matching the final applied-plan evidence. Its embedded pre-apply `tfstate` has
serial **33**, lineage **85034bc7-32bd-e807-1798-81537b9eec28** and **17 managed
resources**. The final plan adds exactly the three addresses listed in
`RECOVERY_PROPOSAL.md`. It is not the final post-apply 20-resource state.
The earlier plan at `2e6f4d288b99da969e067154414b5ccaab4c2648` contains empty
state. All available tracked #731/#742/platform history contains only these
two binary-plan versions and no later state/backup or alternative saved plan.
Archives were inspected in memory; no embedded state was restored or imported.

This narrows the gap to the final local state or a separately authorized,
reviewed reconstruction. It does not prove why that final state is unavailable.
