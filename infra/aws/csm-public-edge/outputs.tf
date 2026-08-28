output "observatory_fqdn" {
  value = local.observatory_fqdn
}

output "api_fqdn" {
  value = local.api_fqdn
}

output "wss_fqdn" {
  value = local.wss_fqdn
}

output "origin_fqdn" {
  value = local.origin_fqdn
}

output "origin_cname_target" {
  value = var.origin_cname_target
}

output "hosted_zone_id" {
  value = local.hosted_zone_id
}

output "hosted_zone_name_servers" {
  description = "Name servers to delegate from the parent domain when create_hosted_zone is true."
  value       = local.hosted_zone_name_servers
}

output "observatory_cloudfront_domain" {
  value = aws_cloudfront_distribution.observatory.domain_name
}

output "api_cloudfront_domain" {
  value = aws_cloudfront_distribution.api.domain_name
}

output "wss_cloudfront_domain" {
  value = aws_cloudfront_distribution.wss.domain_name
}

output "observatory_bucket" {
  value = aws_s3_bucket.observatory.bucket
}

output "runtime_http_api_endpoint" {
  description = "Raw API Gateway execute-api endpoint. This remains a direct public API Gateway endpoint and does not traverse the CloudFront/WAF edge; use api_fqdn for the governed public edge."
  value       = aws_apigatewayv2_api.runtime_http.api_endpoint
}
