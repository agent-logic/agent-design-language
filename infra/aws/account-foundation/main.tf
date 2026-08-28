data "aws_caller_identity" "current" {}
data "aws_region" "current" {}
data "aws_partition" "current" {}

locals {
  normalized_prefix   = lower(replace(var.name_prefix, "_", "-"))
  resource_prefix     = "${local.normalized_prefix}-${var.environment}"
  audit_bucket_name   = "${local.resource_prefix}-audit-${data.aws_caller_identity.current.account_id}-${data.aws_region.current.name}"
  finding_owner       = var.finding_owner
  finding_destination = var.finding_destination

  common_tags = {
    Project             = "agent-design-language"
    Environment         = var.environment
    ManagedBy           = "terraform"
    Issue               = "487"
    finding_owner       = local.finding_owner
    finding_destination = local.finding_destination
  }
}

resource "aws_kms_key" "audit" {
  description             = "KMS key for ADL account-foundation audit and security evidence"
  deletion_window_in_days = 30
  enable_key_rotation     = true

  tags = merge(local.common_tags, {
    Name = "${local.resource_prefix}-audit-kms"
  })
}

resource "aws_kms_alias" "audit" {
  name          = "alias/${local.resource_prefix}-audit"
  target_key_id = aws_kms_key.audit.key_id
}

resource "aws_s3_bucket" "audit_logs" {
  bucket        = local.audit_bucket_name
  force_destroy = false

  tags = merge(local.common_tags, {
    Name      = local.audit_bucket_name
    retention = tostring(var.log_retention_days)
  })
}

resource "aws_s3_bucket_public_access_block" "audit_logs" {
  bucket                  = aws_s3_bucket.audit_logs.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_versioning" "audit_logs" {
  bucket = aws_s3_bucket.audit_logs.id

  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "audit_logs" {
  bucket = aws_s3_bucket.audit_logs.id

  rule {
    apply_server_side_encryption_by_default {
      kms_master_key_id = aws_kms_key.audit.arn
      sse_algorithm     = "aws:kms"
    }
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "audit_logs" {
  bucket = aws_s3_bucket.audit_logs.id

  rule {
    id     = "retain-audit-evidence"
    status = "Enabled"

    filter {
      prefix = ""
    }

    expiration {
      days = var.log_retention_days
    }

    noncurrent_version_expiration {
      noncurrent_days = var.log_retention_days
    }
  }
}

resource "aws_s3_bucket_policy" "audit_logs" {
  bucket = aws_s3_bucket.audit_logs.id
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid       = "CloudTrailAclCheck"
        Effect    = "Allow"
        Principal = { Service = "cloudtrail.amazonaws.com" }
        Action    = "s3:GetBucketAcl"
        Resource  = aws_s3_bucket.audit_logs.arn
      },
      {
        Sid       = "CloudTrailWrite"
        Effect    = "Allow"
        Principal = { Service = "cloudtrail.amazonaws.com" }
        Action    = "s3:PutObject"
        Resource  = "${aws_s3_bucket.audit_logs.arn}/AWSLogs/${data.aws_caller_identity.current.account_id}/*"
        Condition = {
          StringEquals = {
            "s3:x-amz-acl" = "bucket-owner-full-control"
          }
        }
      }
    ]
  })
}

resource "aws_cloudtrail" "account_activity" {
  name                          = "${local.resource_prefix}-account-activity"
  s3_bucket_name                = aws_s3_bucket.audit_logs.id
  include_global_service_events = true
  is_multi_region_trail         = var.cloudtrail_multi_region
  enable_log_file_validation    = true
  kms_key_id                    = aws_kms_key.audit.arn

  event_selector {
    read_write_type           = "All"
    include_management_events = true
  }

  tags = merge(local.common_tags, {
    Name = "${local.resource_prefix}-account-activity"
  })

  depends_on = [aws_s3_bucket_policy.audit_logs]
}

resource "aws_iam_role" "config" {
  count = var.enable_config_recorder ? 1 : 0
  name  = "${local.resource_prefix}-config-recorder"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Effect    = "Allow"
      Principal = { Service = "config.amazonaws.com" }
      Action    = "sts:AssumeRole"
    }]
  })

  tags = local.common_tags
}

resource "aws_iam_role_policy_attachment" "config" {
  count      = var.enable_config_recorder ? 1 : 0
  role       = aws_iam_role.config[0].name
  policy_arn = "arn:${data.aws_partition.current.partition}:iam::aws:policy/service-role/AWS_ConfigRole"
}

resource "aws_config_configuration_recorder" "account" {
  count    = var.enable_config_recorder ? 1 : 0
  name     = "${local.resource_prefix}-config-recorder"
  role_arn = aws_iam_role.config[0].arn

  recording_group {
    all_supported                 = true
    include_global_resource_types = true
  }
}

resource "aws_config_delivery_channel" "account" {
  count          = var.enable_config_recorder ? 1 : 0
  name           = "${local.resource_prefix}-config-delivery"
  s3_bucket_name = aws_s3_bucket.audit_logs.id

  snapshot_delivery_properties {
    delivery_frequency = "TwentyFour_Hours"
  }

  depends_on = [aws_config_configuration_recorder.account]
}

resource "aws_config_configuration_recorder_status" "account" {
  count      = var.enable_config_recorder ? 1 : 0
  name       = aws_config_configuration_recorder.account[0].name
  is_enabled = true

  depends_on = [aws_config_delivery_channel.account]
}

resource "aws_accessanalyzer_analyzer" "account" {
  count         = var.enable_access_analyzer ? 1 : 0
  analyzer_name = "${local.resource_prefix}-access-analyzer"
  type          = "ACCOUNT"

  tags = merge(local.common_tags, {
    Name = "${local.resource_prefix}-access-analyzer"
  })
}

resource "aws_sns_topic" "security_findings" {
  name              = "${local.resource_prefix}-security-findings"
  kms_master_key_id = aws_kms_key.audit.arn

  tags = merge(local.common_tags, {
    Name = "${local.resource_prefix}-security-findings"
  })
}

resource "aws_cloudwatch_event_rule" "access_analyzer_findings" {
  name        = "${local.resource_prefix}-access-analyzer-findings"
  description = "Routes IAM Access Analyzer findings to the declared #487 security findings destination."

  event_pattern = jsonencode({
    source      = ["aws.access-analyzer"]
    detail-type = ["Access Analyzer Finding"]
  })

  tags = merge(local.common_tags, {
    Name = "${local.resource_prefix}-access-analyzer-findings"
  })
}

resource "aws_cloudwatch_event_target" "access_analyzer_findings" {
  rule      = aws_cloudwatch_event_rule.access_analyzer_findings.name
  target_id = "security-findings-sns"
  arn       = aws_sns_topic.security_findings.arn
}

resource "aws_sns_topic_policy" "security_findings" {
  arn = aws_sns_topic.security_findings.arn

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [{
      Sid       = "AllowEventBridgePublish"
      Effect    = "Allow"
      Principal = { Service = "events.amazonaws.com" }
      Action    = "sns:Publish"
      Resource  = aws_sns_topic.security_findings.arn
    }]
  })
}
