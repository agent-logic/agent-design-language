# Agent IAM guardrails

- issue: #485
- profile: agent-logic-admin
- region: us-east-1
- generated_at_utc: 2026-08-26T20:44:12Z
- posture: read-only evidence collection

## Required guardrail posture

- Default agent access is read-only.
- Elevated actions require typed approval, scoped IAM context, and retained CloudTrail/CloudWatch attribution.
- This issue performs no IAM create/update/delete operation.

## Local AWS managed read-only policy metadata

```text
{
    "Policy": {
        "PolicyName": "ReadOnlyAccess",
        "PolicyId": "[AWS_IDENTIFIER_REDACTED]",
        "Arn": "[AWS_ARN_REDACTED]",
        "Path": "/",
        "DefaultVersionId": "v188",
        "AttachmentCount": 0,
        "PermissionsBoundaryUsageCount": 0,
        "IsAttachable": true,
        "Description": "Provides read-only access to AWS services and resources.",
        "CreateDate": "2015-02-06T18:39:48+00:00",
        "UpdateDate": "2026-07-21T18:42:30+00:00",
        "Tags": []
    }
}
```

## Customer managed policy list

```text
{
    "Policies": []
}
```

