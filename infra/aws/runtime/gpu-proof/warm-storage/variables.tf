variable "aws_account_id" {
  type = string
  validation {
    condition     = can(regex("^[0-9]{12}$", var.aws_account_id))
    error_message = "aws_account_id must be exactly 12 decimal digits."
  }
}

variable "aws_profile" {
  type    = string
  default = "agent-logic-admin"
  validation {
    condition     = var.aws_profile == "agent-logic-admin"
    error_message = "aws_profile must be agent-logic-admin."
  }
}

variable "aws_region" {
  type    = string
  default = "us-west-2"
}

variable "availability_zone" {
  description = "Exact AZ shared by both retained volumes and every compute attachment."
  type        = string
  validation {
    condition     = startswith(var.availability_zone, var.aws_region)
    error_message = "availability_zone must belong to aws_region."
  }
}

variable "storage_id" {
  description = "Stable retained-storage identity; unlike a compute run ID it survives launch cleanup."
  type        = string
  validation {
    condition     = can(regex("^adl-issue607-[A-Za-z0-9._-]+$", var.storage_id)) && length(var.storage_id) <= 48
    error_message = "storage_id must begin with adl-issue607- and be at most 48 safe characters."
  }
}

variable "owner_token" {
  type      = string
  sensitive = true
  validation {
    condition     = can(regex("^[0-9a-f]{32}$", var.owner_token))
    error_message = "owner_token must be exactly 32 lowercase hexadecimal characters."
  }
}

variable "kms_key_arn" {
  description = "Existing business-account KMS key; this root never owns or destroys it."
  type        = string
  validation {
    condition     = can(regex("^arn:aws:kms:[a-z0-9-]+:[0-9]{12}:key/[0-9a-f-]+$", var.kms_key_arn))
    error_message = "kms_key_arn must be an exact KMS key ARN."
  }
}

variable "artifact_generation" {
  type = string
  validation {
    condition     = can(regex("^[A-Za-z0-9._:-]+$", var.artifact_generation))
    error_message = "artifact_generation must be a bounded immutable identifier."
  }
}

variable "retention_until" {
  description = "UTC delete-or-extend decision deadline for this warm generation."
  type        = string
  validation {
    condition     = can(regex("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$", var.retention_until))
    error_message = "retention_until must be an RFC3339 UTC second."
  }
}

variable "runtime_seal_sha256" {
  type = string
  validation {
    condition     = can(regex("^[0-9a-f]{64}$", var.runtime_seal_sha256))
    error_message = "runtime_seal_sha256 must be lowercase SHA-256."
  }
}

variable "gpu_seal_sha256" {
  type = string
  validation {
    condition     = can(regex("^[0-9a-f]{64}$", var.gpu_seal_sha256))
    error_message = "gpu_seal_sha256 must be lowercase SHA-256."
  }
}

variable "runtime_size_gib" {
  type    = number
  default = 200
}

variable "runtime_iops" {
  type    = number
  default = 3000
}

variable "runtime_throughput_mibps" {
  type    = number
  default = 250
}

variable "gpu_size_gib" {
  type    = number
  default = 200
}

variable "gpu_iops" {
  type    = number
  default = 3000
}

variable "gpu_throughput_mibps" {
  type    = number
  default = 500
}
