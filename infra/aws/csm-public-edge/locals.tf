locals {
  zone_name           = trimsuffix(var.zone_name, ".")
  environment_segment = var.environment == "prod" ? "" : ".${var.environment}"
  csm_domain_suffix   = "${var.csm_name}${local.environment_segment}.${local.zone_name}"
  observatory_fqdn    = "observatory.${local.csm_domain_suffix}"
  api_fqdn            = "api.${local.csm_domain_suffix}"
  wss_fqdn            = "wss.${local.csm_domain_suffix}"
  origin_fqdn         = "${var.csm_name}${local.environment_segment}.${local.zone_name}"
  resource_prefix     = "csm-${var.csm_name}-${var.environment}"
  hosted_zone_ids = concat(
    var.hosted_zone_id == null ? [] : [var.hosted_zone_id],
    aws_route53_zone.csm[*].zone_id,
    data.aws_route53_zone.csm[*].zone_id
  )
  hosted_zone_id           = one(local.hosted_zone_ids)
  hosted_zone_name_servers = try(aws_route53_zone.csm[0].name_servers, [])
  edge_acm_certificate_arn = var.edge_acm_certificate_arn != null ? var.edge_acm_certificate_arn : aws_acm_certificate_validation.edge[0].certificate_arn

  observatory_origin_id = "${local.resource_prefix}-observatory-s3"
  api_origin_id         = "${local.resource_prefix}-runtime-http-api"
  wss_origin_id         = "${local.resource_prefix}-runtime-wss"

  allowed_origins = distinct(concat(
    ["https://${local.observatory_fqdn}"],
    var.additional_allowed_origins
  ))

  common_tags = {
    Project     = "agent-logic-csm"
    Issue       = "122"
    Environment = var.environment
    CsmName     = var.csm_name
    ManagedBy   = "terraform"
  }

}
