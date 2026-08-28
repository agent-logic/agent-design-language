# AWS-F Runtime platform modules

Issue: #489

This directory is the AWS-F operator surface for the Runtime platform module
set. It intentionally adds issue-owned Terraform roots for the replaceable
Runtime ALB origin and private Runtime node while consuming, not taking over,
#122's public edge authority.

AWS-F owns the private Runtime platform contract:

- Runtime hosts have no direct public ingress by default.
- The public edge, Route53, ACM, CloudFront, WAF, WSS, and allowed-origin
  exposure remain owned by #122.
- Durable AWS resource ownership and cleanup authority are consumed from #488.
- ALB, instance/node, artifact/build, evidence, and cleanup state remain
  separable so the disposable host and the ALB can be created, destroyed, and
  recreated independently.

## Terraform roots

- `infra/aws/runtime/alb-origin` creates or reuses the replaceable Runtime ALB
  origin. It defaults `allowed_ingress_cidrs = []` and can be pointed at
  CloudFront/origin smoke CIDRs only through explicit variable input. It consumes
  an existing regional ACM certificate by ARN or lookup and does not create
  Route53 or ACM resources; those public-edge concerns remain #122-owned.
- `infra/aws/runtime/private-node` creates one private EC2 Spot Runtime node
  behind the ALB. It requires an ALB security group id and sets
  `associate_public_ip_address = false`.
- `infra/aws/runtime/modules/private-runtime-node` is the issue-owned node
  module used by the private-node root.

## Why this is not one Terraform root

The ALB and private-node roots are deliberately separate. A single root that
tries to pass `module.alb.alb_security_group_id` into the node module while also
passing `module.node.instance_id` back into the ALB attachment creates a
module-level dependency cycle. The supported AWS-F flow is therefore two-phase:

1. Plan/apply `infra/aws/runtime/alb-origin` with `target_instance_id = null`.
2. Plan/apply `infra/aws/runtime/private-node` with
   `alb_security_group_id` set to the ALB output.
3. Re-plan/apply `infra/aws/runtime/alb-origin` with `target_instance_id` set
   to the private-node output.
4. Run the disposable path proof, then destroy only the exact disposable stack
   surfaces selected by the runbook.

## Default security posture

The issue-owned reusable roots already fail closed:

- `infra/aws/runtime/alb-origin` defaults `allowed_ingress_cidrs = []`.
- `infra/aws/runtime/private-node` requires `alb_security_group_id`.
- `infra/aws/runtime/modules/private-runtime-node` sets
  `associate_public_ip_address = false`.
- The private Runtime node opens Runtime ingress only from the configured ALB
  security group.
- The instance uses IMDSv2, encrypted gp3 root storage, and one-time Spot
  termination for disposable proof runs.

Operators may explicitly add a narrow smoke CIDR, but a reusable AWS-F plan
must not leave `0.0.0.0/0` Runtime ingress in the committed examples.

## State separation

Both root stacks declare an S3 backend so normal use must provide backend
configuration with locking and account-owned state instead of silently falling
back to local state. Each root also checks the active AWS account and Terraform
workspace before planning/applying. Use separate backend config files,
workspaces, and state keys for the two root stacks:

- `aws-f-runtime-alb-origin.tfstate`
- `aws-f-runtime-private-node.tfstate`

The committed backend examples are:

- `infra/aws/runtime/alb-origin/aws-f-runtime-alb-origin.backend.hcl.example`
- `infra/aws/runtime/private-node/aws-f-runtime-private-node.backend.hcl.example`

Do not store public-edge state in this AWS-F namespace. The public edge remains
under #122.

## Proof posture

The tracked AWS-F proof in this issue is static and non-mutating:

- `cloud_mutation=false`
- `production_traffic=false`
- `credential_material_retained=false`

Live disposable deployment and zero-residue cleanup proof require explicit
operator authorization for AWS mutation and must use the runbook in
`docs/operations/cloud/aws/runtime-platform/README.md`.
