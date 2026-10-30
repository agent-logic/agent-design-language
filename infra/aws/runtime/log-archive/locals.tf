locals {
  archive_prefix = "logs/env=${var.environment}/polis=${var.polis_id}/runtime=${var.runtime_id}"
  bucket_name    = coalesce(var.bucket_name, "agent-logic-runtime-log-archive-${var.environment}-${var.polis_id}-${var.runtime_id}")

  common_tags = {
    Project     = "agent-logic-runtime"
    Issue       = "594"
    Environment = var.environment
    PolisId     = var.polis_id
    RuntimeId   = var.runtime_id
    ManagedBy   = "terraform"
  }
}
