variable "project_id" {
  description = "Company GCP project that owns the disposable GPU smoke workload."
  type        = string
}

variable "region" {
  description = "GCP region for the smoke workload."
  type        = string
  default     = "us-west1"
}

variable "zone" {
  description = "GCP zone for the smoke workload."
  type        = string
  default     = "us-west1-a"
}

variable "run_id" {
  description = "Unique issue-scoped run identifier used in names, labels, and cleanup selectors."
  type        = string
  default     = "adl-494-gpu-smoke"
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

variable "network_name" {
  description = "Existing GCP network name for the disposable smoke workload."
  type        = string
  default     = "default"
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

variable "ssh_source_ranges" {
  description = "Source ranges permitted for IAP TCP forwarding only."
  type        = list(string)
  default     = ["35.235.240.0/20"]
}

variable "model_name" {
  description = "Model label retained in proof evidence; the script may skip model inference if the image lacks a local model runtime."
  type        = string
  default     = "gpu-smoke-local"
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
