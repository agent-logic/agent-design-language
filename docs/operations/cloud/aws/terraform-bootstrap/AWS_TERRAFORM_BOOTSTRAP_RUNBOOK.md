# AWS Terraform bootstrap runbook

Issue: #486

This runbook brings up the small, persistent Terraform backend foundation for
the Agent Logic business AWS account.

## Preconditions

- Use the approved business AWS profile: `agent-logic-admin`.
- Do not use the personal/default AWS profile for ADL infrastructure.
- Do not import, copy, or dual-own existing website, DDNS, public-edge, Runtime,
  or workload Terraform state.
- Confirm the retained state-isolation register before plan/apply:
  `docs/milestones/v0.92.1/evidence/cloud/aws-c/state-isolation-register.md`.
- Review the saved plan before apply. If the plan changes after review, stop and
  re-review.

## Static proof

```bash
terraform -chdir=infra/aws/bootstrap fmt -check -recursive
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap init -backend=false
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap validate
```

## Plan

```bash
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap plan -out issue486-bootstrap.tfplan
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap show -no-color issue486-bootstrap.tfplan
```

Expected resource classes:

- `aws_s3_bucket` and bucket controls for Terraform state;
- `aws_dynamodb_table` for Terraform locks;
- `aws_iam_role`, `aws_iam_policy`, and role attachment for backend access.

## Apply

```bash
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap apply issue486-bootstrap.tfplan
```

## Migrate bootstrap state

The bootstrap root starts with `-backend=false` because it creates its own
backend. After the first reviewed apply succeeds, generate a private backend
config from `terraform output backend_hcl`, set the key to
`v0.92.1/aws-c/bootstrap/foundation.tfstate`, and run:

```bash
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap init -migrate-state -backend-config=<private-backend.hcl>
```

The committed redacted shape is
`infra/aws/bootstrap/backend.hcl.example`. Never commit the raw account id,
private backend config, `.terraform/`, `terraform.tfstate`, or saved plan.

## Readback

After apply, run:

```bash
AWS_PROFILE=agent-logic-admin bash docs/milestones/v0.92.1/evidence/cloud/aws-c/run-terraform-bootstrap-readbacks.sh --lane aws-readback
```

Record the output under `docs/milestones/v0.92.1/evidence/cloud/aws-c/` without
capturing credentials, token material, or local environment dumps.

## Backend handoff values

Future Terraform roots should use the `backend_hcl` output values:

```bash
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap output backend_hcl
```

Keep backend ownership singular: one state bucket, one lock table, no copied
state, and no state imports in this bootstrap issue.

## State-isolation register

The retained register for this issue is:

```text
docs/milestones/v0.92.1/evidence/cloud/aws-c/state-isolation-register.md
```

It is the publication-time boundary proof that #486 does not adopt website,
DDNS, public-edge, Runtime, or workload state. Update that register before any
future #486 plan review if the bootstrap root changes ownership boundaries.
