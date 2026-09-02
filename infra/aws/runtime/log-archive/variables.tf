variable "aws_region" {
  description = "AWS region for the Runtime log archive."
  type        = string
  default     = "us-west-2"
}

variable "environment" {
  description = "Deployment environment used in archive object prefixes."
  type        = string
  default     = "dev"

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{0,61}[a-z0-9]$", var.environment))
    error_message = "environment must be a lowercase DNS-safe label."
  }
}

variable "polis_id" {
  description = "Polis identifier used in archive object prefixes."
  type        = string
  default     = "konishi"

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{0,61}[a-z0-9]$", var.polis_id))
    error_message = "polis_id must be a lowercase DNS-safe label."
  }
}

variable "runtime_id" {
  description = "Runtime identifier used in archive object prefixes."
  type        = string
  default     = "wuji"

  validation {
    condition     = can(regex("^[a-z0-9][a-z0-9-]{0,61}[a-z0-9]$", var.runtime_id))
    error_message = "runtime_id must be a lowercase DNS-safe label."
  }
}

variable "bucket_name" {
  description = "Globally unique private S3 bucket for redacted Runtime logs."
  type        = string
  default     = null

  validation {
    condition = var.bucket_name == null || can(regex(
      "^[a-z0-9][a-z0-9.-]{1,61}[a-z0-9]$",
      var.bucket_name
    ))
    error_message = "bucket_name must be a DNS-compatible S3 bucket name when set."
  }
}
