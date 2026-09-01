module "gpu_smoke_support" {
  source = "../modules/gpu-smoke-support"

  project_id        = var.project_id
  region            = var.region
  support_id        = var.support_id
  network_name      = var.network_name
  ssh_source_ranges = var.ssh_source_ranges
  labels            = var.labels
}
