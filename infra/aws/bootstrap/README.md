# Agent Logic AWS Terraform bootstrap

This root owns the account-foundation Terraform backend for the Agent Logic
business AWS account. It deliberately does not import website, DDNS,
public-edge, Runtime, or workload state.

Resources:

- encrypted/versioned S3 Terraform state bucket;
- encrypted DynamoDB lock table with point-in-time recovery;
- scoped Terraform deployment role and backend-access policy.

Use the operator-approved business profile only:

```bash
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap init -backend=false
AWS_PROFILE=agent-logic-admin terraform -chdir=infra/aws/bootstrap plan -out issue486-bootstrap.tfplan
```

Apply only after reviewing the saved plan and confirming it differs from no
reviewed evidence.
