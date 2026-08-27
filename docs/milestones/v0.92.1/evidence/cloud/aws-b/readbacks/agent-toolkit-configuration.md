# Agent Toolkit for AWS configuration

- issue: #485
- profile: agent-logic-admin
- region: us-east-1
- generated_at_utc: 2026-08-27T19:29:14Z
- posture: read-only evidence collection

## Approved path

- Agent Toolkit for AWS is documented for the approved Codex path only.
- AWS CLI must be 2.35 or newer before this lane can be accepted as configured.
- Toolkit use does not authorize resource creation or IAM writes in this issue.

## Local CLI version

```text
aws-cli/2.36.32 Python/3.14.7 Darwin/25.5.0 source/arm64
```

## STS caller identity for approved profile

```text
{
    "UserId": "[AWS_IDENTIFIER_REDACTED]",
    "Account": "[AWS_ACCOUNT_ID_REDACTED]",
    "Arn": "[AWS_ARN_REDACTED][AWS_ACCOUNT_ID_REDACTED]:user/daniel.austin.admin"
}
```

