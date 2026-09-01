module "gpu_smoke_instance" {
  source = "../modules/gpu-smoke-instance"

  project_id            = var.project_id
  region                = var.region
  zone                  = var.zone
  run_id                = var.run_id
  support_id            = var.support_id
  service_account_email = var.service_account_email
  subnet_name           = var.subnet_name
  machine_type          = var.machine_type
  accelerator_type      = var.accelerator_type
  accelerator_count     = var.accelerator_count
  boot_image            = var.boot_image
  model_name            = var.model_name
  max_budget_usd        = var.max_budget_usd
  ttl_expires_at        = var.ttl_expires_at
  labels                = var.labels
}
