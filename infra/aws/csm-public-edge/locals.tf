locals {
  zone_name        = trimsuffix(var.zone_name, ".")
  observatory_fqdn = "observatory.${var.csm_name}.${var.environment}.${local.zone_name}"
  api_fqdn         = "api.${var.csm_name}.${var.environment}.${local.zone_name}"
  wss_fqdn         = "wss.${var.csm_name}.${var.environment}.${local.zone_name}"
  resource_prefix  = "csm-${var.csm_name}-${var.environment}"

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
