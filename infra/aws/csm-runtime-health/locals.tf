locals {
  resource_prefix = "adl-${var.polis_id}-${var.environment}-runtime-health"
  csm_command     = var.runtime_run_as_user == null ? var.csm_binary_path : "sudo -H -u ${var.runtime_run_as_user} -- ${var.csm_binary_path}"
  common_tags = merge(var.tags, {
    Application       = "agent-logic-runtime"
    Environment       = var.environment
    AgentLogicPolisId = var.polis_id
    ManagedBy         = "terraform"
  })
}
