provider "google" {
  project                     = var.project_id
  region                      = var.region
  impersonate_service_account = var.bootstrap_service_account
}
