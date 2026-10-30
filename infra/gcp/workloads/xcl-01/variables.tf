variable "project_id" {
  description = "GCP project that hosts the Runtime workload."
  type        = string
}

variable "region" {
  description = "GCP region for the Runtime workload."
  type        = string
}

variable "zone" {
  description = "GCP zone for the Runtime workload."
  type        = string
}

variable "run_id" {
  description = "Issue-scoped run identity carried into labels and readiness paths."
  type        = string
}

variable "ttl_expires_at" {
  description = "Cleanup TTL/deadline carried into labels for disposable Runtime proof resources."
  type        = string
}

variable "network_name" {
  description = "Runtime VPC network name."
  type        = string
}

variable "subnet_cidr" {
  description = "Private Runtime subnet CIDR."
  type        = string
  default     = "10.194.10.0/24"
}

variable "machine_type" {
  description = "GCP Runtime host shape selected explicitly for parity testing."
  type        = string
}

variable "boot_image" {
  description = "GCP boot image for the Runtime host."
  type        = string
}

variable "retained_runtime_disk" {
  description = "Existing retained Runtime persistent disk name containing the Runtime install."
  type        = string
}

variable "retained_runtime_disk_device_name" {
  description = "Stable device name used to mount the retained Runtime persistent disk."
  type        = string
  default     = "runtime-retained"
}

variable "artifact_bucket" {
  description = "Optional GCS bucket containing immutable Runtime bootstrap artifacts."
  type        = string
  default     = null
}

variable "artifact_prefix" {
  description = "Optional GCS object prefix containing immutable Runtime bootstrap artifacts."
  type        = string
  default     = "shepherd/"
}

variable "operator_ssh_public_key" {
  description = "Operator break-glass SSH public key. Do not pass private key material."
  type        = string
  default     = null
  sensitive   = true
}

variable "labels" {
  description = "Additional resource labels."
  type        = map(string)
  default     = {}
}
