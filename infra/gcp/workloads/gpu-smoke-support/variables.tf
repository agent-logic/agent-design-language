variable "project_id" {
  description = "Company GCP project that owns the stable #494 support resources."
  type        = string
  default     = "cs-poc-cha8mmii0xk0iaw5vpf8mxf"
}

variable "region" {
  description = "GCP region for subnet lookup."
  type        = string
  default     = "us-central1"
}

variable "support_id" {
  description = "Stable issue-scoped identifier for reusable support resources."
  type        = string
  default     = "adl-494-gpu-smoke"
}

variable "network_name" {
  description = "Existing GCP network name."
  type        = string
  default     = "default"
}

variable "ssh_source_ranges" {
  description = "Source ranges permitted for IAP TCP forwarding only."
  type        = list(string)
  default     = ["35.235.240.0/20"]
}

variable "labels" {
  description = "Additional labels for stable support resources."
  type        = map(string)
  default     = {}
}
