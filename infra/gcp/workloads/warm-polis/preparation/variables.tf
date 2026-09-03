variable "project_id" { type = string }
variable "region" { type = string }
variable "zone" { type = string }
variable "generation" {
  type = string
  validation {
    condition     = can(regex("^[a-z][a-z0-9-]{0,30}$", var.generation))
    error_message = "generation must be a short lowercase immutable generation slug."
  }
}
variable "service_account_email" { type = string }
variable "subnet_name" { type = string }
variable "preparation_boot_image" {
  type = string
  validation {
    condition     = strcontains(var.preparation_boot_image, "/global/images/") && !strcontains(var.preparation_boot_image, "/family/")
    error_message = "preparation_boot_image must be an immutable image, not a family alias."
  }
}
variable "runtime_bundle_uri" {
  type = string
  validation {
    condition     = can(regex("^gs://[^/]+/.+#[0-9]+$", var.runtime_bundle_uri))
    error_message = "runtime_bundle_uri must include an exact numeric GCS object generation suffix."
  }
}
variable "runtime_bundle_sha256" {
  type = string
  validation {
    condition     = can(regex("^[0-9a-f]{64}$", var.runtime_bundle_sha256))
    error_message = "runtime_bundle_sha256 must be a lowercase SHA-256."
  }
}
variable "ollama_bundle_uri" {
  type = string
  validation {
    condition     = can(regex("^gs://[^/]+/.+#[0-9]+$", var.ollama_bundle_uri))
    error_message = "ollama_bundle_uri must include an exact numeric GCS object generation suffix."
  }
}
variable "ollama_bundle_sha256" {
  type = string
  validation {
    condition     = can(regex("^[0-9a-f]{64}$", var.ollama_bundle_sha256))
    error_message = "ollama_bundle_sha256 must be a lowercase SHA-256."
  }
}
variable "runtime_disk_size_gib" {
  type    = number
  default = 80
}
variable "ollama_disk_size_gib" {
  type    = number
  default = 300
}
variable "attach_preparation_vms" {
  description = "True only while hydrating and sealing. Set false and apply before snapshot creation."
  type        = bool
  default     = true
}
