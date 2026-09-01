variable "aws_account_id" {
  type = string
}

variable "aws_profile" {
  type    = string
  default = "agent-logic-admin"
}

variable "aws_region" {
  type    = string
  default = "us-west-2"
}

variable "run_id" {
  type = string
}

variable "owner_token" {
  type      = string
  sensitive = true
}

variable "termination_at" {
  type = string
}

variable "runtime_ami_id" {
  description = "Exact Runtime AMI used for both preparation and warm launch."
  type        = string
}

variable "gpu_ami_id" {
  description = "Exact GPU AMI used for both preparation and warm launch."
  type        = string
}

variable "runtime_ami_metadata_json" {
  type = string
}

variable "gpu_ami_metadata_json" {
  type = string
}

variable "ami_metadata_sha256" {
  type = string
  validation {
    condition     = can(regex("^[0-9a-f]{64}$", var.ami_metadata_sha256))
    error_message = "ami_metadata_sha256 must be lowercase SHA-256."
  }
}

variable "runtime_instance_type" {
  type    = string
  default = "m7i.2xlarge"
}

variable "gpu_instance_type" {
  type    = string
  default = "g6.xlarge"
}

variable "vpc_id" {
  type = string
}

variable "subnet_id" {
  type = string
}

variable "ssh_ingress_cidr" {
  type = string
}

variable "ssh_public_key" {
  type      = string
  sensitive = true
}

variable "artifact_bucket" {
  type = string
}

variable "artifact_read_keys" {
  description = "Exact immutable S3 objects preparation may read."
  type        = list(string)
  validation {
    condition     = length(var.artifact_read_keys) >= 5 && length(var.artifact_read_keys) == length(distinct(var.artifact_read_keys))
    error_message = "artifact_read_keys must contain at least five unique exact objects."
  }
}

variable "receipt_write_prefix" {
  type = string
}

variable "runtime_volume_id" {
  type = string
}

variable "gpu_volume_id" {
  type = string
}

variable "availability_zone" {
  type = string
}

variable "artifact_generation" {
  type = string
}

variable "source_commit" {
  type = string
}

variable "source_archive_key" {
  type = string
}

variable "source_archive_version_id" {
  type = string
}

variable "source_archive_sha256" {
  type = string
}

variable "artifact_manifest_key" {
  type = string
}

variable "artifact_manifest_version_id" {
  type = string
}

variable "artifact_manifest_sha256" {
  type = string
}

variable "kms_key_arn" {
  type = string
}

variable "root_volume_size_gib" {
  type    = number
  default = 80
}
