# Separate from the Runtime publisher: no log writes, SSM, IAM or direct SQS.
resource "aws_iam_role" "shepherd" {
  count = length(var.shepherd_principal_arns) == 0 ? 0 : 1
  name  = "adl-${var.polis_id}-${var.environment}-shepherd"
  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { AWS = sort(tolist(var.shepherd_principal_arns)) }
      Action    = "sts:AssumeRole"
    }]
  })
  tags = local.common_tags
}

resource "aws_iam_role_policy" "shepherd" {
  count = length(aws_iam_role.shepherd)
  name  = "shepherd-read-and-alert"
  role  = aws_iam_role.shepherd[0].id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Effect   = "Allow"
        Action   = ["logs:DescribeLogStreams", "logs:GetLogEvents", "logs:FilterLogEvents"]
        Resource = [aws_cloudwatch_log_group.runtime.arn, "${aws_cloudwatch_log_group.runtime.arn}:*"]
      },
      {
        Effect    = "Allow"
        Action    = ["cloudwatch:GetMetricData", "cloudwatch:GetMetricStatistics", "cloudwatch:ListMetrics", "cloudwatch:DescribeAlarms"]
        Resource  = "*"
        Condition = { StringEquals = { "aws:RequestedRegion" = var.aws_region } }
      },
      {
        Effect   = "Allow"
        Action   = ["sns:Publish"]
        Resource = aws_sns_topic.runtime_health.arn
      }
    ]
  })
}

resource "aws_sqs_queue" "shepherd_alerts" {
  count                     = length(aws_iam_role.shepherd)
  name                      = "adl-${var.polis_id}-${var.environment}-shepherd-alerts"
  sqs_managed_sse_enabled   = true
  message_retention_seconds = 1209600
  tags                      = local.common_tags
}

resource "aws_sqs_queue_policy" "shepherd_alerts" {
  count     = length(aws_sqs_queue.shepherd_alerts)
  queue_url = aws_sqs_queue.shepherd_alerts[0].url
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "sns.amazonaws.com" }
      Action    = "sqs:SendMessage"
      Resource  = aws_sqs_queue.shepherd_alerts[0].arn
      Condition = {
        ArnEquals    = { "aws:SourceArn" = aws_sns_topic.runtime_health.arn }
        StringEquals = { "aws:SourceAccount" = data.aws_caller_identity.current.account_id }
      }
    }]
  })
}

resource "aws_sns_topic_subscription" "shepherd_alerts" {
  count                = length(aws_sqs_queue.shepherd_alerts)
  topic_arn            = aws_sns_topic.runtime_health.arn
  protocol             = "sqs"
  endpoint             = aws_sqs_queue.shepherd_alerts[0].arn
  raw_message_delivery = true
  depends_on           = [aws_sqs_queue_policy.shepherd_alerts]
}
