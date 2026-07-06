# S3 ObsMem Community-Memory Archive Policy

WP-08 uses an Agent Logic S3 bucket as the first community-memory archive
backend for ObsMem and Polis evidence artifacts. The live v0.91.7 bucket is:

- `adl-wp08-obsmem-community-archive-b05e1f4379b5c745-us-west-2`

The approved prefix is `community-memory/`. Objects under that prefix must be
treated as archive artifacts, not mutable runtime authority.

Required controls:

- AWS profile: `agent-logic-admin`
- Region: `us-west-2`
- Public access block: all four public access block settings enabled
- Versioning: enabled
- Object Lock: enabled, default governance retention of 365 days
- Encryption: SSE-S3 with bucket keys enabled
- Lifecycle: transition current versions to Glacier Instant Retrieval after 90
  days and Deep Archive after 365 days
- Noncurrent lifecycle: transition noncurrent versions to Glacier Instant
  Retrieval after 30 days and Deep Archive after 180 days
- Multipart cleanup: abort incomplete multipart uploads after 7 days

Durability wording must stay precise. This bucket uses S3 storage classes whose
vendor durability target is 11 nines per object. ADL must not claim a
mathematical 12-nines guarantee from this single-region bucket alone. #4913 owns
the follow-on durability proof and any stronger multi-copy or multi-region
durability argument.

Reapply or verify the policy with:

```bash
ADL_AWS_S3_OBSMEM_ARCHIVE_ACCOUNT_SHA256="<operator-approved-agent-logic-account-sha256>" \
  bash adl/tools/setup_wp08_s3_obsmem_archive_policy.sh \
    --out docs/milestones/v0.91.7/review/runtime/wp08_s3_obsmem_archive_4688 \
    --profile agent-logic-admin \
    --region us-west-2
```
