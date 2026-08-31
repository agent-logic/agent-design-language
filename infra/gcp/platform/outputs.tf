output "network_name" {
  description = "Private custom-mode VPC name."
  value       = google_compute_network.private.name
}

output "subnet_name" {
  description = "Private regional subnet name."
  value       = google_compute_subnetwork.private.name
}

output "iap_tcp_forwarding_cidr" {
  description = "IAP TCP forwarding CIDR allowed for operator access."
  value       = var.iap_tcp_forwarding_cidr
}

output "workload_service_account_email" {
  description = "Dedicated workload identity email."
  value       = google_service_account.workload.email
}

output "storage_owner_buckets" {
  description = "Separate owner buckets for state, artifacts, models, continuity evidence, and logs."
  value = {
    state               = google_storage_bucket.state.name
    artifacts           = google_storage_bucket.artifacts.name
    models              = google_storage_bucket.models.name
    continuity_evidence = google_storage_bucket.continuity_evidence.name
    logs                = google_storage_bucket.logs.name
  }
}

output "required_labels" {
  description = "Required labels for disposable workload cleanup and cost attribution."
  value       = local.required_labels
}
