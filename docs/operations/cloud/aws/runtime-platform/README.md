# AWS Runtime platform runbook

Issue: #489 / AWS-F

This runbook uses the #489 AWS-F Terraform roots for the Runtime ALB origin and
private Runtime node. It is written so the ALB and instance can come up and go
down separately and quickly while the public edge remains owned by #122.

## Inputs

- AWS profile: `agent-logic-admin`
- Region: normally `us-west-2`
- Runtime port: normally `20997`
- Runtime ALB origin root: `infra/aws/runtime/alb-origin`
- Private Runtime node root: `infra/aws/runtime/private-node`
- Private node module: `infra/aws/runtime/modules/private-runtime-node`
- Public edge root owned by #122: `infra/aws/csm-public-edge`
- Resource adoption truth owned by #488:
  `docs/operations/cloud/aws/adoption/AWS_RESOURCE_ADOPTION_REGISTER.md`

Do not place credential values in Terraform variables, shell history, evidence,
or committed user-data files.

## Phase 1: ALB shell

From `infra/aws/runtime/alb-origin`, provide VPC/subnet/certificate inputs and
keep the Runtime origin closed unless explicit ingress is approved. Public DNS,
ACM issuance, CloudFront, WAF, WSS, and allowed-origin exposure are #122-owned;
AWS-F only consumes an existing regional ACM certificate by ARN or lookup.

```hcl
expected_aws_account_id      = "<agent-logic-aws-account-id>"
expected_terraform_workspace = "aws-f-runtime-alb-origin-dev"
target_instance_id    = null
allowed_ingress_cidrs = []
```

Initialize with an explicit remote-state backend key/workspace and then plan
before apply:

```bash
AWS_PROFILE=agent-logic-admin terraform init -backend-config=aws-f-runtime-alb-origin.backend.hcl
AWS_PROFILE=agent-logic-admin terraform plan -out aws-f-runtime-alb.tfplan
```

This creates or updates only the replaceable Runtime ALB origin. It does not
own CloudFront, WAF, API Gateway, or public Route53 authority.

## Issue #728 disposable proof runner

Issue #728 uses one governed shell entrypoint:

```bash
docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh
```

The runner is staged because the Terraform roots are intentionally separate:
the node needs the ALB security group output, and the final ALB attachment needs
the node instance id. Each mutation stage requires an explicit authorization
file under `.csdlc/evidence/728/` or
`docs/milestones/v0.92.1/evidence/cloud/aws-f/`.

Set these variables for every stage:

```bash
export AWS_PROFILE=agent-logic-admin
export AWS_REGION=us-west-2
export ISSUE_728_EXPECTED_ACCOUNT_ID=<agent-logic-aws-account-id>
export ISSUE_728_AUTHORIZATION_FILE=.csdlc/evidence/728/authorization.env
export ISSUE_728_ALB_ROOT=infra/aws/runtime/alb-origin
export ISSUE_728_NODE_ROOT=infra/aws/runtime/private-node
export ISSUE_728_ALB_WORKSPACE=aws-f-runtime-alb-origin-dev
export ISSUE_728_NODE_WORKSPACE=aws-f-runtime-private-node-dev
export ISSUE_728_ALB_BACKEND_CONFIG=.csdlc/evidence/728/alb-origin.backend.hcl
export ISSUE_728_NODE_BACKEND_CONFIG=.csdlc/evidence/728/private-node.backend.hcl
export ISSUE_728_ALB_VAR_FILE=.csdlc/evidence/728/alb-origin.tfvars
export ISSUE_728_NODE_VAR_FILE=.csdlc/evidence/728/private-node.tfvars
export ISSUE_728_ALB_PLAN=.csdlc/evidence/728/alb-origin.tfplan
export ISSUE_728_NODE_PLAN=.csdlc/evidence/728/private-node.tfplan
export ISSUE_728_ATTACH_PLAN=.csdlc/evidence/728/alb-attach.tfplan
export ISSUE_728_DEADLINE_UTC=2026-09-08T23:59:00Z
export ISSUE_728_COST_CEILING_USD=20
export ISSUE_728_EXTERNAL_HEALTH_URL=https://example.dev.csm.agent-logic.ai/v1/health
export ISSUE_728_EXPECTED_RECEIPT_MARKER=<instance-or-artifact-marker>
```

The authorization file is line-oriented `key=value` text. It must include at
least:

```text
issue=728
approved=true
production_traffic=false
aws_profile=agent-logic-admin
aws_region=us-west-2
expected_account_id=<agent-logic-aws-account-id>
alb_root=infra/aws/runtime/alb-origin
node_root=infra/aws/runtime/private-node
alb_workspace=aws-f-runtime-alb-origin-dev
node_workspace=aws-f-runtime-private-node-dev
alb_backend_config=.csdlc/evidence/728/alb-origin.backend.hcl
node_backend_config=.csdlc/evidence/728/private-node.backend.hcl
alb_var_file=.csdlc/evidence/728/alb-origin.tfvars
node_var_file=.csdlc/evidence/728/private-node.tfvars
alb_plan=.csdlc/evidence/728/alb-origin.tfplan
node_plan=.csdlc/evidence/728/private-node.tfplan
attach_plan=.csdlc/evidence/728/alb-attach.tfplan
external_health_url=https://example.dev.csm.agent-logic.ai/v1/health
expected_receipt_marker=<instance-or-artifact-marker>
receipt_file=.csdlc/evidence/728/disposable-proof-receipt.txt
cost_ceiling_usd=20
deadline_utc=2026-09-08T23:59:00Z
alb_plan_sha256=<printed-by-plan-alb>
node_plan_sha256=<printed-by-plan-node>
attach_plan_sha256=<printed-by-plan-attach>
```

Run the stages in order:

```bash
ISSUE_728_MODE=plan-alb docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh
ISSUE_728_MODE=apply-alb docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh
ISSUE_728_MODE=plan-node docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh
ISSUE_728_MODE=apply-node docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh
ISSUE_728_MODE=plan-attach docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh
ISSUE_728_MODE=apply-attach-prove-destroy docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh
```

The `plan-*` stages print SHA-256 digests for the saved plans. Add each digest
to the authorization file before running the matching `apply-*` stage. The
final stage waits for ALB target health, checks the external HTTP response for
the expected marker, destroys the private node and ALB roots in reverse order,
and fails if Terraform state or AWS readbacks still show the exact instance,
target registration, listener, target group, load balancer, or security groups.

## Phase 2: Private Runtime node

From `infra/aws/runtime/private-node`, set the ALB security group from the ALB
output and use a private subnet:

```hcl
expected_aws_account_id      = "<agent-logic-aws-account-id>"
expected_terraform_workspace = "aws-f-runtime-private-node-dev"
private_subnet_id      = "subnet-private-from-vpc"
alb_security_group_id  = "sg-from-alb-output"
```

Initialize with a distinct remote-state backend key/workspace and then plan
before apply:

```bash
AWS_PROFILE=agent-logic-admin terraform init -backend-config=aws-f-runtime-private-node.backend.hcl
AWS_PROFILE=agent-logic-admin terraform plan -out aws-f-runtime-spot.tfplan
```

The host has no public IP and no direct public ingress. Runtime traffic reaches
it through the ALB target group only.

## Phase 3: Attach target

Re-plan the ALB root with:

```hcl
target_instance_id = "i-from-spot-output"
```

Then apply the saved ALB plan. This attaches the exact private disposable
instance to the ALB target group.

## External path proof

For a disposable proof, call the configured Runtime health path through the
public route that is in scope for the test. The proof must show that the request
reaches the exact instance attached in Phase 3, for example by comparing a
bounded instance identity marker served by the Runtime or smoke responder.

The proof must record:

- exact Terraform root and module revisions;
- exact saved-plan filenames or digests;
- exact backend config file names or state keys, excluding credentials;
- exact Terraform workspace names and verified AWS account id;
- ALB target health result;
- request URL and HTTP status;
- instance identity marker or equivalent bounded receipt;
- `cloud_mutation=false` for static validation, or the exact mutation envelope
  for an explicitly authorized disposable live proof;
- `production_traffic=false`.

## Zero residue teardown

Destroy in reverse order:

1. Detach target by applying the ALB root with `target_instance_id = null`.
2. Destroy the private-node root.
3. Destroy the ALB root only if the operator intends to remove the replaceable
   origin.

Before declaring zero residue, read back:

- EC2 instance state is terminated or absent;
- target group has no registered target;
- security groups owned by the disposable stacks are gone or still attached only
  to retained ALB state;
- no committed evidence contains credential material.

## Non-goals

- Production traffic or cutover.
- Public edge redesign.
- Route53 or ACM resource creation.
- CloudFormation retirement.
- Cross-cloud abstraction.
- Direct public Runtime ingress.
