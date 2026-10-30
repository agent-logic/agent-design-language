variable "project_id" {
  description = "Company GCP project that owns the disposable GPU smoke workload."
  type        = string
  default     = "cs-poc-cha8mmii0xk0iaw5vpf8mxf"
}

variable "region" {
  description = "GCP region for the smoke workload."
  type        = string
  default     = "us-central1"
}

variable "zone" {
  description = "GCP zone for the smoke workload."
  type        = string
  default     = "us-central1-a"
}

variable "run_id" {
  description = "Unique issue-scoped run identifier used for the disposable VM name, labels, and cleanup selectors."
  type        = string
  default     = "adl-494-gpu-smoke"
}

variable "support_id" {
  description = "Stable issue-scoped identifier for reusable support resources."
  type        = string
  default     = "adl-494-gpu-smoke"
}

variable "service_account_email" {
  description = "Stable support service account email used by the disposable instance."
  type        = string
}

variable "subnet_name" {
  description = "Existing GCP subnet name for the disposable smoke workload."
  type        = string
  default     = "default"
}

variable "machine_type" {
  description = "Smallest selected L4 smoke machine type."
  type        = string
  default     = "g2-standard-4"
}

variable "accelerator_type" {
  description = "GCP accelerator type for the readiness smoke."
  type        = string
  default     = "nvidia-l4"
}

variable "accelerator_count" {
  description = "Number of L4 GPUs for the smoke."
  type        = number
  default     = 1
}

variable "boot_image" {
  description = "Boot image with NVIDIA driver/CUDA userspace suitable for GPU smoke."
  type        = string
  default     = "projects/deeplearning-platform-release/global/images/family/common-cu129-ubuntu-2204-nvidia-580"
}

variable "model_name" {
  description = "Model label retained in proof evidence; the script may skip model inference if the image lacks a local model runtime."
  type        = string
  default     = "gpu-smoke-local"
}

variable "max_budget_usd" {
  description = "Hard operator-approved maximum spend for the disposable proof run."
  type        = number
  default     = 20

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
