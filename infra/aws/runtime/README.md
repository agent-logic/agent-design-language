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
  CloudFront/origin smoke CIDRs only through explicit variable input.
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

Use separate state names or backends for the two root stacks:

- `aws-f-runtime-alb-origin.tfstate`
- `aws-f-runtime-private-node.tfstate`

Do not store public-edge state in this AWS-F namespace. The public edge remains
under #122.

## Proof posture

The tracked AWS-F proof in this issue is static and non-mutating:

- `cloud_mutation=false`
- `production_traffic=false`
- `credential_material_retained=false`

Live disposable deployment is a later operator-authorized action using the
runbook in `docs/operations/cloud/aws/runtime-platform/README.md`.
