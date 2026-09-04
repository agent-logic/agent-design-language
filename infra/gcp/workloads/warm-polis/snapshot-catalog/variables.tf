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
variable "runtime_staging_disk" { type = string }
variable "ollama_staging_disk" { type = string }
variable "runtime_manifest_sha256" { type = string }
variable "ollama_manifest_sha256" { type = string }
variable "verification_boot_image" {
  type = string
  validation {
    condition     = strcontains(var.verification_boot_image, "/global/images/") && !strcontains(var.verification_boot_image, "/family/")
    error_message = "verification_boot_image must be an immutable image, not a family alias."
  }
}
variable "service_account_email" { type = string }
variable "subnet_name" { type = string }
variable "enable_verifier" {
  description = "Enable only for restored-content verification; apply false after PASS so retained state contains snapshots only."
  type        = bool
  default     = true
}
