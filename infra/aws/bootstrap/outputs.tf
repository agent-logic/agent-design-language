output "aws_account_id" {
  description = "AWS account ID where bootstrap resources are managed."
  value       = local.account_id
}

output "aws_region" {
  description = "AWS region where bootstrap resources are managed."
  value       = var.aws_region
}

output "state_bucket_name" {
  description = "Dedicated encrypted/versioned S3 bucket for Terraform state."
  value       = aws_s3_bucket.terraform_state.bucket
}

output "lock_table_name" {
  description = "Dedicated DynamoDB table for Terraform state locks."
  value       = aws_dynamodb_table.terraform_locks.name
}

output "deployment_role_arn" {
  description = "Scoped deployment role ARN for future Terraform account-foundation work."
  value       = aws_iam_role.terraform_deploy.arn
}

output "backend_hcl" {
  description = "Backend stanza values for future Terraform roots."
  value = {
    bucket         = aws_s3_bucket.terraform_state.bucket
    dynamodb_table = aws_dynamodb_table.terraform_locks.name
    region         = var.aws_region
    encrypt        = true
  }
}
