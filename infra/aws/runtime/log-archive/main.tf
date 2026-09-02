resource "aws_s3_bucket" "archive" {
  bucket = local.bucket_name

  tags = merge(local.common_tags, {
    Name = local.bucket_name
  })
}

resource "aws_s3_bucket_public_access_block" "archive" {
  bucket = aws_s3_bucket.archive.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_ownership_controls" "archive" {
  bucket = aws_s3_bucket.archive.id

  rule {
    object_ownership = "BucketOwnerEnforced"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "archive" {
  bucket = aws_s3_bucket.archive.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket_versioning" "archive" {
  bucket = aws_s3_bucket.archive.id

  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_s3_bucket_lifecycle_configuration" "archive" {
  bucket = aws_s3_bucket.archive.id

  rule {
    id     = "runtime-log-archive-retention"
    status = "Enabled"

    filter {
      prefix = local.archive_prefix
    }

    expiration {
      days = 30
    }

    noncurrent_version_expiration {
      noncurrent_days = 7
    }

    abort_incomplete_multipart_upload {
      days_after_initiation = 1
    }
  }

  depends_on = [aws_s3_bucket_versioning.archive]
}

data "aws_iam_policy_document" "publisher" {
  statement {
    effect = "Allow"

    actions = [
      "s3:GetBucketLocation",
    ]

    resources = [
      aws_s3_bucket.archive.arn,
    ]
  }

  statement {
    effect = "Allow"

    actions = [
      "s3:AbortMultipartUpload",
      "s3:ListMultipartUploadParts",
      "s3:PutObject",
    ]

    resources = [
      "${aws_s3_bucket.archive.arn}/${local.archive_prefix}/*",
    ]
  }
}

resource "aws_iam_policy" "publisher" {
  name        = "agent-logic-runtime-log-archive-${var.environment}-${var.polis_id}-${var.runtime_id}"
  description = "Least-privilege Runtime redacted-log archive publisher for issue 594."
  policy      = data.aws_iam_policy_document.publisher.json

  tags = local.common_tags
}
