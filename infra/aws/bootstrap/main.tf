data "aws_caller_identity" "current" {}

data "aws_partition" "current" {}

locals {
  account_id = data.aws_caller_identity.current.account_id
  partition  = data.aws_partition.current.partition

  base_name = "${var.name_prefix}-${var.environment}"

  state_bucket_name  = lower("${local.base_name}-terraform-state-${local.account_id}-${var.aws_region}")
  lock_table_name    = "${local.base_name}-terraform-locks"
  deploy_role_name   = "${local.base_name}-terraform-deploy"
  deploy_policy_name = "${local.base_name}-terraform-backend-access"

  deployment_principals = length(var.trusted_deployment_principal_arns) > 0 ? var.trusted_deployment_principal_arns : [
    "arn:${local.partition}:iam::${local.account_id}:root"
  ]

  tags = merge(
    {
      Project     = "agent-logic"
      Component   = "terraform-bootstrap"
      Environment = var.environment
      ManagedBy   = "terraform"
      Issue       = "486"
    },
    var.tags
  )
}

resource "aws_s3_bucket" "terraform_state" {
  bucket = local.state_bucket_name
}

resource "aws_s3_bucket_public_access_block" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_ownership_controls" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id

  rule {
    object_ownership = "BucketOwnerEnforced"
  }
}

resource "aws_s3_bucket_versioning" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id

  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "terraform_state" {
  bucket = aws_s3_bucket.terraform_state.id

  rule {
    id     = "retain-noncurrent-terraform-state"
    status = "Enabled"

    filter {
      prefix = ""
    }

    noncurrent_version_expiration {
      noncurrent_days = var.state_retention_days
    }
  }
}

resource "aws_dynamodb_table" "terraform_locks" {
  name         = local.lock_table_name
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "LockID"

  attribute {
    name = "LockID"
    type = "S"
  }

  point_in_time_recovery {
    enabled = true
  }

  server_side_encryption {
    enabled = true
  }
}

data "aws_iam_policy_document" "terraform_deploy_assume_role" {
  statement {
    sid     = "AllowConfiguredDeploymentPrincipals"
    effect  = "Allow"
    actions = ["sts:AssumeRole"]

    principals {
      type        = "AWS"
      identifiers = local.deployment_principals
    }
  }
}

resource "aws_iam_role" "terraform_deploy" {
  name               = local.deploy_role_name
  assume_role_policy = data.aws_iam_policy_document.terraform_deploy_assume_role.json
  description        = "Scoped Terraform deployment role for Agent Logic account-foundation bootstrap work."
}

data "aws_iam_policy_document" "terraform_backend_access" {
  statement {
    sid    = "ReadTerraformStateBucketMetadata"
    effect = "Allow"
    actions = [
      "s3:GetBucketLocation",
      "s3:GetBucketVersioning",
      "s3:ListBucket"
    ]
    resources = [aws_s3_bucket.terraform_state.arn]
  }

  statement {
    sid    = "ReadWriteTerraformStateObjects"
    effect = "Allow"
    actions = [
      "s3:DeleteObject",
      "s3:GetObject",
      "s3:GetObjectVersion",
      "s3:PutObject"
    ]
    resources = ["${aws_s3_bucket.terraform_state.arn}/*"]
  }

  statement {
    sid    = "UseTerraformLockTable"
    effect = "Allow"
    actions = [
      "dynamodb:DeleteItem",
      "dynamodb:DescribeTable",
      "dynamodb:GetItem",
      "dynamodb:PutItem",
      "dynamodb:UpdateItem"
    ]
    resources = [aws_dynamodb_table.terraform_locks.arn]
  }

  statement {
    sid       = "ReadCallerIdentity"
    effect    = "Allow"
    actions   = ["sts:GetCallerIdentity"]
    resources = ["*"]
  }
}

resource "aws_iam_policy" "terraform_backend_access" {
  name        = local.deploy_policy_name
  description = "Least-privilege access to the Agent Logic Terraform state bucket and lock table."
  policy      = data.aws_iam_policy_document.terraform_backend_access.json
}

resource "aws_iam_role_policy_attachment" "terraform_backend_access" {
  role       = aws_iam_role.terraform_deploy.name
  policy_arn = aws_iam_policy.terraform_backend_access.arn
}
