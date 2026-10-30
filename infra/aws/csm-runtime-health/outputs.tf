output "cloudwatch_log_group_name" {
  value = aws_cloudwatch_log_group.runtime.name
}

output "missing_heartbeat_alarm_arn" {
  value = aws_cloudwatch_metric_alarm.runtime_unhealthy.arn
}

output "recovery_document_name" {
  value = aws_ssm_document.recover_runtime.name
}

output "notification_topic_arn" {
  value = aws_sns_topic.runtime_health.arn
}

output "runtime_publisher_role_arn" {
  value = try(aws_iam_role.runtime_publisher[0].arn, null)
}
