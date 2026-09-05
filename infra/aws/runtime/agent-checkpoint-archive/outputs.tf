output "bucket_name" { value = aws_s3_bucket.archive.bucket }
output "bucket_arn" { value = aws_s3_bucket.archive.arn }
output "kms_key_arn" { value = aws_kms_key.archive.arn }
output "archive_prefix" { value = local.archive_prefix }
