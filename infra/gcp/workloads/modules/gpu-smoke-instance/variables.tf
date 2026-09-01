variable "project_id" {
  description = "Company GCP project that owns the disposable GPU smoke instance."
  type        = string
}

variable "region" {
  description = "GCP region for subnet lookup."
  type        = string
}

variable "zone" {
  description = "GCP zone for the smoke instance."
  type        = string
}

variable "run_id" {
  description = "Unique issue-scoped run identifier used for the disposable VM name, labels, and cleanup selectors."
  type        = string
}

variable "support_id" {
  description = "Stable support identifier used as the instance network tag."
  type        = string
}

variable "service_account_email" {
  description = "Stable support service account email used by the disposable instance."
  type        = string
}

variable "subnet_name" {
  description = "Existing GCP subnet name for the disposable smoke workload."
  type        = string
}

variable "machine_type" {
  description = "Smallest selected L4 smoke machine type."
  type        = string
}

variable "accelerator_type" {
  description = "GCP accelerator type for the readiness smoke."
  type        = string
}

variable "accelerator_count" {
  description = "Number of L4 GPUs for the smoke."
  type        = number
}

variable "boot_image" {
  description = "Boot image with NVIDIA driver/CUDA userspace suitable for GPU smoke."
  type        = string
}

variable "model_name" {
  description = "Model label retained in proof evidence; the script may skip model inference if the image lacks a local model runtime."
  type        = string
}

variable "max_budget_usd" {
  description = "Hard operator-approved maximum spend for the disposable proof run."
  type        = number

  validation {
    condition     = var.max_budget_usd <= 20
    error_message = "Issue #494 is capped at USD 20."
  }
}

variable "ttl_expires_at" {
  description = "Cleanup deadline retained in labels/metadata for disposable resources."
  type        = string
}

variable "labels" {
  description = "Additional labels for disposable proof resources."
  type        = map(string)
  default     = {}
}
