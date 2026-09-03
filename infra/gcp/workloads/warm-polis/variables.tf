variable "project_id" {
  description = "Company GCP project that owns the warm Polis launch."
  type        = string
}

variable "region" {
  description = "GCP region for subnet lookup."
  type        = string
}

variable "zone" {
  description = "GCP zone for the two launch VMs and restored disks."
  type        = string
}

variable "run_id" {
  description = "Unique launch identifier."
  type        = string
}

variable "issue_id" {
  description = "Tracked issue identity recorded in labels and metadata."
  type        = string
  default     = "663"
}

variable "max_budget_usd" {
  description = "Operator-approved maximum spend for this live launch."
  type        = number

  validation {
    condition     = var.max_budget_usd > 0 && var.max_budget_usd <= 20
    error_message = "Warm Polis live execution requires a positive budget capped at USD 20."
  }
}

variable "support_id" {
  description = "Existing support network tag used for OS Login/SSH access."
  type        = string
}

variable "service_account_email" {
  description = "Existing workload service account email."
  type        = string
}

variable "subnet_name" {
  description = "Existing private subnet name."
  type        = string
}

variable "runtime_machine_type" {
  description = "Runtime and Guardian VM machine type."
  type        = string
  default     = "e2-standard-4"
}

variable "ollama_machine_type" {
  description = "Ollama GPU VM machine type."
  type        = string
  default     = "g2-standard-4"
}

variable "runtime_boot_image" {
  description = "Exact immutable Runtime image self-link or ID; family aliases are forbidden."
  type        = string

  validation {
    condition     = strcontains(var.runtime_boot_image, "/global/images/") && !strcontains(var.runtime_boot_image, "/family/")
    error_message = "runtime_boot_image must identify one immutable image, never an image family."
  }
}

variable "ollama_boot_image" {
  description = "Exact immutable GPU image self-link or ID with NVIDIA and Ollama launch dependencies installed."
  type        = string

  validation {
    condition     = strcontains(var.ollama_boot_image, "/global/images/") && !strcontains(var.ollama_boot_image, "/family/")
    error_message = "ollama_boot_image must identify one immutable image, never an image family."
  }
}

variable "runtime_snapshot" {
  description = "Exact self-link of the sealed Runtime content snapshot."
  type        = string

  validation {
    condition     = can(regex("^(https://www.googleapis.com/compute/(v1|beta)/)?projects/[^/]+/global/snapshots/[^/]+$", var.runtime_snapshot))
    error_message = "runtime_snapshot must be an exact project snapshot self-link or resource ID."
  }
}

variable "ollama_snapshot" {
  description = "Exact self-link of the sealed Ollama/model content snapshot."
  type        = string

  validation {
    condition     = can(regex("^(https://www.googleapis.com/compute/(v1|beta)/)?projects/[^/]+/global/snapshots/[^/]+$", var.ollama_snapshot))
    error_message = "ollama_snapshot must be an exact project snapshot self-link or resource ID."
  }
}

variable "artifact_generation" {
  description = "Exact generation marker expected on both restored disks."
  type        = string

  validation {
    condition     = can(regex("^[a-z][a-z0-9-]{0,30}$", var.artifact_generation))
    error_message = "artifact_generation must be a short lowercase immutable generation slug."
  }
}

variable "runtime_content_sha256" {
  description = "SHA-256 of the Runtime disk generation manifest."
  type        = string

  validation {
    condition     = can(regex("^[0-9a-f]{64}$", var.runtime_content_sha256))
    error_message = "runtime_content_sha256 must be a lowercase SHA-256."
  }
}

variable "ollama_content_sha256" {
  description = "SHA-256 of the Ollama/model disk generation manifest."
  type        = string

  validation {
    condition     = can(regex("^[0-9a-f]{64}$", var.ollama_content_sha256))
    error_message = "ollama_content_sha256 must be a lowercase SHA-256."
  }
}

variable "resident_models" {
  description = "Models that must be resident before the GPU node becomes ready."
  type        = list(string)
  default     = ["llama3.1:8b", "qwen3:8b"]

  validation {
    condition     = length(var.resident_models) >= 2
    error_message = "Warm Polis requires at least two resident models."
  }
}

variable "accelerator_type" {
  description = "GPU accelerator type."
  type        = string
  default     = "nvidia-l4"
}

variable "accelerator_count" {
  description = "GPU count for the Ollama node."
  type        = number
  default     = 1
}

variable "assign_external_ip" {
  description = "Attach ephemeral external addresses. Keep false when private egress already exists."
  type        = bool
  default     = false
}

variable "labels" {
  description = "Additional labels."
  type        = map(string)
  default     = {}
}
