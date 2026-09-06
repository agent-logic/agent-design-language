variable "project_id" {
  description = "Company GCP project that owns the disposable two-node qualification."
  type        = string
}

variable "region" {
  description = "GCP region for subnet lookup."
  type        = string
}

variable "zone" {
  description = "GCP zone for both disposable qualification nodes."
  type        = string
}

variable "run_id" {
  description = "Unique issue-scoped run identifier for VM names, labels, and cleanup selectors."
  type        = string
}

variable "support_id" {
  description = "Stable support identifier used as the instance network tag."
  type        = string
}

variable "service_account_email" {
  description = "Stable support service account email used by the disposable instances."
  type        = string
}

variable "subnet_name" {
  description = "Existing GCP subnet name for the disposable workload."
  type        = string
}

variable "runtime_machine_type" {
  description = "Runtime/CSM node machine type."
  type        = string
}

variable "ollama_machine_type" {
  description = "Ollama/GPU node machine type."
  type        = string
}

variable "runtime_boot_image" {
  description = "Runtime node boot image."
  type        = string
}

variable "ollama_boot_image" {
  description = "Ollama/GPU node boot image with NVIDIA driver/CUDA userspace."
  type        = string
}

variable "accelerator_type" {
  description = "GCP accelerator type for the Ollama node."
  type        = string
}

variable "accelerator_count" {
  description = "GPU count for the Ollama node."
  type        = number
}

variable "max_budget_usd" {
  description = "Hard operator-approved maximum spend for the disposable proof run."
  type        = number

  validation {
    condition     = var.max_budget_usd <= 20
    error_message = "Issue #509 is capped at USD 20."
  }
}

variable "ttl_expires_at" {
  description = "Cleanup deadline retained in labels/metadata for disposable resources."
  type        = string
}

variable "paid_deadline_epoch" {
  description = "Absolute Unix deadline enforced by every disposable paid guest. Zero disables the issue-specific guard for reusable-module callers."
  type        = number
  default     = 0
}

variable "source_revision" {
  description = "Exact ADL source revision under qualification."
  type        = string
}

variable "assign_external_ip" {
  description = "Attach ephemeral external IPv4 addresses for outbound bootstrap when Cloud NAT is not available. Firewall rules still govern inbound access."
  type        = bool
}

variable "enable_oslogin" {
  description = "Enable GCP OS Login on disposable qualification nodes. The company org policy requires this for SSH access."
  type        = bool
  default     = true
}

variable "resident_models" {
  description = "Exact resident Ollama model set used by the six-agent UTS proof."
  type        = list(string)
}

variable "artifact_bucket" {
  description = "GCS bucket containing the immutable Ollama runtime/model bundle manifest."
  type        = string
}

variable "artifact_manifest_object" {
  description = "GCS object name for the immutable Ollama runtime/model bundle manifest."
  type        = string
}

variable "artifact_manifest_sha256" {
  description = "Expected SHA-256 for the Ollama runtime/model bundle manifest."
  type        = string
}

variable "runtime_startup_script" {
  description = "Startup script installed on the Runtime/CSM node."
  type        = string
}

variable "ollama_startup_script" {
  description = "Startup script installed on the Ollama/GPU node."
  type        = string
}

variable "runtime_data_disk" {
  description = "Optional self-link of a prepared Runtime data disk to attach."
  type        = string
  default     = null
  nullable    = true
}

variable "attach_data_disks" {
  description = "Create both optional prepared-data attachments. Disk self-links must be supplied when true."
  type        = bool
  default     = false
}

variable "ollama_data_disk" {
  description = "Optional self-link of a prepared Ollama/model data disk to attach."
  type        = string
  default     = null
  nullable    = true
}

variable "runtime_data_device_name" {
  description = "Stable guest device name for the optional Runtime data disk."
  type        = string
  default     = "adl-runtime-data"
}

variable "ollama_data_device_name" {
  description = "Stable guest device name for the optional Ollama/model data disk."
  type        = string
  default     = "adl-ollama-data"
}

variable "artifact_generation" {
  description = "Expected generation marker on optional prepared data disks."
  type        = string
  default     = ""
}

variable "runtime_content_sha256" {
  description = "Expected Runtime generation-manifest SHA-256."
  type        = string
  default     = ""
}

variable "ollama_content_sha256" {
  description = "Expected Ollama/model generation-manifest SHA-256."
  type        = string
  default     = ""
}

variable "labels" {
  description = "Additional labels for disposable proof resources."
  type        = map(string)
  default     = {}
}

variable "issue_id" {
  description = "Issue identity recorded in instance labels and metadata."
  type        = string
  default     = "509"
}

variable "lane" {
  description = "Workload lane recorded in instance labels and metadata."
  type        = string
  default     = "drt-d"
}

variable "retention" {
  description = "Resource retention posture recorded in the ttl label."
  type        = string
  default     = "disposable"
}
