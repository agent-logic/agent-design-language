output "organization_id" {
  description = "Accepted organization id."
  value       = var.organization_id
}

output "foundation_folder_id" {
  description = "Accepted foundation folder id."
  value       = var.foundation_folder_id
}

output "host_project_id" {
  description = "Accepted host project id."
  value       = var.host_project_id
}

output "host_project_number" {
  description = "Resolved host project number."
  value       = data.google_project.host.number
}

output "billing_account_name" {
  description = "Accepted billing account resource name."
  value       = local.billing_account_name
}

output "corporate_owner_group" {
  description = "Corporate owner group used by the baseline."
  value       = var.corporate_owner_group
}

output "required_labels" {
  description = "Required labels for #492-managed resources."
  value       = local.required_labels
}
