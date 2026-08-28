locals {
  zone_name           = trimsuffix(var.zone_name, ".")
  environment_segment = var.environment == "prod" ? "" : ".${var.environment}"
  origin_fqdn         = var.origin_fqdn_override == null ? "${var.csm_name}${local.environment_segment}.${local.zone_name}" : trimsuffix(var.origin_fqdn_override, ".")
  resource_prefix     = "csm-${var.csm_name}-${var.environment}"

  common_tags = {
    Project     = "agent-logic-csm"
    Issue       = "122"
    Environment = var.environment
    CsmName     = var.csm_name
    ManagedBy   = "terraform"
  }
}
