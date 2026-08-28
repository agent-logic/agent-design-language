# AWS account foundation

Issue #485 intentionally does not apply Terraform or mutate AWS account-foundation resources.

This directory is reserved for reviewed account-foundation configuration owned by AWS-B and downstream AWS-C+ issues. For #485, the authoritative deliverable is the access and billing baseline plus redacted readback evidence under:

- `docs/operations/cloud/aws/access-billing/`
- `docs/milestones/v0.92.1/evidence/cloud/aws-b/`

Any future Terraform backend, state migration, deployment role, or account-foundation apply belongs to #486 or later typed issues, not this baseline.
