data "aws_caller_identity" "current" {}

resource "aws_cloudwatch_log_group" "runtime" {
  name              = "/agent-logic/runtime-v3/${var.polis_id}-${var.environment}"
  retention_in_days = var.log_retention_days
}

resource "aws_cloudwatch_log_metric_filter" "healthy_heartbeat" {
  name           = "${local.resource_prefix}-healthy-heartbeat"
  log_group_name = aws_cloudwatch_log_group.runtime.name
  pattern        = "{ $.event = \"runtime_health_heartbeat\" && $.polis_id = \"${var.polis_id}\" && $.ready_metric = 1 && $.live_metric = 1 }"

  metric_transformation {
    name      = "HealthyHeartbeat-${var.polis_id}-${var.environment}"
    namespace = "AgentLogic/RuntimeV3"
    value     = "1"
    unit      = "Count"
  }
}

resource "aws_sns_topic" "runtime_health" {
  name = local.resource_prefix
}

resource "aws_sns_topic_subscription" "email" {
  count     = var.notification_email == null ? 0 : 1
  topic_arn = aws_sns_topic.runtime_health.arn
  protocol  = "email"
  endpoint  = var.notification_email
}

resource "aws_cloudwatch_metric_alarm" "runtime_unhealthy" {
  alarm_name          = "${local.resource_prefix}-missing"
  alarm_description   = "Runtime v3 stopped emitting ready/live health heartbeats."
  namespace           = "AgentLogic/RuntimeV3"
  metric_name         = aws_cloudwatch_log_metric_filter.healthy_heartbeat.metric_transformation[0].name
  statistic           = "Sum"
  period              = 60
  evaluation_periods  = var.missing_heartbeat_periods
  datapoints_to_alarm = var.missing_heartbeat_periods
  threshold           = 1
  comparison_operator = "LessThanThreshold"
  treat_missing_data  = "breaching"

  alarm_actions = [aws_sns_topic.runtime_health.arn]
  ok_actions    = [aws_sns_topic.runtime_health.arn]
}

resource "aws_iam_role_policy" "runtime_cloudwatch" {
  count = var.runtime_instance_role_name == null ? 0 : 1
  name  = "${local.resource_prefix}-cloudwatch"
  role  = var.runtime_instance_role_name
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid      = "RuntimeHealthLogDiscovery"
        Effect   = "Allow"
        Action   = ["logs:DescribeLogGroups"]
        Resource = "*"
      },
      {
        Sid    = "RuntimeHealthLogDelivery"
        Effect = "Allow"
        Action = [
          "logs:CreateLogStream",
          "logs:DescribeLogStreams",
          "logs:PutLogEvents"
        ]
        Resource = "${aws_cloudwatch_log_group.runtime.arn}:*"
      }
    ]
  })
}

resource "aws_iam_role" "runtime_publisher" {
  count = length(var.runtime_publisher_principal_arns) == 0 ? 0 : 1
  name  = "${local.resource_prefix}-publisher"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        AWS = sort(tolist(var.runtime_publisher_principal_arns))
      }
      Action = "sts:AssumeRole"
    }]
  })
  tags = local.common_tags
}

resource "aws_iam_role_policy" "runtime_publisher" {
  count = length(aws_iam_role.runtime_publisher)
  name  = "${local.resource_prefix}-cloudwatch"
  role  = aws_iam_role.runtime_publisher[0].id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid      = "RuntimeHealthLogDiscovery"
        Effect   = "Allow"
        Action   = ["logs:DescribeLogGroups"]
        Resource = "*"
      },
      {
        Sid    = "RuntimeHealthLogDelivery"
        Effect = "Allow"
        Action = [
          "logs:CreateLogStream",
          "logs:DescribeLogStreams",
          "logs:PutLogEvents"
        ]
        Resource = "${aws_cloudwatch_log_group.runtime.arn}:*"
      }
    ]
  })
}

resource "aws_ssm_document" "recover_runtime" {
  name            = "${local.resource_prefix}-recover"
  document_type   = "Command"
  document_format = "JSON"
  content = jsonencode({
    schemaVersion = "2.2"
    description   = "Idempotently recover one unhealthy Agent Logic Runtime v3 node through CSM."
    mainSteps = [{
      action = "aws:runShellScript"
      name   = "recoverRuntimeV3"
      inputs = {
        timeoutSeconds = "120"
        runCommand = concat(
          [
            "set -eu",
            "if ${local.csm_command} runtime-v3 status --init ${var.runtime_init_path} --json; then exit 0; fi"
          ],
          var.runtime_plist_path == null ? [] : ["test -f ${var.runtime_plist_path}"],
          [
            "${local.csm_command} runtime-v3 start --init ${var.runtime_init_path}${var.runtime_plist_path == null ? "" : " --plist ${var.runtime_plist_path}"} --json",
            "${local.csm_command} runtime-v3 status --init ${var.runtime_init_path} --json"
          ]
        )
      }
    }]
  })
}

resource "aws_iam_role" "eventbridge_recovery" {
  name = "${local.resource_prefix}-eventbridge"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect = "Allow"
      Principal = {
        Service = "events.amazonaws.com"
      }
      Action = "sts:AssumeRole"
    }]
  })
}

resource "aws_iam_role_policy" "eventbridge_recovery" {
  name = "${local.resource_prefix}-send-command"
  role = aws_iam_role.eventbridge_recovery.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid      = "UseExactRecoveryDocument"
        Effect   = "Allow"
        Action   = "ssm:SendCommand"
        Resource = aws_ssm_document.recover_runtime.arn
      },
      {
        Sid      = "TargetTaggedEc2RuntimeNode"
        Effect   = "Allow"
        Action   = "ssm:SendCommand"
        Resource = "arn:aws:ec2:${var.aws_region}:${data.aws_caller_identity.current.account_id}:instance/*"
        Condition = {
          StringEquals = {
            "ec2:ResourceTag/AgentLogicPolisId" = var.polis_id
            "ec2:ResourceTag/Environment"       = var.environment
          }
        }
      },
      {
        Sid      = "TargetTaggedHybridRuntimeNode"
        Effect   = "Allow"
        Action   = "ssm:SendCommand"
        Resource = "arn:aws:ssm:${var.aws_region}:${data.aws_caller_identity.current.account_id}:managed-instance/*"
        Condition = {
          StringEquals = {
            "ssm:resourceTag/AgentLogicPolisId" = var.polis_id
            "ssm:resourceTag/Environment"       = var.environment
          }
        }
      }
    ]
  })
}

resource "aws_cloudwatch_event_rule" "recover_runtime" {
  name        = "${local.resource_prefix}-recover"
  description = "Invoke bounded SSM recovery only when this Polis heartbeat alarm enters ALARM."
  event_pattern = jsonencode({
    source      = ["aws.cloudwatch"]
    detail-type = ["CloudWatch Alarm State Change"]
    resources   = [aws_cloudwatch_metric_alarm.runtime_unhealthy.arn]
    detail = {
      state = {
        value = ["ALARM"]
      }
    }
  })
}

resource "aws_cloudwatch_event_target" "recover_runtime" {
  rule     = aws_cloudwatch_event_rule.recover_runtime.name
  arn      = aws_ssm_document.recover_runtime.arn
  role_arn = aws_iam_role.eventbridge_recovery.arn

  run_command_targets {
    key    = "tag:AgentLogicPolisId"
    values = [var.polis_id]
  }


  run_command_targets {
    key    = "tag:Environment"
    values = [var.environment]
  }
}

resource "aws_cloudwatch_event_rule" "ssm_terminal_result" {
  name        = "${local.resource_prefix}-ssm-result"
  description = "Report terminal outcomes from this Runtime recovery document."
  event_pattern = jsonencode({
    source      = ["aws.ssm"]
    detail-type = ["EC2 Command Status-change Notification"]
    detail = {
      "document-name" = [aws_ssm_document.recover_runtime.name]
      status          = ["Success", "Failed", "TimedOut", "Cancelled"]
    }
  })
}

resource "aws_cloudwatch_event_target" "ssm_terminal_result" {
  rule = aws_cloudwatch_event_rule.ssm_terminal_result.name
  arn  = aws_sns_topic.runtime_health.arn
}

data "aws_iam_policy_document" "runtime_health_topic" {
  statement {
    sid    = "AllowAccountAdministration"
    effect = "Allow"
    principals {
      type        = "AWS"
      identifiers = ["arn:aws:iam::${data.aws_caller_identity.current.account_id}:root"]
    }
    actions = [
      "sns:GetTopicAttributes",
      "sns:SetTopicAttributes",
      "sns:AddPermission",
      "sns:RemovePermission",
      "sns:DeleteTopic",
      "sns:Subscribe",
      "sns:ListSubscriptionsByTopic",
      "sns:Publish"
    ]
    resources = [aws_sns_topic.runtime_health.arn]
  }

  statement {
    sid    = "AllowCloudWatchAlarmNotifications"
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["cloudwatch.amazonaws.com"]
    }
    actions   = ["sns:Publish"]
    resources = [aws_sns_topic.runtime_health.arn]
    condition {
      test     = "ArnEquals"
      variable = "aws:SourceArn"
      values   = [aws_cloudwatch_metric_alarm.runtime_unhealthy.arn]
    }
  }

  statement {
    sid    = "AllowEventBridgeTerminalResults"
    effect = "Allow"
    principals {
      type        = "Service"
      identifiers = ["events.amazonaws.com"]
    }
    actions   = ["sns:Publish"]
    resources = [aws_sns_topic.runtime_health.arn]
    condition {
      test     = "ArnEquals"
      variable = "aws:SourceArn"
      values   = [aws_cloudwatch_event_rule.ssm_terminal_result.arn]
    }
  }
}

resource "aws_sns_topic_policy" "runtime_health" {
  arn    = aws_sns_topic.runtime_health.arn
  policy = data.aws_iam_policy_document.runtime_health_topic.json
}
