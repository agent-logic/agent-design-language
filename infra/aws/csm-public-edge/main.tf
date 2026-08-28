data "aws_caller_identity" "current" {}

data "aws_route53_zone" "csm" {
  count        = var.create_hosted_zone || var.hosted_zone_id != null ? 0 : 1
  name         = "${local.zone_name}."
  private_zone = false
}

resource "aws_route53_zone" "csm" {
  count = var.create_hosted_zone ? 1 : 0
  name  = local.zone_name
  tags  = local.common_tags
}

resource "terraform_data" "approved_account_guard" {
  input = "agent-logic-business-account"

  lifecycle {
    precondition {
      condition     = data.aws_caller_identity.current.account_id == var.approved_aws_account_id
      error_message = "Refusing to apply #122 public edge outside the approved Agent Logic AWS account."
    }
  }
}

resource "terraform_data" "origin_mode_guard" {
  input = var.runtime_origin_mode

  lifecycle {
    precondition {
      condition = (
        contains(["external_https", "aws_alb_public"], var.runtime_origin_mode)
        ? var.runtime_origin_url != null && startswith(var.runtime_origin_url, "https://")
        : var.runtime_origin_url == null
      )
      error_message = "external_https/aws_alb_public require an HTTPS runtime_origin_url; aws_alb_private must not set runtime_origin_url."
    }

    precondition {
      condition = (
        var.runtime_origin_mode == "aws_alb_private"
        ? var.private_alb_listener_arn != null
        : var.private_alb_listener_arn == null
      )
      error_message = "aws_alb_private requires private_alb_listener_arn; public modes must not set it."
    }

    precondition {
      condition = var.runtime_origin_url == null || (
        !strcontains(var.runtime_origin_url, local.api_fqdn)
        && !strcontains(var.runtime_origin_url, local.observatory_fqdn)
        && !strcontains(var.runtime_origin_url, local.wss_fqdn)
      )
      error_message = "runtime_origin_url must not point back at this stack's public hostnames."
    }
  }
}

resource "terraform_data" "wss_origin_mode_guard" {
  input = var.wss_origin_mode

  lifecycle {
    precondition {
      condition     = startswith(var.wss_origin_https_url, "https://")
      error_message = "external_wss/aws_alb_public_wss require an HTTPS origin endpoint that accepts WebSocket upgrade."
    }

    precondition {
      condition     = length(var.wss_origin_hostname) > 0
      error_message = "external_wss/aws_alb_public_wss require wss_origin_hostname for CloudFront origin Host/SNI/TLS validation."
    }

    precondition {
      condition     = var.wss_origin_https_url == "https://${var.wss_origin_hostname}"
      error_message = "wss_origin_https_url must exactly equal https://wss_origin_hostname with no path; this root does not configure CloudFront origin_path, so path-bearing origins are rejected."
    }

    precondition {
      condition = (
        !strcontains(var.wss_origin_https_url, local.api_fqdn)
        && !strcontains(var.wss_origin_https_url, local.observatory_fqdn)
        && !strcontains(var.wss_origin_https_url, local.wss_fqdn)
      )
      error_message = "wss_origin_https_url must not point back at this stack's public hostnames."
    }

    precondition {
      condition = !var.wss_forward_viewer_host || (
        var.wss_origin_hostname == local.wss_fqdn
      )
      error_message = "wss_forward_viewer_host may be true only when the origin hostname and certificate are intentionally prepared for wss_fqdn."
    }
  }
}

resource "aws_route53_record" "origin_cname" {
  count   = var.origin_cname_target == null ? 0 : 1
  zone_id = local.hosted_zone_id
  name    = local.origin_fqdn
  type    = "CNAME"
  ttl     = 60
  records = [trimsuffix(var.origin_cname_target, ".")]
}

resource "aws_acm_certificate" "edge" {
  count                     = var.edge_acm_certificate_arn == null ? 1 : 0
  provider                  = aws.us_east_1
  domain_name               = local.observatory_fqdn
  subject_alternative_names = [local.api_fqdn, local.wss_fqdn]
  validation_method         = "DNS"
  tags                      = local.common_tags

  lifecycle {
    create_before_destroy = true
  }
}

resource "aws_route53_record" "edge_cert_validation" {
  for_each = var.edge_acm_certificate_arn == null ? {
    for option in aws_acm_certificate.edge[0].domain_validation_options :
    option.domain_name => {
      name   = option.resource_record_name
      record = option.resource_record_value
      type   = option.resource_record_type
    }
  } : {}

  zone_id = local.hosted_zone_id
  name    = each.value.name
  type    = each.value.type
  ttl     = 60
  records = [each.value.record]
}

resource "aws_acm_certificate_validation" "edge" {
  count                   = var.edge_acm_certificate_arn == null ? 1 : 0
  provider                = aws.us_east_1
  certificate_arn         = aws_acm_certificate.edge[0].arn
  validation_record_fqdns = [for record in aws_route53_record.edge_cert_validation : record.fqdn]
}

resource "aws_s3_bucket" "observatory" {
  bucket = "${local.resource_prefix}-observatory-assets"
  tags   = local.common_tags
}

resource "aws_s3_bucket_public_access_block" "observatory" {
  bucket                  = aws_s3_bucket.observatory.id
  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_versioning" "observatory" {
  bucket = aws_s3_bucket.observatory.id

  versioning_configuration {
    status = "Enabled"
  }
}

resource "aws_cloudwatch_log_group" "api" {
  name              = "/aws/apigateway/${local.resource_prefix}-runtime-http"
  retention_in_days = var.log_retention_days
  tags              = local.common_tags
}

resource "aws_apigatewayv2_api" "runtime_http" {
  name          = "${local.resource_prefix}-runtime-http"
  protocol_type = "HTTP"
  tags          = local.common_tags

  cors_configuration {
    allow_credentials = true
    allow_headers = [
      "authorization",
      "content-type",
      "origin",
      "x-csm-correlation-id",
      "x-request-id",
      "traceparent",
      "tracestate"
    ]
    allow_methods = ["GET", "POST", "OPTIONS"]
    allow_origins = local.allowed_origins
    max_age       = 300
  }
}

resource "aws_apigatewayv2_integration" "runtime_http" {
  api_id                 = aws_apigatewayv2_api.runtime_http.id
  integration_type       = "HTTP_PROXY"
  integration_method     = "ANY"
  integration_uri        = var.runtime_origin_mode == "aws_alb_private" ? null : var.runtime_origin_url
  payload_format_version = "1.0"

  lifecycle {
    precondition {
      condition     = var.runtime_origin_mode != "aws_alb_private"
      error_message = "aws_alb_private HTTP integration is reserved for a later reviewed private integration design."
    }
  }
}

resource "aws_apigatewayv2_route" "runtime_default" {
  api_id    = aws_apigatewayv2_api.runtime_http.id
  route_key = "$default"
  target    = "integrations/${aws_apigatewayv2_integration.runtime_http.id}"
}

resource "aws_apigatewayv2_stage" "runtime_default" {
  api_id      = aws_apigatewayv2_api.runtime_http.id
  name        = "$default"
  auto_deploy = true

  access_log_settings {
    destination_arn = aws_cloudwatch_log_group.api.arn
    format = jsonencode({
      requestId      = "$context.requestId"
      ip             = "$context.identity.sourceIp"
      requestTime    = "$context.requestTime"
      httpMethod     = "$context.httpMethod"
      routeKey       = "$context.routeKey"
      status         = "$context.status"
      protocol       = "$context.protocol"
      responseLength = "$context.responseLength"
    })
  }

  default_route_settings {
    throttling_burst_limit = 100
    throttling_rate_limit  = 50
  }
}

resource "aws_wafv2_web_acl" "edge" {
  provider    = aws.us_east_1
  name        = "${local.resource_prefix}-edge"
  description = "Issue #122 CSM public edge WAF for ${var.csm_name}/${var.environment}"
  scope       = "CLOUDFRONT"
  tags        = local.common_tags

  default_action {
    allow {}
  }

  rule {
    name     = "rate-limit"
    priority = 1

    action {
      block {}
    }

    statement {
      rate_based_statement {
        limit              = var.waf_rate_limit
        aggregate_key_type = "IP"
      }
    }

    visibility_config {
      cloudwatch_metrics_enabled = true
      metric_name                = "${local.resource_prefix}-rate-limit"
      sampled_requests_enabled   = true
    }
  }

  visibility_config {
    cloudwatch_metrics_enabled = true
    metric_name                = "${local.resource_prefix}-edge"
    sampled_requests_enabled   = true
  }
}

resource "aws_cloudfront_origin_access_control" "observatory" {
  name                              = "${local.resource_prefix}-observatory"
  description                       = "Issue #122 Observatory static asset access"
  origin_access_control_origin_type = "s3"
  signing_behavior                  = "always"
  signing_protocol                  = "sigv4"
}

data "aws_cloudfront_cache_policy" "caching_optimized" {
  name = "Managed-CachingOptimized"
}

data "aws_cloudfront_cache_policy" "caching_disabled" {
  name = "Managed-CachingDisabled"
}

data "aws_cloudfront_origin_request_policy" "all_viewer_except_host" {
  name = "Managed-AllViewerExceptHostHeader"
}

data "aws_cloudfront_origin_request_policy" "all_viewer" {
  name = "Managed-AllViewer"
}

resource "aws_cloudfront_origin_request_policy" "wss_without_host" {
  name    = "${local.resource_prefix}-wss-without-host"
  comment = "Issue #122 WSS policy: WebSocket/auth/correlation headers without viewer Host"

  cookies_config {
    cookie_behavior = length(var.wss_forwarded_cookies) == 0 ? "none" : "whitelist"

    dynamic "cookies" {
      for_each = length(var.wss_forwarded_cookies) == 0 ? [] : [1]
      content {
        items = var.wss_forwarded_cookies
      }
    }
  }

  headers_config {
    header_behavior = "whitelist"
    headers {
      items = [
        "Authorization",
        "Origin",
        "Sec-WebSocket-Key",
        "Sec-WebSocket-Version",
        "Sec-WebSocket-Protocol",
        "Sec-WebSocket-Extensions",
        "X-CSM-Correlation-Id",
        "X-Request-Id",
        "Traceparent",
        "Tracestate"
      ]
    }
  }

  query_strings_config {
    query_string_behavior = length(var.wss_forwarded_query_strings) == 0 ? "none" : "whitelist"

    dynamic "query_strings" {
      for_each = length(var.wss_forwarded_query_strings) == 0 ? [] : [1]
      content {
        items = var.wss_forwarded_query_strings
      }
    }
  }
}

resource "aws_cloudfront_distribution" "observatory" {
  enabled             = true
  comment             = "${local.resource_prefix} Observatory static edge"
  aliases             = [local.observatory_fqdn]
  default_root_object = "index.html"
  web_acl_id          = aws_wafv2_web_acl.edge.arn
  tags                = local.common_tags

  origin {
    domain_name              = aws_s3_bucket.observatory.bucket_regional_domain_name
    origin_id                = local.observatory_origin_id
    origin_access_control_id = aws_cloudfront_origin_access_control.observatory.id
  }

  default_cache_behavior {
    target_origin_id       = local.observatory_origin_id
    viewer_protocol_policy = "redirect-to-https"
    allowed_methods        = ["GET", "HEAD", "OPTIONS"]
    cached_methods         = ["GET", "HEAD"]
    cache_policy_id        = data.aws_cloudfront_cache_policy.caching_optimized.id
    compress               = true
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    acm_certificate_arn      = local.edge_acm_certificate_arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }
}

resource "aws_cloudfront_distribution" "api" {
  enabled    = true
  comment    = "${local.resource_prefix} Runtime HTTP API edge"
  aliases    = [local.api_fqdn]
  web_acl_id = aws_wafv2_web_acl.edge.arn
  tags       = local.common_tags

  origin {
    domain_name = trimprefix(aws_apigatewayv2_api.runtime_http.api_endpoint, "https://")
    origin_id   = local.api_origin_id

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  default_cache_behavior {
    target_origin_id         = local.api_origin_id
    viewer_protocol_policy   = "https-only"
    allowed_methods          = ["GET", "HEAD", "OPTIONS", "PUT", "POST", "PATCH", "DELETE"]
    cached_methods           = ["GET", "HEAD"]
    cache_policy_id          = data.aws_cloudfront_cache_policy.caching_disabled.id
    origin_request_policy_id = data.aws_cloudfront_origin_request_policy.all_viewer_except_host.id
    compress                 = false
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    acm_certificate_arn      = local.edge_acm_certificate_arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }
}

resource "aws_cloudfront_distribution" "wss" {
  enabled    = true
  comment    = "${local.resource_prefix} Runtime WSS realtime edge"
  aliases    = [local.wss_fqdn]
  web_acl_id = aws_wafv2_web_acl.edge.arn
  tags       = local.common_tags

  origin {
    domain_name = var.wss_origin_hostname
    origin_id   = local.wss_origin_id

    custom_header {
      name  = "X-CSM-Public-Host"
      value = local.wss_fqdn
    }

    custom_origin_config {
      http_port              = 80
      https_port             = 443
      origin_protocol_policy = "https-only"
      origin_ssl_protocols   = ["TLSv1.2"]
    }
  }

  default_cache_behavior {
    target_origin_id         = local.wss_origin_id
    viewer_protocol_policy   = "https-only"
    allowed_methods          = ["GET", "HEAD", "OPTIONS"]
    cached_methods           = ["GET", "HEAD"]
    cache_policy_id          = data.aws_cloudfront_cache_policy.caching_disabled.id
    origin_request_policy_id = var.wss_forward_viewer_host ? data.aws_cloudfront_origin_request_policy.all_viewer.id : aws_cloudfront_origin_request_policy.wss_without_host.id
    compress                 = false
  }

  ordered_cache_behavior {
    path_pattern             = var.websocket_path_pattern
    target_origin_id         = local.wss_origin_id
    viewer_protocol_policy   = "https-only"
    allowed_methods          = ["GET", "HEAD", "OPTIONS"]
    cached_methods           = ["GET", "HEAD"]
    cache_policy_id          = data.aws_cloudfront_cache_policy.caching_disabled.id
    origin_request_policy_id = var.wss_forward_viewer_host ? data.aws_cloudfront_origin_request_policy.all_viewer.id : aws_cloudfront_origin_request_policy.wss_without_host.id
    compress                 = false
  }

  restrictions {
    geo_restriction {
      restriction_type = "none"
    }
  }

  viewer_certificate {
    acm_certificate_arn      = local.edge_acm_certificate_arn
    ssl_support_method       = "sni-only"
    minimum_protocol_version = "TLSv1.2_2021"
  }
}

data "aws_iam_policy_document" "observatory_bucket" {
  statement {
    sid     = "AllowCloudFrontRead"
    actions = ["s3:GetObject"]
    resources = [
      "${aws_s3_bucket.observatory.arn}/*"
    ]

    principals {
      type        = "Service"
      identifiers = ["cloudfront.amazonaws.com"]
    }

    condition {
      test     = "StringEquals"
      variable = "AWS:SourceArn"
      values   = [aws_cloudfront_distribution.observatory.arn]
    }
  }
}

resource "aws_s3_bucket_policy" "observatory" {
  bucket = aws_s3_bucket.observatory.id
  policy = data.aws_iam_policy_document.observatory_bucket.json
}

resource "aws_route53_record" "observatory" {
  zone_id = local.hosted_zone_id
  name    = local.observatory_fqdn
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.observatory.domain_name
    zone_id                = aws_cloudfront_distribution.observatory.hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "api" {
  zone_id = local.hosted_zone_id
  name    = local.api_fqdn
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.api.domain_name
    zone_id                = aws_cloudfront_distribution.api.hosted_zone_id
    evaluate_target_health = false
  }
}

resource "aws_route53_record" "wss" {
  zone_id = local.hosted_zone_id
  name    = local.wss_fqdn
  type    = "A"

  alias {
    name                   = aws_cloudfront_distribution.wss.domain_name
    zone_id                = aws_cloudfront_distribution.wss.hosted_zone_id
    evaluate_target_health = false
  }
}
