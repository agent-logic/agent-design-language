output "service_account_email" {
  description = "Stable service account email used by disposable GPU smoke instances."
  value       = module.gpu_smoke_support.service_account_email
}

output "firewall_name" {
  description = "Stable IAP SSH firewall rule for disposable GPU smoke instances."
  value       = module.gpu_smoke_support.firewall_name
}

output "network_tag" {
  description = "Network tag the disposable instance must carry for IAP SSH access."
  value       = module.gpu_smoke_support.network_tag
}
