output "site_bucket" {
  description = "Private versioned bucket receiving the static bundle."
  value       = aws_s3_bucket.site.id
}

output "distribution_id" {
  description = "CloudFront distribution used for invalidation and readback."
  value       = aws_cloudfront_distribution.site.id
}

output "access_log_bucket" {
  description = "Private bucket retaining CloudFront access logs for 90 days."
  value       = aws_s3_bucket.logs.id
}

output "observatory_url" {
  value = "https://${var.observatory_fqdn}"
}
