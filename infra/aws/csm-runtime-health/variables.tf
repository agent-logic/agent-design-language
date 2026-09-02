variable "aws_region" {
  description = "AWS region containing the Runtime managed node."
  type        = string
  default     = "us-west-2"
}

variable "environment" {
  description = "Deployment environment."
  type        = string
  validation {
    condition     = contains(["dev", "prod"], var.environment)
    error_message = "environment must be dev or prod."
  }
}

variable "polis_id" {
  description = "Stable Polis identity used for logs, metrics, alarms, and exact managed-node targeting."
  type        = string
  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{0,62}$", var.polis_id))
    error_message = "polis_id must be a bounded lower-case DNS label."
  }
}

variable "runtime_instance_role_name" {
  description = "Optional IAM role attached to a Runtime EC2 instance. The module grants only CloudWatch Logs delivery."
  type        = string
  default     = null
}

variable "runtime_publisher_principal_arns" {
  description = "Principals allowed to assume the narrow Runtime health publisher role (for example a Wuji operator principal)."
  type        = set(string)
  default     = []
}

variable "runtime_init_path" {
  description = "Absolute Runtime v3 init path used by the bounded SSM recovery document."
  type        = string
  default     = "/opt/agent-logic/runtime-v3/runtime-init.toml"
  validation {
    condition     = can(regex("^/[A-Za-z0-9._/-]+$", var.runtime_init_path))
    error_message = "runtime_init_path must be an absolute shell-safe path."
  }
}

variable "runtime_plist_path" {
  description = "Optional absolute launchd plist path used to bootstrap an unloaded macOS Runtime service."
  type        = string
  default     = null
  validation {
    condition     = var.runtime_plist_path == null || can(regex("^/[A-Za-z0-9._/-]+$", var.runtime_plist_path))
    error_message = "runtime_plist_path must be null or an absolute shell-safe path."
  }
}

variable "runtime_run_as_user" {
  description = "Optional local account used to run CSM (required for a per-user launchd service)."
  type        = string
  default     = null
  validation {
    condition     = var.runtime_run_as_user == null || can(regex("^[A-Za-z_][A-Za-z0-9_-]*$", var.runtime_run_as_user))
    error_message = "runtime_run_as_user must be null or a shell-safe local account name."
  }
}

variable "csm_binary_path" {
  description = "Absolute CSM binary path on the Runtime node."
  type        = string
  default     = "/opt/agent-logic/bin/csm"
  validation {
    condition     = can(regex("^/[A-Za-z0-9._/-]+$", var.csm_binary_path))
    error_message = "csm_binary_path must be an absolute shell-safe path."
  }
}

variable "missing_heartbeat_periods" {
  description = "Consecutive one-minute periods without a healthy heartbeat before recovery."
  type        = number
  default     = 3
  validation {
    condition     = var.missing_heartbeat_periods >= 2 && var.missing_heartbeat_periods <= 10
    error_message = "missing_heartbeat_periods must be between 2 and 10."
  }
}

variable "notification_email" {
  description = "Optional email subscription for alarm and SSM terminal-result notifications. Confirmation is external."
  type        = string
  default     = null
}

variable "log_retention_days" {
  description = "CloudWatch Logs retention."
  type        = number
  default     = 30
}

variable "tags" {
  description = "Additional resource tags."
  type        = map(string)
  default     = {}
}
