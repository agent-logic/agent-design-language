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

variable "run_id" {
  type = string
  validation {
    condition     = can(regex("^adl-issue345-[A-Za-z0-9._-]+$", var.run_id)) && length(var.run_id) <= 48
    error_message = "run_id must begin with adl-issue345-, use safe characters, and be at most 48 characters."
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
  default     = "g6.xlarge"
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

variable "termination_at" {
  type = string
  validation {
    condition     = can(regex("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$", var.termination_at))
    error_message = "termination_at must be an exact second-resolution UTC timestamp ending in Z."
  }
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
  description = "Only S3 prefix both nodes may read and write."
  type        = string
  default     = "shepherd/"
  validation {
    condition     = can(regex("^[A-Za-z0-9][A-Za-z0-9._/-]*/$", var.artifact_prefix)) && !startswith(var.artifact_prefix, "/")
    error_message = "artifact_prefix must be a non-empty relative S3 prefix ending in /."
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
