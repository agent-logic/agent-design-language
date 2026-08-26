output "observatory_fqdn" {
  value = local.observatory_fqdn
}

output "api_fqdn" {
  value = local.api_fqdn
}

output "wss_fqdn" {
  value = local.wss_fqdn
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
  value = aws_apigatewayv2_api.runtime_http.api_endpoint
}
