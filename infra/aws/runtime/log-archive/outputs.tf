output "archive_bucket_name" {
  value = aws_s3_bucket.archive.bucket
}

output "archive_bucket_arn" {
  value = aws_s3_bucket.archive.arn
}

output "archive_prefix" {
  value = local.archive_prefix
}

output "publisher_policy_arn" {
  value = aws_iam_policy.publisher.arn
}

output "publisher_prefix_arn" {
  value = "${aws_s3_bucket.archive.arn}/${local.archive_prefix}/*"
}
