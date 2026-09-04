provider "google" {
  project = var.project_id
  region  = var.region
  zone    = var.zone
}

locals {
  labels = merge(var.labels, {
    adl_issue               = var.issue_id
    adl_artifact_generation = lower(replace(var.artifact_generation, "_", "-"))
    adl_retention           = "disposable-launch"
  })
}

data "google_compute_subnetwork" "private" {
  name   = var.subnet_name
  region = var.region
}

resource "google_compute_disk" "runtime" {
  name     = "${var.run_id}-runtime-data"
  zone     = var.zone
  type     = "pd-balanced"
  snapshot = var.runtime_snapshot
  labels   = local.labels
}

resource "google_compute_disk" "ollama" {
  name     = "${var.run_id}-ollama-data"
  zone     = var.zone
  type     = "pd-balanced"
  snapshot = var.ollama_snapshot
  labels   = local.labels
}

resource "google_compute_firewall" "runtime_to_ollama" {
  name      = "${var.run_id}-runtime-to-ollama"
  network   = data.google_compute_subnetwork.private.network
  direction = "INGRESS"

  allow {
    protocol = "tcp"
    ports    = ["11434"]
  }

  source_tags = ["${var.support_id}-runtime"]
  target_tags = ["${var.support_id}-ollama"]
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
  paid_deadline_epoch      = var.paid_deadline_epoch
  ttl_expires_at           = "none"
  source_revision          = var.artifact_generation
  issue_id                 = var.issue_id
  lane                     = "warm-polis"
  retention                = "disposable-launch"
  assign_external_ip       = var.assign_external_ip
  resident_models          = var.resident_models
  artifact_bucket          = ""
  artifact_manifest_object = ""
  artifact_manifest_sha256 = var.ollama_content_sha256
  runtime_startup_script   = file("${path.module}/startup-runtime.sh")
  ollama_startup_script    = file("${path.module}/startup-ollama.sh")
  attach_data_disks        = true
  runtime_data_disk        = google_compute_disk.runtime.self_link
  ollama_data_disk         = google_compute_disk.ollama.self_link
  runtime_data_device_name = "adl-runtime-data"
  ollama_data_device_name  = "adl-ollama-data"
  artifact_generation      = var.artifact_generation
  runtime_content_sha256   = var.runtime_content_sha256
  ollama_content_sha256    = var.ollama_content_sha256
  labels                   = local.labels

  depends_on = [google_compute_firewall.runtime_to_ollama]
}
