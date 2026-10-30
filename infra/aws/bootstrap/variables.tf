variable "aws_region" {
  description = "AWS region that owns the Terraform bootstrap resources."
  type        = string
  default     = "us-west-2"
}

variable "name_prefix" {
  description = "Stable prefix for account-foundation bootstrap resources."
  type        = string
  default     = "agent-logic"

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{1,30}[a-z0-9]$", var.name_prefix))
    error_message = "name_prefix must be lowercase DNS-safe text."
  }
}

variable "environment" {
  description = "Environment label for bootstrap resources."
  type        = string
  default     = "foundation"

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{1,24}[a-z0-9]$", var.environment))
    error_message = "environment must be lowercase DNS-safe text."
  }
}

variable "trusted_deployment_principal_arns" {
  description = "IAM principal ARNs allowed to assume the Terraform deployment role. Defaults to the current account root so the role is usable after bootstrap."
  type        = list(string)
  default     = []
}

variable "state_retention_days" {
  description = "Noncurrent Terraform state object retention window."
  type        = number
  default     = 90

  validation {
    condition     = var.state_retention_days >= 30
    error_message = "state_retention_days must be at least 30."
  }
}

variable "tags" {
  description = "Additional tags applied to every bootstrap resource."
  type        = map(string)
  default     = {}
}
