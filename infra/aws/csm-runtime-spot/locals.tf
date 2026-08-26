locals {
  resource_prefix = "csm-${var.csm_name}-${var.environment}"

  common_tags = {
    Project     = "agent-logic-csm"
    Issue       = "122"
    Environment = var.environment
    CsmName     = var.csm_name
    ManagedBy   = "terraform"
  }
}
