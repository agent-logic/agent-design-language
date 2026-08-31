variable "project_id" {
  description = "Company GCP project that owns the stable #494 support resources."
  type        = string
}

variable "region" {
  description = "GCP region for subnet lookup."
  type        = string
}

variable "support_id" {
  description = "Stable issue-scoped identifier for reusable support resources."
  type        = string
}

variable "network_name" {
  description = "Existing GCP network name."
  type        = string
}

variable "ssh_source_ranges" {
  description = "Source ranges permitted for IAP TCP forwarding only."
  type        = list(string)
}

variable "labels" {
  description = "Labels applied to support resources that support labels."
  type        = map(string)
  default     = {}
}
