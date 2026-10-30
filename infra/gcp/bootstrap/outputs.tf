output "project_id" {
  description = "GCP project targeted by this bootstrap."
  value       = var.project_id
}

output "state_bucket_name" {
  description = "Private versioned GCS bucket for Terraform remote state."
  value       = google_storage_bucket.terraform_state.name
}

output "state_bucket_url" {
  description = "GCS URL for the Terraform state bucket."
  value       = google_storage_bucket.terraform_state.url
}

output "bootstrap_service_account" {
  description = "Service account granted state-bucket administration."
  value       = var.bootstrap_service_account
}
