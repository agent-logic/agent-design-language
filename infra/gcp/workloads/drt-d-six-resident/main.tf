data "google_compute_network" "private" {
  name = var.network_name
}

resource "google_compute_router" "issue509" {
  count   = var.create_cloud_nat ? 1 : 0
  name    = "${var.run_id}-router"
  network = data.google_compute_network.private.id
  region  = var.region
}

resource "google_compute_router_nat" "issue509" {
  count                              = var.create_cloud_nat ? 1 : 0
  name                               = "${var.run_id}-nat"
  router                             = google_compute_router.issue509[0].name
  region                             = var.region
  nat_ip_allocate_option             = "AUTO_ONLY"
  source_subnetwork_ip_ranges_to_nat = "ALL_SUBNETWORKS_ALL_IP_RANGES"
}

module "two_node_ollama_runtime" {
  source = "../modules/two-node-ollama-runtime"

  project_id               = var.project_id
  region                   = var.region
  zone                     = var.zone
  run_id                   = var.run_id
  support_id               = var.support_id
  service_account_email    = var.service_account_email
  subnet_name              = var.subnet_name
  runtime_machine_type     = var.runtime_machine_type
  ollama_machine_type      = var.ollama_machine_type
  runtime_boot_image       = var.runtime_boot_image
  ollama_boot_image        = var.ollama_boot_image
  accelerator_type         = var.accelerator_type
  accelerator_count        = var.accelerator_count
  max_budget_usd           = var.max_budget_usd
  ttl_expires_at           = var.ttl_expires_at
  source_revision          = var.source_revision
  assign_external_ip       = var.assign_external_ip
  enable_oslogin           = var.enable_oslogin
  resident_models          = var.resident_models
  artifact_bucket          = var.artifact_bucket
  artifact_manifest_object = var.artifact_manifest_object
  artifact_manifest_sha256 = var.artifact_manifest_sha256
  runtime_startup_script   = file("${path.module}/startup-runtime.sh")
  ollama_startup_script    = file("${path.module}/startup-ollama.sh")
  labels                   = var.labels

  depends_on = [google_compute_router_nat.issue509]
}
