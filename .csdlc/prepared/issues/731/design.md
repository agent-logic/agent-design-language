# Issue 731 design

Status: pre-bind design packet for typed review.

## Purpose

Issue #731 corrects the incomplete live-delivery portion of #493 without
expanding the foundation design. The work must deploy only the reviewed private
GCP foundation in project `cs-host-377d41e71a824f92802120`, run exactly one
private `e2-micro` disposable Linux workload, destroy it, and retain an
independent zero-residue readback.

## Authority boundary

- No live GCP mutation occurs until a fresh operator authorization names the
  exact project, region, zone, saved-plan digest, network, subnet, run ID,
  instance name, cleanup deadline, impersonated identity, and rollback commands.
- The authorization window is at most 120 minutes, and the VM lifetime is at
  most 30 minutes.
- The proof-run spend cap is USD 1. The first-30-day foundation steady-state
  incremental spend cap is USD 5. Any higher estimate stops before apply.
- Credentials, token contents, static service-account keys, and sensitive raw
  cloud output must not be printed or committed.

## Static denominator

The foundation denominator is the existing Terraform under
`infra/gcp/platform/**`:

- one custom-mode VPC;
- one regional subnet in `us-west2` with CIDR `10.42.0.0/24`;
- three firewall rules: IAP SSH ingress, explicit private egress, and denied
  unapproved egress;
- OS Login project metadata;
- two operator IAM memberships;
- one workload service account;
- five labelled buckets and their declared IAM memberships;
- one project log-writer membership; and
- one logging metric.

The `variables.tf` generic defaults are not live #731 authority. The saved plan
must use explicit/private tfvars equivalent to `terraform.tfvars.example`, with
project `cs-host-377d41e71a824f92802120`, CSM `axioma`, network
`axioma-dev-csm-private`, and subnet `axioma-dev-csm-private-us-west2`. A plan
that resolves the generic `platform` / `csm-dev-private` defaults fails before
apply.

The disposable workload denominator is exactly one `e2-micro` Linux VM with one
auto-delete standard persistent boot disk no larger than 10 GiB, no external
IP, no static address, no GPU, no additional disk, no load balancer, no DNS, and
no production traffic. Workload labels must include `issue=493`,
`ttl=disposable`, `csm=axioma`, `env=dev`, a unique run ID, and an absolute
cleanup deadline.

## Execution sequence

1. Validate typed lifecycle state and this design packet.
2. Confirm dependency/provenance truth for #490, GCP-B1, #492/#580, and
   #493/#587.
3. Run read-only GCP preflight for required APIs, quota, IAM, billing, remote
   state, and impersonation readiness.
4. Produce a saved Terraform plan and validate its denominator before apply.
5. Present the fresh authorization packet and stop until the operator approves
   live mutation.
6. After authorization only, apply the bounded foundation and retain redacted
   readback proof.
7. After authorization only, create exactly one labelled private `e2-micro`
  workload, prove `RUNNING` with no external IP and one readiness
   observation, then destroy it and its boot disk.
8. Independently prove zero disposable residue and either exact foundation
   readback or rollback.
9. Record SOR/SRP truth, obtain exact-head review, publish, and wait for
   required CI.

## Stop conditions

Stop before mutation on missing/expired/incomplete authorization, failed
read-only preflight, generic Terraform defaults being used instead of exact
#731 tfvars, denominator drift, public network exposure, broad IAM, unexpected
replacement, over-budget estimate, credential exposure, cleanup deadline risk,
or any disposable residue after destroy.
