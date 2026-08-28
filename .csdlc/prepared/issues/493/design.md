# #493 GCP-D private platform foundation design

## Outcome

Build one private GCP platform foundation for disposable non-GPU workloads, using the #492 organization/billing baseline as a terminal dependency.

## Scope

- Create the `infra/gcp/platform/` Terraform root for a private platform foundation.
- Add a platform-foundation runbook under `docs/operations/cloud/gcp/platform-foundation/`.
- Add retained GCP-D proof under `docs/milestones/v0.92.1/evidence/cloud/gcp-d/`.
- Add an issue-owned validator at `.csdlc/prepared/issues/493/validate-gcp-d-platform-foundation.sh`.

## Required platform surfaces

- Private custom-mode VPC and regional subnet. The Terraform root must reject
  VM public IPs, `0.0.0.0/0` ingress, default SSH, and unmanaged firewall
  broadening. Only IAP TCP forwarding CIDR `35.235.240.0/20` may reach SSH.
- Egress posture is explicit: allow outbound package/bootstrap traffic only
  through reviewed NAT or explicitly documented direct egress; no inbound public
  data-plane shortcut may be introduced by this issue.
- IAP/OS Login operator access posture. Project/instance metadata must enable
  OS Login, and the root must not accept key-based SSH metadata.
- Separate human and workload identities. Human operator access is represented
  by IAM bindings/groups; disposable workloads use a dedicated service account
  with least-privilege storage/logging permissions and no checked-in keys.
- Separate storage owners/buckets for Terraform state, artifacts, models,
  continuity evidence, and logs. Each bucket must have a distinct logical owner
  label, uniform bucket-level access, versioning where retention matters, and
  no public access grants.
- Logging/metric/watchdog visibility for disposable workloads. The root/runbook
  must define required labels (`csm`, `env`, `issue`, `ttl`, `owner`), a deadline
  or TTL field, logging sinks or metrics for workload lifecycle events, and a
  watchdog/readback command that can find overdue resources.
- A disposable non-GPU workload path with deterministic cleanup selectors and
  zero-residue proof. Cleanup must identify instances, disks, addresses,
  service accounts/bindings created for the run, firewall rules, logs/evidence
  buckets, and any state objects by exact labels/prefixes without relying on
  chat context.

## Non-goals

- No GPU qualification.
- No production traffic.
- No Shared VPC expansion.
- No Unity or Observatory implementation.
- No static credential disclosure or checked-in service account keys.

## Validation plan

1. Static private-network proof rejects `access_config`, broad ingress, default
   SSH, missing IAP CIDR, missing OS Login, and missing egress/NAT posture.
2. Data-boundary proof verifies separate state/artifact/model/evidence/log
   buckets or owner declarations, uniform bucket-level access, no public grants,
   and distinct logical owner labels.
3. Telemetry/watchdog proof verifies logging metric/sink declarations, required
   labels, TTL/deadline fields, and a readback command that fails closed when
   selectors are absent.
4. Cleanup proof verifies disposable workload selectors and zero-residue command
   contract for compute, disk, address, firewall, IAM/service-account, storage,
   and state objects.
5. Exact-head review must pass before publication.

## Live proof boundary

Local static validation may prove the Terraform and runbook contract without cloud mutation. Any live GCP apply/destroy proof must use the approved company GCP project/account context, avoid credential disclosure, retain redacted evidence, and stop if cleanup cannot be proven.
