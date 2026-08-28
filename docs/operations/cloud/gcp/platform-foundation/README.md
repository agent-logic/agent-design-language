# GCP-D private platform foundation

This runbook brings up the private GCP foundation for disposable CSM workloads.
It is intentionally boring: one custom-mode VPC, one private subnet, IAP and OS
Login for operator access, one workload service account, separate storage-owner
buckets, and labels that make cleanup/readback deterministic.

It does not run GPU qualification, Observatory, Unity, AWS, production traffic,
Shared VPC expansion, or static service-account-key creation.

## Prerequisites

- Terraform `>= 1.6`.
- Google provider `~> 6.0`.
- Authenticated `gcloud auth application-default login` or an approved
  workload identity flow for the company project.
- A remote GCS backend bucket. Copy
  `infra/gcp/platform/backend.tf.example` to `backend.tf` and set the bucket.

Do not commit `backend.tf`, state files, tfplans, credential files, or provider
tokens.

## Configure

Copy the example variables and edit the project/environment names:

```sh
cd infra/gcp/platform
cp terraform.tfvars.example terraform.tfvars
```

The important knobs are:

- `project_id`
- `region`
- `environment`
- `csm_name`
- `network_name`
- `subnet_name`
- `subnet_cidr`
- `operator_group_email`

## Plan

```sh
terraform init
terraform fmt -check
terraform validate
terraform plan -out gcp-d-platform.tfplan
```

The plan must show:

- private custom-mode VPC and subnet;
- no public external instance address;
- no broad public ingress;
- IAP source range `35.235.240.0/20`;
- OS Login enabled;
- separate human operator group and workload service account;
- separate buckets for state, artifacts, models, continuity evidence, and logs;
- required `csm`, `env`, `issue`, `owner`, and `ttl` labels.

## Apply

Apply only after reviewing the saved plan:

```sh
terraform apply gcp-d-platform.tfplan
```

## Zero residue cleanup

Disposable workload resources must carry labels that let an operator or watchdog
read back every object before destroy:

- `issue=493`
- `ttl=disposable`
- `csm=<name>`
- `env=<environment>`
- `deadline=<timestamp>`

The cleanup proof must list and then destroy matching instances, disks,
addresses, firewall overrides, service-account grants, storage objects including
noncurrent object versions, and state references. A successful proof ends with
zero residue for every selector.

Read the current residue set before and after destroy:

```sh
./docs/operations/cloud/gcp/platform-foundation/readback-disposable-residue.sh \
  --project "$PROJECT_ID" \
  --zone "$ZONE" \
  --csm "$CSM_NAME" \
  --env "$ENVIRONMENT" \
  --network-name "$NETWORK_NAME"
```

Destroy the foundation only when no disposable workloads remain:

```sh
terraform destroy
```

## Evidence boundary

For this issue, local static proof is allowed. Live GCP apply/destroy proof
requires explicit operator authorization and must not print, copy, retain, or
commit credentials.
