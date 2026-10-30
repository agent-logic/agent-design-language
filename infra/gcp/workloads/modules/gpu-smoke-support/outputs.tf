output "service_account_email" {
  description = "Stable service account email used by disposable GPU smoke instances."
  value       = google_service_account.gpu_smoke.email
}

output "firewall_name" {
  description = "Stable IAP SSH firewall rule for disposable GPU smoke instances."
  value       = google_compute_firewall.iap_ssh.name
}

output "network_tag" {
  description = "Network tag the disposable instance must carry for IAP SSH access."
  value       = var.support_id
}
