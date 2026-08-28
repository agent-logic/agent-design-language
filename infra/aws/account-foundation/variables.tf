variable "environment" {
  description = "Environment label for account-foundation audit/security resources."
  type        = string
  default     = "dev"
}

variable "name_prefix" {
  description = "Stable prefix for account-foundation resources."
  type        = string
  default     = "agent-logic"
}

variable "aws_profile" {
  description = "Approved business AWS profile used by Terraform plan/apply for this account-foundation root."
  type        = string
  default     = "agent-logic-admin"

  validation {
    condition     = var.aws_profile == "agent-logic-admin"
    error_message = "aws_profile must be agent-logic-admin for ADL AWS-D account-foundation operations."
  }
}

variable "finding_owner" {
  description = "Stable owner for enabled audit/security findings."
  type        = string
  default     = "agent-logic-cloud-ops"
}

variable "finding_destination" {
  description = "Human-readable destination for enabled audit/security findings."
  type        = string
  default     = "security-ops-sns-topic"
}

variable "log_retention_days" {
  description = "S3 lifecycle retention for audit logs."
  type        = number
  default     = 365

  validation {
    condition     = var.log_retention_days >= 90
    error_message = "log_retention_days must be at least 90 days."
  }
}

variable "cloudtrail_multi_region" {
  description = "Whether CloudTrail records all regions."
  type        = bool
  default     = true
}

variable "enable_config_recorder" {
  description = "Enable AWS Config recorder and delivery channel."
  type        = bool
  default     = true
}

variable "enable_access_analyzer" {
  description = "Enable IAM Access Analyzer for account trust-edge visibility."
  type        = bool
  default     = true
}
