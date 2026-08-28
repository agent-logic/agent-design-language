variable "project_id" {
  description = "Company GCP host project for the Terraform bootstrap."
  type        = string
  default     = "cs-host-377d41e71a824f92802120"

  validation {
    condition     = var.project_id == "cs-host-377d41e71a824f92802120"
    error_message = "GCP-B is scoped only to cs-host-377d41e71a824f92802120."
  }
}

variable "region" {
  description = "Default region for bootstrap-managed regional resources."
  type        = string
  default     = "us-central1"
}

variable "state_bucket_name" {
  description = "Globally unique private GCS bucket name for ADL Terraform remote state."
  type        = string
  default     = "adl-tf-state-cs-host-377d41e71a824f92802120"
}

variable "bootstrap_service_account" {
  description = "Company-controlled Terraform bootstrap service account."
  type        = string
  default     = "tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"

  validation {
    condition     = var.bootstrap_service_account == "tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com"
    error_message = "GCP-B is scoped only to the approved tf-bootstrap service account."
  }
}

variable "labels" {
  description = "Labels applied to bootstrap resources."
  type        = map(string)
  default = {
    app       = "adl"
    milestone = "v0-92-1"
    issue     = "491"
    lane      = "gcp-b"
  }
}
