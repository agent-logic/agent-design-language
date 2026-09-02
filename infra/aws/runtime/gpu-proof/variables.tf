variable "aws_account_id" {
  type = string
  validation {
    condition     = can(regex("^[0-9]{12}$", var.aws_account_id))
    error_message = "aws_account_id must be exactly 12 decimal digits."
  }
}

variable "aws_profile" {
  description = "Approved Agent Logic business-account profile. This qualification must never fall back to the default profile."
  type        = string
  default     = "agent-logic-admin"
  validation {
    condition     = var.aws_profile == "agent-logic-admin"
    error_message = "aws_profile must be agent-logic-admin."
  }
}

variable "aws_region" {
  type    = string
  default = "us-west-2"
}

variable "issue_number" {
  description = "Exact tracked issue owning this qualification graph."
  type        = number
  default     = 345
  validation {
    condition     = contains([345, 607], var.issue_number)
    error_message = "issue_number must select the retained #345 path or the reviewed #607 warm path."
  }
}

variable "run_id" {
  type = string
  validation {
    condition     = can(regex("^adl-issue[0-9]+-[A-Za-z0-9._-]+$", var.run_id)) && startswith(var.run_id, "adl-issue${var.issue_number}-") && length(var.run_id) <= 48
    error_message = "run_id must begin with the selected issue number, use safe characters, and be at most 48 characters."
  }
}

variable "owner_token" {
  type = string
  validation {
    condition     = can(regex("^[0-9a-f]{32}$", var.owner_token))
    error_message = "owner_token must be exactly 32 lowercase hexadecimal characters."
  }
}

variable "runtime_ami_id" {
  type = string
  validation {
    condition     = can(regex("^ami-[0-9a-f]+$", var.runtime_ami_id))
    error_message = "runtime_ami_id must be an EC2 AMI ID."
  }
}

variable "gpu_ami_id" {
  type = string
  validation {
    condition     = can(regex("^ami-[0-9a-f]+$", var.gpu_ami_id))
    error_message = "gpu_ami_id must be an EC2 AMI ID."
  }
}

variable "vpc_id" {
  type = string
  validation {
    condition     = can(regex("^vpc-[0-9a-f]+$", var.vpc_id))
    error_message = "vpc_id must be an EC2 VPC ID."
  }
}

variable "subnet_id" {
  description = "Pre-resolved subnet for both nodes; Terraform performs no discovery."
  type        = string
  validation {
    condition     = can(regex("^subnet-[0-9a-f]+$", var.subnet_id))
    error_message = "subnet_id must be an EC2 subnet ID."
  }
}

variable "runtime_instance_type" {
  description = "Regular On-Demand node for Guardian, Runtime, six agents, UTS, ACC, and Freedom Gate."
  type        = string
  default     = "r7i.2xlarge"
}

variable "gpu_instance_type" {
  description = "On-Demand GPU node for Ollama and at least two resident models."
  type        = string
  default     = "g6.4xlarge"
}

variable "detailed_monitoring" {
  type    = bool
  default = false
}

variable "ssh_ingress_cidr" {
  description = "Required operator public IPv4 address as an exact /32 CIDR."
  type        = string
  validation {
    condition = (
      can(regex("^([0-9]{1,3}\\.){3}[0-9]{1,3}/32$", var.ssh_ingress_cidr)) &&
      can(cidrhost(var.ssh_ingress_cidr, 0))
    )
    error_message = "ssh_ingress_cidr must be one valid IPv4 /32 CIDR."
  }
}

variable "ssh_public_key" {
  description = "Required public key imported into one Terraform-managed EC2 key pair shared by both nodes; never provide a private key."
  type        = string
  validation {
    condition = can(regex(
      "^(ssh-ed25519|ssh-rsa|ecdsa-sha2-nistp(256|384|521)) [A-Za-z0-9+/]+={0,3}( .*)?$",
      trimspace(var.ssh_public_key)
    ))
    error_message = "ssh_public_key must be a non-empty supported OpenSSH public key."
  }
}

variable "runtime_root_volume_size_gib" {
  type    = number
  default = 80
  validation {
    condition     = var.runtime_root_volume_size_gib >= 40 && floor(var.runtime_root_volume_size_gib) == var.runtime_root_volume_size_gib
    error_message = "runtime_root_volume_size_gib must be an integer of at least 40 GiB."
  }
}

variable "runtime_root_volume_iops" {
  type    = number
  default = 3000
}

variable "runtime_root_volume_throughput_mibps" {
  type    = number
  default = 125
}

variable "gpu_root_volume_size_gib" {
  type    = number
  default = 200
  validation {
    condition     = var.gpu_root_volume_size_gib >= 200 && floor(var.gpu_root_volume_size_gib) == var.gpu_root_volume_size_gib
    error_message = "gpu_root_volume_size_gib must be an integer of at least 200 GiB."
  }
}

variable "gpu_root_volume_iops" {
  type    = number
  default = 3000
}

variable "gpu_root_volume_throughput_mibps" {
  type    = number
  default = 125
}

variable "authorized_max_hourly_usd" {
  description = "Authorized combined hourly ceiling retained as tags; Terraform does not query pricing."
  type        = number
  validation {
    condition     = var.authorized_max_hourly_usd > 0
    error_message = "authorized_max_hourly_usd must be positive."
  }
}

variable "authorized_max_total_usd" {
  description = "Authorized combined total ceiling retained as tags; external preflight enforces cost."
  type        = number
  validation {
    condition     = var.authorized_max_total_usd > 0 && var.authorized_max_total_usd <= 20
    error_message = "authorized_max_total_usd must be positive and no greater than 20 USD."
  }
}

variable "artifact_bucket" {
  type = string
}

variable "artifact_prefix" {
  description = "Exact receipt prefix the nodes may write; read access is separately restricted to artifact_read_keys."
  type        = string
  default     = "shepherd/"
  validation {
    condition     = can(regex("^[A-Za-z0-9][A-Za-z0-9._/-]*/$", var.artifact_prefix)) && !startswith(var.artifact_prefix, "/")
    error_message = "artifact_prefix must be a non-empty relative S3 prefix ending in /."
  }
}

variable "artifact_read_keys" {
  description = "Exact versioned object keys the guests may read; controller locks and authorization markers are excluded."
  type        = list(string)
  validation {
    condition = (
      length(var.artifact_read_keys) >= 5 &&
      length(var.artifact_read_keys) == length(distinct(var.artifact_read_keys)) &&
      alltrue([for key in var.artifact_read_keys : can(regex("^[A-Za-z0-9][A-Za-z0-9._/-]+$", key))])
    )
    error_message = "artifact_read_keys must contain at least five unique safe relative S3 object keys."
  }
}

variable "gpu_user_data" {
  description = "Automatic cloud-init that starts Ollama and at least two resident models."
  type        = string
  sensitive   = true
  validation {
    condition     = length(trimspace(var.gpu_user_data)) > 0
    error_message = "gpu_user_data must not be empty."
  }
}

variable "runtime_user_data" {
  description = "Automatic Guardian/Runtime/six-agent cloud-init; __GPU_PRIVATE_IP__ is replaced with the private Ollama host."
  type        = string
  sensitive   = true
  validation {
    condition     = strcontains(var.runtime_user_data, "__GPU_PRIVATE_IP__")
    error_message = "runtime_user_data must contain __GPU_PRIVATE_IP__."
  }
}

variable "warm_volume_availability_zone" {
  description = "Exact AZ of both issue #607 retained volumes. Null selects the original cold #345 path."
  type        = string
  default     = null
}

variable "runtime_warm_volume_id" {
  description = "Prepared Runtime content volume owned by the separate warm-storage state."
  type        = string
  default     = null
  validation {
    condition     = var.runtime_warm_volume_id == null || can(regex("^vol-[0-9a-f]+$", var.runtime_warm_volume_id))
    error_message = "runtime_warm_volume_id must be null or an EBS volume ID."
  }
}

variable "gpu_warm_volume_id" {
  description = "Prepared Ollama/model content volume owned by the separate warm-storage state."
  type        = string
  default     = null
  validation {
    condition     = var.gpu_warm_volume_id == null || can(regex("^vol-[0-9a-f]+$", var.gpu_warm_volume_id))
    error_message = "gpu_warm_volume_id must be null or an EBS volume ID."
  }
}

variable "runtime_warm_device_name" {
  type    = string
  default = "/dev/sdf"
}

variable "gpu_warm_device_name" {
  type    = string
  default = "/dev/sdf"
}

variable "runtime_warm_seal_sha256" {
  type    = string
  default = null
  validation {
    condition     = var.runtime_warm_seal_sha256 == null || can(regex("^[0-9a-f]{64}$", var.runtime_warm_seal_sha256))
    error_message = "runtime_warm_seal_sha256 must be null or lowercase SHA-256."
  }
}

variable "gpu_warm_seal_sha256" {
  type    = string
  default = null
  validation {
    condition     = var.gpu_warm_seal_sha256 == null || can(regex("^[0-9a-f]{64}$", var.gpu_warm_seal_sha256))
    error_message = "gpu_warm_seal_sha256 must be null or lowercase SHA-256."
  }
}

variable "warm_artifact_generation" {
  description = "Immutable prepared generation expected in both sealed-volume manifests."
  type        = string
  default     = null
}

variable "warm_source_commit" {
  description = "Exact source revision embedded in the Runtime sealed volume."
  type        = string
  default     = null
  validation {
    condition     = var.warm_source_commit == null || can(regex("^[0-9a-f]{40}$", var.warm_source_commit))
    error_message = "warm_source_commit must be null or an exact lowercase Git commit."
  }
}

variable "warm_kms_key_arn" {
  description = "Exact KMS key required on both prepared warm volumes."
  type        = string
  default     = null
  validation {
    condition     = var.warm_kms_key_arn == null || can(regex("^arn:aws:kms:[a-z0-9-]+:[0-9]{12}:key/[0-9a-f-]+$", var.warm_kms_key_arn))
    error_message = "warm_kms_key_arn must be null or an exact KMS key ARN."
  }
}
