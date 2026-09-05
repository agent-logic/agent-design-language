# Runtime agent checkpoint archive

This Terraform root creates the private, KMS-encrypted S3 archive used by Runtime v3's five-minute per-agent partial checkpoints. It also attaches separate least-privilege writer and restore policies to existing IAM roles.

Apply is intentionally an operator-controlled deployment step. After apply, copy the `bucket_name` and `kms_key_arn` outputs into the Runtime init configuration:

```toml
[agent_partial_checkpoints]
enabled = true
interval_seconds = 300
snapshot_concurrency = 4
max_partial_bytes = 16777216
local_max_bytes = 2147483648
local_max_files = 8192
retained_partials_per_agent = 12
spool_max_bytes = 536870912
spool_max_files = 4096

[agent_partial_checkpoints.s3_archive]
region = "us-west-2"
bucket = "<bucket_name output>"
kms_key_arn = "<kms_key_arn output>"
restore_profile = "agent-checkpoint-restore"
```

The named AWS CLI profile must assume the separately managed restore role. The Runtime's normal instance role remains write-only and verifies each upload from the SHA-256 checksum returned by `PutObject`. After local startup, an asynchronous recovery pass fetches only the fixed latest pointer and referenced record for each resident agent; S3 delay or failure never gates Runtime availability.

`validate.sh` runs formatting, initialization, validation, a no-refresh plan, and deterministic policy assertions. It never applies resources.
