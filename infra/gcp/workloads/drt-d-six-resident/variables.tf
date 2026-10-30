variable "project_id" {
  description = "Company GCP project that owns the disposable two-node DRT-D qualification."
  type        = string
  default     = "cs-poc-cha8mmii0xk0iaw5vpf8mxf"
}

variable "region" {
  description = "GCP region for the qualification workload."
  type        = string
  default     = "us-west1"
}

variable "zone" {
  description = "GCP zone for the qualification workload."
  type        = string
  default     = "us-west1-a"
}

variable "run_id" {
  description = "Unique issue-scoped run identifier used for disposable VM names, labels, and cleanup selectors."
  type        = string
  default     = "adl-509-drt-d"
}

variable "support_id" {
  description = "Stable support identifier from #494/#495 used as the IAP SSH network tag."
  type        = string
  default     = "adl-494-gpu-smoke"
}

variable "service_account_email" {
  description = "Stable support service account email used by disposable instances."
  type        = string
}

variable "subnet_name" {
  description = "Existing GCP subnet name for the disposable workload."
  type        = string
  default     = "default"
}

variable "network_name" {
  description = "Existing GCP network name for optional Cloud NAT."
  type        = string
  default     = "default"
}

variable "runtime_machine_type" {
  description = "Runtime/CSM node machine type."
  type        = string
  default     = "e2-standard-4"
}

variable "ollama_machine_type" {
  description = "Ollama/GPU node machine type."
  type        = string
  default     = "g2-standard-4"
}

variable "runtime_boot_image" {
  description = "Runtime node boot image with launch tooling already present; normal startup must not install packages."
  type        = string
  default     = "projects/deeplearning-platform-release/global/images/family/common-cu129-ubuntu-2204-nvidia-580"
}

variable "ollama_boot_image" {
  description = "Ollama/GPU node boot image with NVIDIA driver/CUDA userspace."
  type        = string
  default     = "projects/deeplearning-platform-release/global/images/family/common-cu129-ubuntu-2204-nvidia-580"
}

variable "accelerator_type" {
  description = "GCP accelerator type for the Ollama node."
  type        = string
  default     = "nvidia-l4"
}

variable "accelerator_count" {
  description = "GPU count for the Ollama node."
  type        = number
  default     = 1
}

variable "max_budget_usd" {
  description = "Hard operator-approved maximum spend for the disposable proof run."
  type        = number
  default     = 20
}

variable "ttl_expires_at" {
  description = "Cleanup deadline retained in labels/metadata for disposable resources."
  type        = string
}

variable "source_revision" {
  description = "Exact ADL source revision under qualification."
  type        = string
}

variable "assign_external_ip" {
  description = "Attach ephemeral external IPv4 addresses for outbound bootstrap when Cloud NAT is not available. Firewall rules still govern inbound access."
  type        = bool
  default     = false
}

variable "enable_oslogin" {
  description = "Enable GCP OS Login on disposable qualification nodes. The company org policy requires this for SSH access."
  type        = bool
  default     = true
}

variable "create_cloud_nat" {
  description = "Create disposable Cloud Router/NAT for private VM outbound bootstrap when external VM IPs are blocked."
  type        = bool
  default     = true
}

variable "resident_models" {
  description = "Exact resident Ollama model set used by the six-agent UTS proof."
  type        = list(string)
  default     = ["llama3.1:8b", "qwen3:8b", "phi4-mini:latest"]
}

variable "artifact_bucket" {
  description = "GCS bucket containing immutable Ollama runtime/model bundle artifacts."
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

variable "labels" {
  description = "Additional labels for disposable proof resources."
  type        = map(string)
  default     = {}
}
