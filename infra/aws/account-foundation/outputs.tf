output "audit_bucket_name" {
  description = "S3 bucket that stores audit/security evidence."
  value       = aws_s3_bucket.audit_logs.id
}

output "cloudtrail_name" {
  description = "Account activity CloudTrail name."
  value       = aws_cloudtrail.account_activity.name
}

output "config_recorder_name" {
  description = "AWS Config recorder name when enabled."
  value       = try(aws_config_configuration_recorder.account[0].name, null)
}

output "access_analyzer_name" {
  description = "IAM Access Analyzer name when enabled."
  value       = try(aws_accessanalyzer_analyzer.account[0].analyzer_name, null)
}

output "security_findings_topic_arn" {
  description = "SNS topic ARN for security findings routing."
  value       = aws_sns_topic.security_findings.arn
}

output "finding_owner" {
  description = "Declared owner for enabled finding producers."
  value       = local.finding_owner
}

output "finding_destination" {
  description = "Declared destination for enabled finding producers."
  value       = local.finding_destination
}

