run "runtime_log_archive_plan_contract" {
  command = plan

  assert {
    condition     = aws_s3_bucket_public_access_block.archive.block_public_acls && aws_s3_bucket_public_access_block.archive.ignore_public_acls && aws_s3_bucket_public_access_block.archive.block_public_policy && aws_s3_bucket_public_access_block.archive.restrict_public_buckets
    error_message = "The archive bucket must block all public access."
  }

  assert {
    condition     = aws_s3_bucket_ownership_controls.archive.rule[0].object_ownership == "BucketOwnerEnforced"
    error_message = "The archive bucket must use bucket-owner-enforced ownership."
  }

  assert {
    condition     = length([for rule in aws_s3_bucket_server_side_encryption_configuration.archive.rule : rule if length([for encryption in rule.apply_server_side_encryption_by_default : encryption if encryption.sse_algorithm == "AES256"]) == 1]) == 1
    error_message = "The archive bucket must default to SSE-S3."
  }

  assert {
    condition     = aws_s3_bucket_versioning.archive.versioning_configuration[0].status == "Enabled"
    error_message = "The archive bucket must enable versioning."
  }

  assert {
    condition     = length([for rule in aws_s3_bucket_lifecycle_configuration.archive.rule : rule if length([for expiration in rule.expiration : expiration if expiration.days == 30]) == 1 && length([for noncurrent in rule.noncurrent_version_expiration : noncurrent if noncurrent.noncurrent_days == 7]) == 1 && length([for incomplete in rule.abort_incomplete_multipart_upload : incomplete if incomplete.days_after_initiation == 1]) == 1]) == 1
    error_message = "The archive bucket lifecycle must retain current versions 30 days, noncurrent versions 7 days, and abort incomplete multipart uploads after 1 day."
  }

  assert {
    condition     = data.aws_iam_policy_document.publisher.statement[0].actions == toset(["s3:GetBucketLocation"]) && data.aws_iam_policy_document.publisher.statement[1].actions == toset(["s3:AbortMultipartUpload", "s3:ListMultipartUploadParts", "s3:PutObject"])
    error_message = "The publisher policy must be limited to bucket location and exact-prefix object/multipart writes."
  }

  assert {
    condition     = local.archive_prefix == "logs/env=dev/polis=konishi/runtime=wuji"
    error_message = "The archive prefix must be identity-partitioned by environment, Polis, and Runtime."
  }
}
