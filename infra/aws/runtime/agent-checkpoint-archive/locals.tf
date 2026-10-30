locals {
  bucket_name    = coalesce(var.bucket_name, "agent-logic-runtime-agent-checkpoints-${var.environment}")
  archive_prefix = "v1/polis"
  common_tags = merge(var.tags, {
    Application = "agent-logic-runtime"
    Environment = var.environment
    ManagedBy   = "terraform"
    Purpose     = "agent-partial-checkpoints"
  })
}
