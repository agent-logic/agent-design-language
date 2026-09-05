#!/usr/bin/env bash
set -euo pipefail

module_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
terraform -chdir="$module_dir" fmt -check
terraform -chdir="$module_dir" init -backend=false -input=false
terraform -chdir="$module_dir" validate
terraform -chdir="$module_dir" plan -input=false -lock=false -refresh=false \
  -var=environment=dev \
  -var=runtime_role_name=agent-logic-runtime-dev \
  -var=restore_role_name=agent-logic-runtime-restore-dev \
  -out="$module_dir/agent-checkpoint-archive.tfplan"
terraform -chdir="$module_dir" show -json agent-checkpoint-archive.tfplan > "$module_dir/agent-checkpoint-archive.tfplan.json"
jq -e '
  [.resource_changes[].type] as $types |
  ($types | index("aws_s3_bucket")) != null and
  ($types | index("aws_s3_bucket_public_access_block")) != null and
  ($types | index("aws_s3_bucket_ownership_controls")) != null and
  ($types | index("aws_s3_bucket_versioning")) != null and
  ($types | index("aws_s3_bucket_server_side_encryption_configuration")) != null and
  ($types | index("aws_s3_bucket_lifecycle_configuration")) != null and
  ($types | index("aws_kms_key")) != null and
  ([.resource_changes[] | select(.type == "aws_kms_key") | .change.after.enable_key_rotation] == [true]) and
  ([.resource_changes[] | select(.type == "aws_s3_bucket_public_access_block") | .change.after |
    .block_public_acls and .block_public_policy and .ignore_public_acls and .restrict_public_buckets] == [true]) and
  ([.resource_changes[] | select(.type == "aws_s3_bucket_ownership_controls") |
    .change.after.rule[0].object_ownership] == ["BucketOwnerEnforced"]) and
  ([.resource_changes[] | select(.type == "aws_s3_bucket_versioning") |
    .change.after.versioning_configuration[0].status] == ["Enabled"]) and
  ([.resource_changes[] | select(.type == "aws_s3_bucket_lifecycle_configuration") |
    .change.after.rule[0] | .expiration[0].days == 30 and
    .noncurrent_version_expiration[0].noncurrent_days == 7 and
    .abort_incomplete_multipart_upload[0].days_after_initiation == 1] == [true])
' "$module_dir/agent-checkpoint-archive.tfplan.json" >/dev/null

rg -q 'DenyInsecureTransport' "$module_dir/main.tf"
rg -q 'DenyIncorrectEncryption' "$module_dir/main.tf"
rg -q 'DenyIncorrectKmsKey' "$module_dir/main.tf"
rg -q 's3:PutObject' "$module_dir/main.tf"
rg -q 'actions   = \["s3:PutObject", "s3:AbortMultipartUpload"\]' "$module_dir/main.tf"
rg -q 'actions   = \["s3:GetObject"\]' "$module_dir/main.tf"
